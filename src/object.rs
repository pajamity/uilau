extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex};

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
  pub kind: Arc<ObjectKind>,
  pub length: Arc<Mutex<gst::ClockTime>>,
  pub clip: Option<ges::UriClip>, // or Arc<Mutex<Option<Clip>>> ?

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,
  pub layer_id: Arc<Mutex<i32>>,
}

impl Object {
  pub fn new(id: &str, name: &str, kind: ObjectKind, length: gst::ClockTime, start: gst::ClockTime, layer_id: i32) -> Self {
    let s = Self {
      // id: Arc::new(Mutex::new(id.to_string())),
      id: id.to_string(),
      name: Arc::new(Mutex::new(name.to_string())),
      kind: Arc::new(kind),
      length: Arc::new(Mutex::new(length)),
      clip: None,

      start: Arc::new(Mutex::new(start)),
      layer_id: Arc::new(Mutex::new(layer_id)),
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
      kind: Arc::new(ObjectKind::Clip),
      length: Arc::new(Mutex::new(length)),
      clip: Some(clip),

      start: Arc::new(Mutex::new(start)),
      layer_id: Arc::new(Mutex::new(layer_id)),
    }
  }
}

