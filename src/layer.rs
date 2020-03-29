use std::sync::{Arc, Mutex};

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use super::object::{ObjectKind, Object};

#[derive(Clone)]
pub struct Layer {
  pub ges_layer: ges::Layer,
  pub objects: Arc<Mutex<Vec<Arc<Mutex<Object>>>>>
}

impl Layer {
  pub fn new(layer: ges::Layer) -> Self {
    let s = Self {
      ges_layer: layer,
      objects: Arc::new(Mutex::new(vec![])),
    };

    s
  }

  pub fn add_object(&mut self, object: Arc<Mutex<Object>>) {
    let object_ = object.clone();
    let obj = &*object.lock().unwrap();
    match *obj.kind {
      ObjectKind::Clip => {
        let clip = obj.clip.as_ref().expect("No clip is set");
        self.ges_layer.add_clip(clip).unwrap();
      }
      _ => {}
    }

    let objs = &mut *self.objects.lock().unwrap();
    objs.push(object_);
  }

}