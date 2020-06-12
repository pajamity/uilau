extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::object::{ObjectKind, Object};
use super::layer::Layer;

#[derive(Clone)]
pub struct Project {
  pub ges_timeline: ges::Timeline,
  // todo: Arc<Mutex<Vec>> necessary?
  // pub layers: HashMap<String, Arc<Mutex<Layer>>>,
  pub layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>,
  pub objects: HashMap<String, Arc<Mutex<Object>>>,
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
      objects: HashMap::new(),
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

  pub fn get_layer(&self, layer_id: &str) -> Arc<Mutex<Layer>> {
    let layers = &*self.layers.lock().unwrap();
    layers[String::from(layer_id)].clone()
  }

  pub fn add_object(&mut self, object: &Arc<Mutex<Object>>) {
    let obj_ = object.clone();
    let mut obj = &*object.lock().unwrap();

    let id = String::from(&obj.id);
    self.objects.insert(id, obj_);
  }
}