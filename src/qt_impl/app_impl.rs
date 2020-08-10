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

  fn play(&mut self) {
    let project = &*self.project.lock().unwrap();
    project.ges_pipeline
      .set_state(gst::State::Playing)
      .expect("could not change the state");
  }
  
  fn pause(&mut self) {
    let project = &*self.project.lock().unwrap();
    project.ges_pipeline
      .set_state(gst::State::Paused)
      .expect("could not change the state");
  }

  fn duration_ms(&self) -> u64 {
    let project = &*self.project.lock().unwrap();
    if let Some(dur) = project.ges_pipeline.query_duration::<gst::ClockTime>() {
      let ms = dur.mseconds().unwrap();
      println!("dur {} vs {}", ms, project.ges_timeline.get_duration().mseconds().unwrap());
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

  fn seek_to(&mut self, to: u64) {
    println!("called");
    let project = &*self.project.lock().unwrap();
    println!("kalled {}", to);
    if project.ges_pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, gst::MSECOND * (to as u64))
      .is_err() {
      println!("Seeking failed");
    }
  }

  fn move_timeline_object(&mut self, obj_name: String, dst_layer_id: u64, dst_time_ms: f32) {
    let project = &mut *self.project.lock().unwrap();

    project.move_object_to_layer(&obj_name, dst_layer_id as usize);
    let obj = project.get_object_by_name(&obj_name).unwrap();
    let obj = &mut *obj.lock().unwrap();
    obj.set_start(gst::USECOND * ((dst_time_ms * 1000.0) as u64));

    // todo: commitで時々フリーズする(大きな動画？)
    project.ges_timeline.commit_sync();

    //
    // let dst_layer_id = dst_layer_id as usize;
    // // fixme: not efficient if we had lots of objects
    // let layers = project.layers.clone();
    // let dst_layer = project.get_layer(dst_layer_id as usize);
    //
    // let layers = &mut *layers.lock().unwrap();
    // for (layer_id, layer) in layers.iter().enumerate() {
    //   let layer = &mut *layer.lock().unwrap();
    //   let objects = layer.objects().clone();
    //   for (_, obj) in objects {
    //     let objj = obj.clone();
    //     let mut obj = &mut *obj.lock().unwrap();
    //     if obj.id == object_id {
    //       println!("Found obj: {}", obj.id);
    //
    //       // move between layers
    //       if layer_id != dst_layer_id {
    //         layer.remove_object(&mut obj);
    //
    //         let dst_idx = project.find_layer_idx(&dst_layer).unwrap();
    //         project.add_object_to_layer(&obj, dst_idx);
    //       }
    //
    //       // move inside layers
    //       obj.set_start(((dst_time_ms * 1000.0) as u64) * gst::USECOND);
    //     }
    //   }
    // }
  }

  fn timeline_add_file_object(&mut self, file_urls: String, dst_layer_id: u64, dst_time_ms: f32) {
    let project = &mut *self.project.lock().unwrap();

    let len = {
      let objects = &*project.objects.lock().unwrap();
      objects.len()
    };

    for url in file_urls.split("::::") {
      println!("Opening {}", url);

      &self.objects.model.begin_insert_rows(len, len); // Notify Qt

      let clip = ges::UriClip::new(&url).expect("Could not create clip");
      let mut obj = Object::new_from_uri_clip( &util::random_name_for_layer(), clip);
      obj.set_start(gst::USECOND * (dst_time_ms * 1000.0) as u64);
      let obj = Arc::new(Mutex::new(obj));
      project.add_object_to_layer(&obj, dst_layer_id as usize);

      &self.objects.model.end_insert_rows();
    }
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
      let obj = Arc::new(Mutex::new(obj));

      &self.objects.model.begin_insert_rows(len, len); // Notify Qt
      project.add_object_to_layer(&obj, dst_layer_id as usize);
      &self.objects.model.end_insert_rows();

      let obj = &*obj.lock().unwrap();
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
          let aa = clip.get_child_property("text").unwrap();
          println!("aaa {:?}", aa.get::<String>());
          clip.set_duration((gst::SECOND * 5));
        }
        _ => panic!("unreachable")
      }
    } else {
      // todo: if object already exists
    }

    project.ges_timeline.commit_sync();
  }

  fn timeline_configure_filter(&mut self, obj_name: String, dst_layer_id: u64, dst_time_ms: f32) {
    let project = &mut *self.project.lock().unwrap();
    if obj_name.is_empty() { // New Text object
      let len = {
        let objects = &*project.objects.lock().unwrap();
        objects.len()
      };

      let video_desc= "alpha method=green";
      // let video_desc= "videobalanceoooooo saturation=1.5 hue=+0.5";
    //   let video_desc= "mixer.sink_0 \
    // videotestsrc pattern=smpte75 ! alpha method=green ! mixer.sink_1 \
    // videomixer name=mixer sink_0::zorder=0 sink_1::zorder=1 !";
      let audio_desc= "";
   //    let video_desc= " gst-launch-1.0 videotestsrc pattern=snow ! mixer.sink_0 \
   // videotestsrc pattern=smpte75 ! alpha method=green ! mixer.sink_1 \
   // videomixer name=mixer sink_0::zorder=0 sink_1::zorder=1 ! \
   // videoconvert ! autovideosink";
      let clip = ges::EffectClip::new(video_desc, audio_desc).unwrap();

      let mut obj = Object::new_from_effect_clip(&util::random_name_for_layer(), clip);
      obj.set_start(gst::USECOND * (dst_time_ms * 1000.0) as u64);
      let obj = Arc::new(Mutex::new(obj));
      println!("weaaaaakkk");

      &self.objects.model.begin_insert_rows(len, len); // Notify Qt
      project.add_object_to_layer(&obj, dst_layer_id as usize);
      &self.objects.model.end_insert_rows();          println!("weaaaaa");


      let obj = &*obj.lock().unwrap();
      match &obj.content {
        ObjectContent::Filter { clip } => {
          clip.set_duration(gst::SECOND * 5);
        }
        _ => panic!("unreachable")
      }

    } else {
      // todo: if object already exists
    }

    project.ges_timeline.commit_sync();
  }

}

impl App {
  fn setup() -> (Project, gst::Element) {
    // let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    // playbin.set_property("uri", &glib::Value::from("file:///usr/share/big-buck-bunny_trailer.webm")).unwrap();
    let proj = Self::setup_sample_project();

    let sink = gst::ElementFactory::make("qmlglsink", None).unwrap();
    let sinkbin = gst::ElementFactory::make("glsinkbin", None).unwrap();  

    proj.ges_pipeline.preview_set_video_sink(&sinkbin);

    sinkbin
      .set_property("sink", &sink.to_value())
      .unwrap();
    // playbin
    //   .set_property("video-sink", &sinkbin.to_value())
    //   .unwrap();

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

    // let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
    // clip.add(&effect).unwrap();
  
    // let asset = clip.get_asset().unwrap();
    // let duration = asset
    //   .downcast::<ges::UriClipAsset>()
    //   .unwrap()
    //   .get_duration();
    //
    // clip.set_inpoint(duration / 2);
    // clip.set_duration(duration / 4);
  
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