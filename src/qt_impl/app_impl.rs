extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex};
use glib::translate::*;

use crate::interface::*;
use crate::ffi::*;
use crate::project::*;
use crate::object::{Object, ObjectContent};
use crate::util;

use super::*;

// functions in main.cpp
extern {
  fn set_widget_to_sink(sink: *const gstreamer_sys::GstElement, video_item: *const usize);
}

pub struct App {
  // Qt
  pub emit: AppEmitter,
  // rust_qt_binding_generator does not allow us to have QObject (Lists) as an item for QList (Layers) so we need to connect Project and these structs dedicated to Qt Model
  layers: Layers,
  objects: TimelineObjects,

  // GStreamer
  pub project: Arc<Mutex<Project>>,
  pub sink: Arc<gst::Element>,
}

impl AppTrait for App {
  fn new(emit: AppEmitter, layers: Layers, objects: TimelineObjects) -> Self {
    let (project, sink) = Self::setup();

    let mut s = Self {
      emit,
      project: Arc::new(Mutex::new(project)),
      layers,
      objects,
      sink: Arc::new(sink)
    };

    s.layers.set_project(&s.project);
    s.objects.set_project(&s.project);

    {
      let proj = &*s.project.lock().unwrap();
      s.layers.set_layers(&proj.layers);
      s.objects.set_objects(&proj.objects);
    }
    
    // This constructor is called from `engine.load()` in main_cpp(). But we are going to obtain the address for videoItem later in main_cpp() (set_video_item_pointer())
    // so we wait until the pointer of video_item is passed
    s.wait_for_pointer();

    s.emit.duration_ms_changed();

    s
  }

  fn emit(&mut self) -> &mut AppEmitter {
    &mut self.emit
  }

  fn layers(&self) -> &Layers { &self.layers }
  fn layers_mut(&mut self) -> &mut Layers { &mut self.layers }
  fn objects(&self) -> &TimelineObjects { &self.objects }
  fn objects_mut(&mut self) -> &mut TimelineObjects { &mut self.objects }

  fn playing(&self) -> bool {
    let project = &*self.project.lock().unwrap();
    println!("playing?: {}", project.playing);
    project.playing
  }

  fn play(&mut self) {
    { // the emitter will call playing() and try to obtain a lock for project, so we have parentheses here
      let project = &mut *self.project.lock().unwrap();
      project.play();
    }
    self.emit.playing_changed();
  }
  
  fn pause(&mut self) {
    {
      let project = &mut *self.project.lock().unwrap();
      project.pause();
    }
    self.emit.playing_changed();
  }

  fn duration_ms(&self) -> u64 {
    let project = &*self.project.lock().unwrap();
    if let Some(dur) = project.ges_pipeline.query_duration::<gst::ClockTime>() {
      let ms = dur.mseconds().unwrap();
      return ms;
    }
    0
  }

  fn position_ms(&self) -> u64 {
    let project = &*self.project.lock().unwrap();
    if let Some(pos) = project.ges_pipeline.query_position::<gst::ClockTime>() {
      let ms = pos.mseconds().unwrap();
      println!("pos {}", ms);
      return ms;
    }
    0
  }

  fn canvas_height(&self) -> u64 {
    return 480
  }

  fn canvas_width(&self) -> u64 {
    return 640
  }

  fn seek_to(&mut self, to: u64) {
    let project = &*self.project.lock().unwrap();
    if project.ges_pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, gst::MSECOND * (to as u64))
      .is_err() {
      println!("Seeking failed");
    }
  }

  fn move_timeline_object(&mut self, obj_name: String, dst_layer_id: u64, dst_time_ms: f32) {
    println!("Moving object to: {}", dst_layer_id);

    {
      let project = &mut *self.project.lock().unwrap();
      project.print_objects();

      project.move_object_to_layer(&obj_name, dst_layer_id as usize);
      let obj = project.get_object_by_name(&obj_name).unwrap();
      let obj = &mut *obj.lock().unwrap();
      obj.set_start(gst::USECOND * ((dst_time_ms * 1000.0) as u64));
      // todo: 一時停止しないとcommitで時々フリーズする(大きな動画？)
      project.pause();
      project.ges_timeline.commit();
    }

    self.emit.playing_changed();
  }

  fn timeline_add_file_object(&mut self, file_urls: String, dst_layer_id: u64, dst_time_ms: f32) {
    println!("Adding object to: {}", dst_layer_id);

    {
      let project = &mut *self.project.lock().unwrap();
      project.pause();

      let len = {
        let objects = &*project.objects.lock().unwrap();
        objects.len()
      };

      for url in file_urls.split("::::") {
        println!("Opening {}", url);

        &self.objects.model.begin_insert_rows(len, len); // Notify Qt

        let clip = ges::UriClip::new(&url).expect("Could not create clip");
        let mut obj = Object::new_from_uri_clip(&util::random_name_for_layer(), clip);
        obj.set_start(gst::USECOND * (dst_time_ms * 1000.0) as u64);
        let obj = Arc::new(Mutex::new(obj));
        project.add_object_to_layer(&obj, dst_layer_id as usize);

      }

      project.ges_timeline.commit();
    }

    self.emit.playing_changed();
    &self.objects.model.end_insert_rows();
  }

  fn timeline_remove_object(&mut self, obj_name: String) {
    let project = &mut *self.project.lock().unwrap();
    let obj = project.get_object_by_name(&obj_name).unwrap();
    let idx = project.find_object_index(&obj).unwrap();

    &self.objects.model.begin_remove_rows(idx, idx);
    project.remove_object_by_name(&obj_name);
    &self.objects.model.end_remove_rows();

    project.ges_timeline.commit_sync();
  }

  fn timeline_configure_text(&mut self, obj_name: String, dst_layer_id: u64, dst_time_ms: f32, text: String) {
    let project = &mut *self.project.lock().unwrap();
    if obj_name.is_empty() { // New Text object
      let len = {
        let objects = &*project.objects.lock().unwrap();
        objects.len()
      };

      // https://gstreamer.freedesktop.org/documentation/gst-editing-services/gesclip.html?gi-language=c#GESClip
      let clip = ges::TitleClip::new().unwrap();

      let mut obj = Object::new_from_title_clip(&util::random_name_for_layer(), clip);
      obj.set_start(gst::USECOND * (dst_time_ms * 1000.0) as u64);
      obj.set_duration(gst::SECOND * 5);
      let obj = Arc::new(Mutex::new(obj));

      &self.objects.model.begin_insert_rows(len, len); // Notify Qt
      project.add_object_to_layer(&obj, dst_layer_id as usize);
      &self.objects.model.end_insert_rows();

      let obj = &mut *obj.lock().unwrap();
      // todo: edit window
      // 先にLayerに追加してからでないと、ClipはTrackにアクセスできない
      match &obj.content {
        ObjectContent::Text { clip } => {
          clip.set_child_property("text", &text).unwrap();
          clip.set_child_property("posx", &20).unwrap();
          clip.set_child_property("posy", &20).unwrap();
          clip.set_child_property("height", &100).unwrap();
          clip.set_child_property("width", &100).unwrap();
          // clip.set_child_property("text-height", &100).unwrap();
          // clip.set_child_property("text-width", &100).unwrap();
          clip.set_child_property("font-desc", &"IPAPGothic 50").unwrap();
          clip.set_child_property("color", &(0x9900ffff as u32));
          clip.set_child_property("background-color", &(0x00000000 as u32));
          clip.set_child_property("foreground-color", &(0x00000000 as u32));
          // let aa = clip.get_child_property("text").unwrap();
          // println!("{:?}", aa.get::<String>());
          // clip.set_duration((gst::SECOND * 5));
        }
        _ => panic!("unreachable")
      }
    } else {
      // todo: if object already exists
    }

    project.ges_timeline.commit_sync();
  }

  fn timeline_configure_filter(&mut self, obj_name: String, dst_layer_id: u64, dst_time_ms: f32) {
    println!("Adding filter object to: {}", dst_layer_id);

    {
      if obj_name.is_empty() { // New Text object
        // let video_desc= "alpha method=green";
        let video_desc = "agingtv";

        // Designating "" as audio_desc causes an error since the entire description becomes 'bin.( audioconvert ! audioresample ! )' which is invalid
        // todo: find a way to pass NULL to GStreamer (native lib)
        let audio_desc = "audioamplify";
        let audio_desc = "audiopanorama";
        let clip = ges::EffectClip::new(video_desc, audio_desc).unwrap();
        clip.set_duration(gst::SECOND * 5);

        let mut obj = Object::new_from_effect_clip(&util::random_name_for_layer(), clip);
        obj.set_start(gst::USECOND * (dst_time_ms * 1000.0) as u64);
        let obj = Arc::new(Mutex::new(obj));

        {
          let project = &mut *self.project.lock().unwrap();
          let len = {
            let objects = &*project.objects.lock().unwrap();
            objects.len()
          };

          &self.objects.model.begin_insert_rows(len, len); // Notify Qt
          project.add_object_to_layer(&obj, dst_layer_id as usize);
        }
        &self.objects.model.end_insert_rows(); // Note that this will call TimelineObject's methods and lock the project at second hand

        let obj = &*obj.lock().unwrap();
        match &obj.content {
          ObjectContent::Filter { clip } => {
            // todo
            let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
            clip.add(&effect).unwrap();

            let effect = ges::Effect::new("audiopanorama").expect("Failed to create effect");
            clip.add(&effect).unwrap();
          }
          _ => panic!("unreachable")
        }
      } else {
        // todo: if object already exists
      }
    }
    {
      let project = &mut *self.project.lock().unwrap();
      project.pause();
      project.ges_timeline.commit();
    }

    self.emit.playing_changed();
  }

  fn timeline_change_object_inpoint(&mut self, obj_name: String, inpoint_ms: f32) {
    {
      let project = &mut *self.project.lock().unwrap();

      let obj = project.get_object_by_name(&obj_name).unwrap();
      let obj = &mut *obj.lock().unwrap();

      let diff = { // ClockTime cannot deal with negative values
        let start = &*obj.start.lock().unwrap();
        inpoint_ms as i64 - start.mseconds().unwrap() as i64
      };
      let new_len = {
        let len = &*obj.duration.lock().unwrap();
        let len = len.mseconds().unwrap() as i64;
        let max = obj.max_duration.mseconds().unwrap() as i64;
        std::cmp::min(max, len - diff)
      } as u64;
      obj.set_start(gst::MSECOND * inpoint_ms as u64);
      obj.set_duration(gst::MSECOND * new_len);

      println!("Start: {}, Duration: {}, Diff: {}", inpoint_ms, new_len, diff);

      match &obj.content {
        // change inpoint and duration
        ObjectContent::Clip { clip } => {
          let new_inpoint = if (diff > 0) {
            clip.get_inpoint() + gst::MSECOND * diff as u64
          } else {
            clip.get_inpoint() - gst::MSECOND * (-diff as u64)
          };
          clip.set_inpoint(new_inpoint);
        },
        ObjectContent::Text { clip } => {
          let new_inpoint = if (diff > 0) {
            clip.get_inpoint() + gst::MSECOND * diff as u64
          } else {
            clip.get_inpoint() - gst::MSECOND * (-diff as u64)
          };
          clip.set_inpoint(new_inpoint);
        },
        ObjectContent::Filter { clip } => {
          let new_inpoint = if (diff > 0) {
            clip.get_inpoint() + gst::MSECOND * diff as u64
          } else {
            clip.get_inpoint() - gst::MSECOND * (-diff as u64)
          };
          clip.set_inpoint(new_inpoint);
        },
        _ => {}
      }

      project.pause();
      project.ges_timeline.commit();
    }

    self.emit.playing_changed();
  }

  fn timeline_change_object_outpoint(&mut self, obj_name: String, outpoint_ms: f32) {
    {
      let project = &mut *self.project.lock().unwrap();

      let obj = project.get_object_by_name(&obj_name).unwrap();
      let obj = &mut *obj.lock().unwrap();

      let new_outpoint = gst::USECOND * (outpoint_ms * 1000.0) as u64;
      let new_len = {
        let start = &*obj.start.lock().unwrap();

        new_outpoint - start
      };
      println!("setting duration to: {}", new_len);
      obj.set_duration(new_len);

      project.pause();
      project.ges_timeline.commit();
    }

    self.emit.playing_changed();
  }

  fn timeline_apply_object_filter(&mut self, obj_name: String, description: String) {
    let project = &mut *self.project.lock().unwrap();
    let obj = project.get_object_by_name(&obj_name).unwrap();
    let obj = &*obj.lock().unwrap();

    let effect = ges::Effect::new(&description).unwrap();

    match &obj.content {
      ObjectContent::Clip { clip } => {
        clip.add(&effect).unwrap();
      },
      ObjectContent::Filter { clip } => {
        clip.add(&effect).unwrap();
      },
      ObjectContent::Text { clip } => {
        clip.add(&effect).unwrap();
      },
      _ => panic!("unimplemented")
    }

    project.ges_timeline.commit_sync();
  }

  fn timeline_set_object_x(&mut self, obj_name: String, x: i64) {
    let project = &mut *self.project.lock().unwrap();
    let obj = project.get_object_by_name(&obj_name).unwrap();
    let obj = &*obj.lock().unwrap();

    match &obj.content {
      ObjectContent::Clip { clip } => {
        // get track
        let tracks = project.ges_timeline.get_tracks();
        let mut track: Option<ges::Track> = None;
        for t in tracks {
          let prop = t.get_property("track-type").unwrap();
          println!("track type: {:?}", prop.get::<ges::TrackType>());
          match prop.get::<ges::TrackType>().unwrap() {
            Some(ges::TrackType::VIDEO) => {
              track = Some(t);
              break;
            }
            _ => {}
          }
        }

        // see: http://gstreamer-devel.966125.n4.nabble.com/GES-resizing-and-positioning-clips-td4666919.html
        let track = track.expect("No video tracks found");
        let track_el = clip.find_track_element(Some(&track), glib::types::Type::Unit);
        match track_el {
          Some(track_el) => {
            println!("track element: {:?}", track_el);
            track_el.set_child_property("posx", &x).unwrap();
          },
          None => panic!("Unreachable")
        }

        // let effects = clip.get_top_effects();
        // let mut videobox: Option<ges::TrackElement> = None;
        // for effect in effects {
        //   println!("name: {}", effect.get_name().unwrap());
        //   // todo: internal name convention for effects? + avoid hard-coding
        //   if effect.get_name().unwrap() == "uilau_default_videobox" {
        //     videobox = Some(effect);
        //   }
        // }
        //
        // let desc = format!("videobox border-alpha=0 left={}", -x);
        // println!("{}", desc);
        // match videobox {
        //   Some(vbox) => {
        //     vbox.set_property("bin-description", &desc);
        //   },
        //   None => {
        //     let effect = ges::Effect::new(&desc).expect("Failed to create effect");
        //     clip.add(&effect).unwrap();
        //     effect.set_name(Some("uilau_default_videobox")).unwrap();
        //   }
        // }
      },
      ObjectContent::Text { clip } => {
        panic!("at the disco")
      },
      _ => panic!("unimplemented")
    }

    project.pause();
    project.ges_timeline.commit();
  }
}

impl App {
  fn setup() -> (Project, gst::Element) {
    let proj = Self::setup_sample_project();

    let sink = gst::ElementFactory::make("qmlglsink", None).unwrap();
    let sinkbin = gst::ElementFactory::make("glsinkbin", None).unwrap();  

    proj.ges_pipeline.preview_set_video_sink(&sinkbin);

    sinkbin
      .set_property("sink", &sink.to_value())
      .unwrap();

    (proj, sink)
  }

  fn create_sample_clip() -> ges::UriClip {
    let uri = "file:///usr/share/big-buck-bunny_trailer.webm";
    let clip = ges::UriClip::new(uri).expect("failed to create clip.");
    clip
  }

  fn setup_sample_project() -> Project {
    let mut proj = Project::new();
    let layer_ = proj.add_layer();

    let layer_idx = proj.find_layer_idx(&layer_).unwrap();

    let clip = Self::create_sample_clip();
    let mut obj = Object::new_from_uri_clip("bigbunny", clip);
    obj.set_start(gst::SECOND * 2);
    let obj = Arc::new(Mutex::new(obj));
    proj.add_object_to_layer(&obj, layer_idx);

    proj.add_layer();

    proj
  }

  fn wait_for_pointer(&self) {
    use std::thread;
    use std::time::{Duration, Instant};

    let sink = self.sink.clone();
    thread::spawn(move || {
      loop {
        unsafe {
          if VIDEO_ITEM != 0 as *const usize {
            println!("Address of sink Rust gives C++: {:?}", (*sink).as_ptr());
            println!("Address of videoItem Rust gives C++: {:?}", VIDEO_ITEM);
            set_widget_to_sink((*sink).as_ptr(), VIDEO_ITEM);  
            break
          }
        }
        thread::sleep(Duration::from_millis(50));
      }
    });
  }
}