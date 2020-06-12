extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex};

use crate::interface::*;
use crate::ffi::*;
use crate::project::*;
use crate::object::{Object, ObjectKind};

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
  pub project: Project,
  pub sink: Arc<gst::Element>,
}

impl AppTrait for App {
  fn new(emit: AppEmitter, layers: Layers, objects: TimelineObjects) -> Self {
    let (project, sink) = Self::setup();

    let mut s = Self {
      emit,
      project,
      layers,
      objects,
      sink: Arc::new(sink)
    };
    
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
    self.project.ges_pipeline
      .set_state(gst::State::Playing)
      .expect("could not change the state");
  }
  
  fn pause(&mut self) {
    self.project.ges_pipeline
      .set_state(gst::State::Paused)
      .expect("could not change the state");
  }

  fn duration_ms(&self) -> u64 {
    if let Some(dur) = self.project.ges_pipeline.query_duration::<gst::ClockTime>() {
      let ms = dur.mseconds().unwrap();
      return ms;
    }
    0
  }

  fn position_ms(&self) -> u64 {
    if let Some(pos) = self.project.ges_pipeline.query_position::<gst::ClockTime>() {
      let ms = pos.mseconds().unwrap();
      return ms;
    }
    0
  }

  fn seek_to(&mut self, to: u64) {
    if self.project.ges_pipeline.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, (to as u64) * gst::MSECOND)
      .is_err() {
      eprintln!("Seeking failed");
    }
  }

  fn move_timeline_object(&self, object_id: String, dst_layer_id: u64, dst_time_ms: f32) {
    let dst_layer_id = dst_layer_id as usize;
    // fixme: not efficient if we had lots of objects
    let layers = self.project.layers.clone();
    let dst_layer = self.project.get_layer(dst_layer_id as usize);

    let layers = &mut *layers.lock().unwrap();
    for (layer_id, layer) in layers.iter().enumerate() {
      let layer = &mut *layer.lock().unwrap();
      let objects = layer.objects.clone();
      let objects = &*objects.lock().unwrap();
      for (_, obj) in objects {
        let objj = obj.clone();
        let mut obj = &mut *obj.lock().unwrap();
        if obj.id == object_id {
          println!("Found obj: {}", obj.id);

          // move between layers
          if layer_id != dst_layer_id {
            layer.remove_object(&mut obj);

            let d = dst_layer.clone();
            obj.set_layer(d);
            let dst_layer = &mut *dst_layer.lock().unwrap();
            dst_layer.add_object(objj);
          }

          // move inside layers
          obj.set_start((dst_time_ms * 1000.0) as u64 * gst::USECOND);
        }
      }
    }
  }

  fn timeline_add_file_object(&mut self, file_urls: String, dst_layer_id: u64, dst_time_ms: f32) {
    println!("ff {}", file_urls);
    for url in file_urls.split("::::") {
      println!("Opening {}", url);

      let clip = ges::UriClip::new(&url).expect("Could not create clip");
      let layer_ = self.project.get_layer(dst_layer_id as usize);
      let mut obj = Object::new_from_uri_clip("v9", "piyo", (dst_time_ms * 1000.0) as u64 * gst::USECOND, clip);
      let layer = &mut *layer_.lock().unwrap();
      obj.set_layer(layer_.clone());
      layer.add_object(Arc::new(Mutex::new(obj)));
    }
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
    let layer = &mut *layer_.lock().unwrap();
  
    let clip = Self::create_sample_clip();
    let mut obj = Object::new_from_uri_clip("v1", "bigbunny", 10 * gst::SECOND, clip);
    obj.set_layer(layer_.clone());
    layer.add_object(Arc::new(Mutex::new(obj)));
  
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