extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum ObjectKind {
  Video,
  Audio,
  Text,
  Shape,
  Filter
}

#[derive(Clone)]
pub struct Object {
  pub id: Arc<Mutex<String>>,
  pub name: Arc<Mutex<String>>,
  pub kind: Arc<ObjectKind>,
  pub length: Arc<Mutex<gst::ClockTime>>,
  pub clip: Option<Arc<Mutex<ges::Clip>>>, // or Arc<Mutex<Option<Clip>>> ?

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,
  pub layer_id: Arc<Mutex<i32>>,
}

impl Object {
  pub fn new(id: &str, name: &str, kind: ObjectKind, length: gst::ClockTime, start: gst::ClockTime, layer_id: i32) -> Self {
    let s = Self {
      id: Arc::new(Mutex::new(id.to_string())),
      name: Arc::new(Mutex::new(name.to_string())),
      kind: Arc::new(kind),
      length: Arc::new(Mutex::new(length)),
      clip: None,

      start: Arc::new(Mutex::new(start)),
      layer_id: Arc::new(Mutex::new(layer_id)),
    };

    s
  }
}

