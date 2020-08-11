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
  pub duration: Arc<Mutex<gst::ClockTime>>,

  // move to "objectPlacement" ?
  pub start: Arc<Mutex<gst::ClockTime>>,

  pub layer: Option<Weak<Mutex<Layer>>>,
}

impl Object {
  pub fn new(name: &str, content: ObjectContent, duration: gst::ClockTime, start: gst::ClockTime) -> Self {
    let s = Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content,
      duration: Arc::new(Mutex::new(duration)),

      start: Arc::new(Mutex::new(start)),
      layer: None,
    };

    s
  }

  pub fn new_from_uri_clip(name: &str, clip: ges::UriClip) -> Self {
    let asset = clip.get_asset().unwrap();
    let duration = asset
      .downcast::<ges::UriClipAsset>()
      .unwrap()
      .get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Clip { clip },
      duration: Arc::new(Mutex::new(duration)),

      start: Arc::new(Mutex::new(gst::SECOND * 0)),
      layer: None
    }
  }

  pub fn new_from_title_clip(name: &str, clip: ges::TitleClip) -> Self {
    let duration = clip.get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Text { clip },
      duration: Arc::new(Mutex::new(duration)),

      start: Arc::new(Mutex::new(gst::MSECOND * 0)),
      layer: None
    }
  }

  pub fn new_from_effect_clip(name: &str, clip: ges::EffectClip) -> Self {
    let duration = clip.get_duration();

    Self {
      name: Arc::new(Mutex::new(name.to_string())),
      content: ObjectContent::Filter { clip },
      duration: Arc::new(Mutex::new(duration)),

      start: Arc::new(Mutex::new(gst::MSECOND * 0)),
      layer: None
    }
  }

  pub fn set_layer(&mut self, layer: &Arc<Mutex<Layer>>) {
    self.layer = Some(Arc::downgrade(layer));
  }

  pub fn set_start(&mut self, val: gst::ClockTime) {
    *self.start.lock().unwrap() = val;

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

  pub fn set_duration(&mut self, val: gst::ClockTime) {
    *self.duration.lock().unwrap() = val;

    match &self.content {
      ObjectContent::Clip { clip} => {
        clip.set_duration(val);
      },
      ObjectContent::Text { clip } => {
        clip.set_duration(val);
      },
      ObjectContent::Filter { clip } => {
        clip.set_duration(val);
      },
      _ => {}
    }
  }
}

