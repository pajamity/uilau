use std::sync::{Arc, Mutex, Weak};

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::collections::HashMap;

use super::project::Project;
use super::object::{ObjectKind, Object};

use crate::qt_impl::*;

#[derive(Clone)]
pub struct Layer {
  pub id: String,
  pub name: Arc<Mutex<String>>,
  pub ges_layer: ges::Layer,

  pub project: Option<Weak<Project>>,
}

impl Layer {
  pub fn new(layer: ges::Layer) -> Self {
    let objects = HashMap::new();

    let s = Self {

      ges_layer: layer,
      project: None,
    };

    s
  }

  pub fn set_project(&mut self, project: &Arc<Project>) {
    self.project = Some(project.downgrade().unwrap());
  }

  pub fn objects(&self) -> HashMap<String, Arc<Mutex<Object>>> {
    let proj = self.project.expect("Project is not set!");
    let proj = &*proj.upgrade().unwrap();

    proj.objects.into_iter()
      .filter(|&(_, obj)| {
        let obj = *obj.lock().unwrap();
        if let Some(layer) = obj.layer {
          if let Some(layer) = layer.upgrade() {
            let layer = layer.lock().unwrap();
            self.id == layer.id
          }
        }
        false
      })
      .collect()
  }

  pub fn add_object(&mut self, object: Arc<Mutex<Object>>) {
    self.project.add_object(object.clone());

    let obj = &*object.lock().unwrap();
    let arc = self.project.get_layer() // Layer doesn't own itself
    obj.set_layer();

    match obj.kind {
      ObjectKind::Clip => {
        let clip = obj.clip.as_ref().expect("No clip is set");
        self.ges_layer.add_clip(clip).unwrap();
      }
      _ => {}
    }
  }

  pub fn remove_object_by_id(&mut self, id: &str) {
    let objs = &mut *self.objects.lock().unwrap();
    let obj = objs.remove(&String::from(id));

    if let Some(obj) = obj {
      let obj = &*obj.lock().unwrap();
      match obj.kind {
        ObjectKind::Clip => {
          let clip = obj.clip.as_ref().expect("No clip is set");
          self.ges_layer.remove_clip(clip).unwrap();
        }
        _ => {}
      }
    }
  }

  pub fn remove_object(&mut self, obj: &mut Object) {
    let objs = &mut *self.objects.lock().unwrap();
    objs.remove(&obj.id).expect("Object not found in layer");
    // obj.layer = None;

    match obj.kind {
      ObjectKind::Clip => {
        let clip = obj.clip.as_ref().expect("No clip is set");
        self.ges_layer.remove_clip(clip).unwrap();
      }
      _ => {}
    }
  }

  pub fn qt_representation(&self) -> TimelineObjects {
    Timel
  }
}