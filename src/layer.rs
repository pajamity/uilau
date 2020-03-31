use std::sync::{Arc, Mutex};

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::collections::HashMap;

use super::object::{ObjectKind, Object};

#[derive(Clone)]
pub struct Layer {
  pub ges_layer: ges::Layer,
  pub objects: Arc<Mutex<HashMap<String, Arc<Mutex<Object>>>>>
}

impl Layer {
  pub fn new(layer: ges::Layer) -> Self {
    let objects = HashMap::new();

    let s = Self {
      ges_layer: layer,
      objects: Arc::new(Mutex::new(objects)),
    };

    s
  }

  // Layer "owns" objects
  pub fn add_object(&mut self, object: Arc<Mutex<Object>>) {
    let obj_ = object.clone();
    let obj = &*object.lock().unwrap();

    match obj.kind {
      ObjectKind::Clip => {
        let clip = obj.clip.as_ref().expect("No clip is set");
        self.ges_layer.add_clip(clip).unwrap();
      }
      _ => {}
    }

    let objs = &mut *self.objects.lock().unwrap();
    let id = String::from(&obj.id);
    objs.insert(id, obj_);
  }

  pub fn remove_object(&mut self, id: &str) {
    let objs = &mut *self.objects.lock().unwrap();
    objs.remove(&String::from(id));

    // TODO: remove from ges_layer
  }

}