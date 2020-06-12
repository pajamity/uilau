extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::interface::*;
use crate::project::*;
use crate::object::{Object, ObjectKind};

pub struct TimelineObjects {
  emit: TimelineObjectsEmitter,
  model: TimelineObjectsList,

  pub objects: Option<Arc<Mutex<HashMap<String, Arc<Mutex<Object>>>>>>
}

impl TimelineObjects {
  fn set_objects(&mut self, objs: &Arc<Mutex<HashMap<String, Arc<Mutex<Object>>>>>) {
    self.objects = Some(objs.clone());
  }
}

impl TimelineObjectsTrait for TimelineObjects {
  fn new(emit: TimelineObjectsEmitter, model: TimelineObjectsList) -> Self {
    Self {
      emit,
      model,
      objects: None
    }
  }

  fn emit(&mut self) -> &mut TimelineObjectsEmitter {
    &mut self.emit
  }

  fn row_count(&self) -> usize {

  }
}