extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex};

use super::object::{ObjectKind, Object};
use super::layer::Layer;

#[derive(Clone)]
pub struct Project {
  pub ges_timeline: ges::Timeline,
  pub layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>,
  pub ges_pipeline: ges::Pipeline,
}

impl Project {
  pub fn new() -> Self {
    let ges_timeline = ges::Timeline::new_audio_video();
    let ppl = ges::Pipeline::new();
    ppl.set_timeline(&ges_timeline).unwrap();

    let s = Self {
      ges_timeline,
      layers: Arc::new(Mutex::new(vec![])),
      ges_pipeline: ppl,
    };

    s
  }

  pub fn add_layer(&mut self) -> Arc<Mutex<Layer>> {
    let ges_layer = self.ges_timeline.append_layer();
    let layer = Arc::new(Mutex::new(Layer::new(ges_layer)));
    let layers = &mut *self.layers.lock().unwrap();
    let ret = layer.clone();
    &layers.push(layer);

    ret
  }
}