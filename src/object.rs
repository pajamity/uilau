extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex, Weak};

use super::layer::Layer;

#[derive(Clone, Copy)]
pub enum ObjectKind {
  Clip, // corresponds to ges::Clip (should be splitted into Video & Audio?)
  Video,
  Audio,
  Text,
  Shape,
  Filter
}

#[derive(Clone)]
pub struct Object {
  // pub id: Arc<Mutex<String>>,
  pub id: String,
  pub name: Arc<Mutex<String>>,
  pub kind: ObjectKind,
  pub length: Arc<Mutex<gst::ClockTime>>,
  pub clip: Option<ges::UriClip>,

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,
  pub layer_id: Arc<Mutex<i32>>,

  pub layer: Option<Weak<Mutex<Layer>>>,
}

impl Object {
  pub fn new(id: &str, name: &str, kind: ObjectKind, length: gst::ClockTime, start: gst::ClockTime, layer_id: i32) -> Self {
    let s = Self {
      id: id.to_string(),
      name: Arc::new(Mutex::new(name.to_string())),
      kind,
      length: Arc::new(Mutex::new(length)),
      clip: None,

      start: Arc::new(Mutex::new(start)),
      layer_id: Arc::new(Mutex::new(layer_id)),
      layer: None,
    };

    s
  }

  pub fn new_from_uri_clip(id: &str, name: &str, start: gst::ClockTime, layer_id: i32, clip: ges::UriClip) -> Self {
    let asset = clip.get_asset().unwrap();
    let length = asset
      .downcast::<ges::UriClipAsset>()
      .unwrap()
      .get_duration();

    Self {
      id: id.to_string(),
      name: Arc::new(Mutex::new(name.to_string())),
      kind: ObjectKind::Clip,
      length: Arc::new(Mutex::new(length)),
      clip: Some(clip),

      start: Arc::new(Mutex::new(start)),
      layer_id: Arc::new(Mutex::new(layer_id)),

      layer: None
    }
  }

  pub fn set_layer(&mut self, layer: Arc<Mutex<Layer>>) {
    self.layer = Some(Arc::downgrade(&layer));
  }

  pub fn set_start(&mut self, val: gst::ClockTime) {
    println!("moving object to {}", val);
    let mut start = *self.start.lock().unwrap();
    println!("got lock for start");
    start = val;

    if let Some(clip) = &self.clip {
      clip.set_start(val);
    }
  }
}

