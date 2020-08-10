extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex, Weak};

use crate::interface::*;
use crate::project::*;
use crate::object::{Object, ObjectContent};

pub struct TimelineObjects {
  pub emit: TimelineObjectsEmitter,
  pub model: TimelineObjectsList,

  pub objects: Option<Arc<Mutex<Vec<Arc<Mutex<Object>>>>>>,
  pub project: Option<Weak<Mutex<Project>>>,
}

impl TimelineObjects {
  pub fn set_objects(&mut self, objs: &Arc<Mutex<Vec<Arc<Mutex<Object>>>>>) {
    self.objects = Some(objs.clone());
  }

  pub fn set_project(&mut self, project: &Arc<Mutex<Project>>) {
    self.project = Some(Arc::downgrade(project));
  }

  fn get_obj(&self, idx: usize) -> Option<Arc<Mutex<Object>>> {
    if let Some(objs) = &self.objects {
      let objs = &*objs.lock().unwrap();
      Some(objs[idx].clone())
    } else { None }
  }
}

impl TimelineObjectsTrait for TimelineObjects {
  fn new(emit: TimelineObjectsEmitter, model: TimelineObjectsList) -> Self {
    Self {
      emit,
      model,
      objects: None,
      project: None
    }
  }

  fn emit(&mut self) -> &mut TimelineObjectsEmitter {
    &mut self.emit
  }

  fn row_count(&self) -> usize {
    if let Some(objs) = &self.objects {
      let objs = &*objs.lock().unwrap();
      objs.len()
    } else {
      0
    }
  }

  fn kind(&self, index: usize) -> &str {
    let obj = self.get_obj(index).unwrap();
    let obj = &*obj.lock().unwrap();
    match obj.content {
      ObjectContent::Audio => "audio",
      ObjectContent::Video => "video",
      ObjectContent::Clip{ clip:_ } => "clip",
      ObjectContent::Filter{ clip:_ } => "filter",
      ObjectContent::Shape => "shape",
      ObjectContent::Text{ clip:_ } => "text"
    }
  }

  fn layer_id(&self, index: usize) -> u64 {
    let obj = self.get_obj(index).unwrap();
    let obj = &*obj.lock().unwrap();
    let project = self.project.as_ref().unwrap().upgrade().unwrap();
    let project = &*project.lock().unwrap();
    if let Some(layer) = &obj.layer {
      let layer = layer.upgrade().unwrap();
      project.find_layer_idx(&layer).unwrap() as u64
    } else {
      9999999 // todo
    }
  }

  fn length_ms(&self, index: usize) -> u64 {
    let obj = self.get_obj(index).unwrap();
    let obj = &*obj.lock().unwrap();
    let len = *obj.length.lock().unwrap();
    len.mseconds().unwrap()
  }

  fn name(&self, index: usize) -> &str {
    let obj = self.get_obj(index).unwrap();
    let obj = &*obj.lock().unwrap();
    let name = &*obj.name.lock().unwrap();
    Box::leak(format!("{}", name).into_boxed_str())
  }

  fn start_ms(&self, index: usize) -> u64 {
    let obj = self.get_obj(index).unwrap();
    let obj = &*obj.lock().unwrap();
    let st = *obj.start.lock().unwrap();
    st.mseconds().unwrap()
  }
}