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
  pub max_duration: gst::ClockTime, // 0 for Effects, original duration for object w/ clips

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
      max_duration: duration,

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
      max_duration: duration,

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
      max_duration: duration,

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
      max_duration: gst::MSECOND * 0,

      start: Arc::new(Mutex::new(gst::MSECOND * 0)),
      layer: None
    }
  }

  pub fn set_layer(&mut self, layer: &Arc<Mutex<Layer>>) {
    self.layer = Some(Arc::downgrade(layer));
  }

  pub fn set_start(&mut self, val: gst::ClockTime) {
    *self.start.lock().unwrap() = val;

    match self.get_clip() {
      Some(clip) => {
        clip.set_start(val);
      }
      None => {}
    }
  }

  pub fn set_duration(&mut self, val: gst::ClockTime) {
    *self.duration.lock().unwrap() = val;

    match self.get_clip() {
      Some(clip) => {
        clip.set_duration(val);
      },
      None => {}
    }
  }

  // use this when calling common methods of GESClips.
  // todo: we can better with Generics?
  pub fn get_clip(&self) -> Option<&ges::Clip> {
    match &self.content {
      ObjectContent::Clip { clip } => Some(clip.upcast_ref::<ges::Clip>()),
      ObjectContent::Text { clip } => Some(clip.upcast_ref::<ges::Clip>()),
      ObjectContent::Filter { clip } => Some(clip.upcast_ref::<ges::Clip>()),
      _ => None
    }
  }

}

