extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex, Weak};

use super::layer::Layer;
use crate::interface::*;
use crate::project::*;

use crate::interface::TimelineObjectsEmitter;

#[derive(Clone, Copy)]
pub enum ObjectKind {
  Clip(ges::UriClip), // corresponds to ges::Clip (should be splitted into Video & Audio?)
  Video,
  Audio,
  Text(ges::TitleClip),
  Shape,
  Filter
}

// #[derive(Clone)]
pub struct Object {
  pub name: Arc<Mutex<String>>,
  pub kind: ObjectKind,
  pub length: Arc<Mutex<gst::ClockTime>>,
  pub clip: Option<ges::UriClip>,

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,

  pub layer: Option<Weak<Mutex<Layer>>>,
}

impl Object {
  pub fn new(name: &str, kind: ObjectKind, length: gst::ClockTime, start: gst::ClockTime) -> Self {
    let s = Self {
      name: Arc::new(Mutex::new(name.to_string())),
      kind,
      length: Arc::new(Mutex::new(length)),
      clip: None,

      start: Arc::new(Mutex::new(start)),
      layer: None,
    };

    s
  }

  pub fn new_from_uri_clip(name: &str, start: gst::ClockTime, clip: ges::UriClip) -> Self {
    let asset = clip.get_asset().unwrap();
    let length = asset
      .downcast::<ges::UriClipAsset>()
      .unwrap()
      .get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      kind: ObjectKind::Clip,
      length: Arc::new(Mutex::new(length)),
      clip: Some(clip),

      start: Arc::new(Mutex::new(start)),
      layer: None
    }
  }

  pub fn new_from_title_clip(name: &str, start: gst::ClockTime, clip: ges::TitleClip) -> Self {
    let asset = clip.get_asset().unwrap();
    let length = asset
      .downcast::<ges::Asset>()
      .unwrap()
      .get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      kind: ObjectKind::Text,
      length: Arc::new(Mutex::new(length)),
      clip: None, //Some(clip),
      // todo: add ges::TitleClip

      start: Arc::new(Mutex::new(start)),
      layer: None
    }
  }

  pub fn set_layer(&mut self, layer: &Arc<Mutex<Layer>>) {
    self.layer = Some(Arc::downgrade(layer));
  }

  pub fn set_start(&mut self, val: gst::ClockTime) {
    println!("moving object to {}", val);
    let mut start = *self.start.lock().unwrap();
    println!("prev: {}", start);
    start = val;

    if let Some(clip) = &self.clip {
      println!("set start for clip");
      clip.set_start(val);
    }
  }
}

