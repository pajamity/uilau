extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex, Weak};

use super::layer::Layer;
use crate::interface::*;
use crate::project::*;

use crate::interface::TimelineObjectsEmitter;

#[derive(Clone)]
pub enum ObjectContent {
  Clip {
    clip: ges::UriClip
  }, // corresponds to ges::Clip (should be splitted into Video & Audio?)
  Video,
  Audio,
  Text {
    clip: ges::TitleClip
  },
  Shape,
  Filter {
    clip: ges::EffectClip
  }
}

#[derive(Clone)]
pub struct Object {
  pub name: Arc<Mutex<String>>,
  pub content: ObjectContent,
  pub length: Arc<Mutex<gst::ClockTime>>,

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,

  pub layer: Option<Weak<Mutex<Layer>>>,
}

impl Object {
  pub fn new(name: &str, content: ObjectContent, length: gst::ClockTime, start: gst::ClockTime) -> Self {
    let s = Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content,
      length: Arc::new(Mutex::new(length)),

      start: Arc::new(Mutex::new(start)),
      layer: None,
    };

    s
  }

  pub fn new_from_uri_clip(name: &str, clip: ges::UriClip) -> Self {
    let asset = clip.get_asset().unwrap();
    let length = asset
      .downcast::<ges::UriClipAsset>()
      .unwrap()
      .get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Clip { clip },
      length: Arc::new(Mutex::new(length)),

      start: Arc::new(Mutex::new(gst::SECOND * 0)),
      layer: None
    }
  }

  pub fn new_from_title_clip(name: &str, clip: ges::TitleClip) -> Self {
    let length = clip.get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Text { clip },
      length: Arc::new(Mutex::new(length)),

      start: Arc::new(Mutex::new(gst::MSECOND * 0)),
      layer: None
    }
  }

  pub fn new_from_effect_clip(name: &str, clip: ges::EffectClip) -> Self {
    let length = clip.get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Filter { clip },
      length: Arc::new(Mutex::new(length)),

      start: Arc::new(Mutex::new(gst::MSECOND * 0)),
      layer: None
    }
  }

  pub fn set_layer(&mut self, layer: &Arc<Mutex<Layer>>) {
    self.layer = Some(Arc::downgrade(layer));
  }

  pub fn set_start(&mut self, val: gst::ClockTime) {
    println!("moving object to {}", val);
    let mut start = *self.start.lock().unwrap();
    start = val;

    match &self.content {
      ObjectContent::Clip { clip} => {
        clip.set_start(val);
      },
      ObjectContent::Text { clip } => {
        clip.set_start(val);
      },
      ObjectContent::Filter { clip } => {
        clip.set_start(val);
      },
      _ => {}
    }
  }
}

