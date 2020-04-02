use std::sync::{Arc, Mutex, Weak};

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
    let mut obj = &*object.lock().unwrap();
    // self cannot own self itself
    // obj.layer = Some(Weak::new(Mutex::new(self)));

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

  pub fn remove_object_by_id(&mut self, id: &str) {
    let objs = &mut *self.objects.lock().unwrap();
    let obj = objs.remove(&String::from(id));

    if let Some(obj) = obj {
      println!("trying to lock...");
      let obj = &*obj.lock().unwrap();
      println!("got lock 7");
      match obj.kind {
        ObjectKind::Clip => {
          let clip = obj.clip.as_ref().expect("No clip is set");
          self.ges_layer.remove_clip(clip).unwrap();
        }
        _ => {}
      }
    }
  }

  pub fn remove_object(&mut self, obj: &Object) {
    let objs = &mut *self.objects.lock().unwrap();
    objs.remove(&obj.id).expect("Object not found in layer");

    match obj.kind {
      ObjectKind::Clip => {
        let clip = obj.clip.as_ref().expect("No clip is set");
        self.ges_layer.remove_clip(clip).unwrap();
      }
      _ => {}
    }
  }

}