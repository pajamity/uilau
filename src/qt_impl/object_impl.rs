extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex};

use crate::interface::*;
use crate::project::*;
use crate::object::{Object, ObjectKind};

impl TimelineObjectsTrait for Object {
  fn new(emit: TimelineObjectsEmitter, model: TimelineobjectsList) -> Self {

  }

  fn emit(&mut self) -> &mut TimelineObjectsEmitter {
  }
}