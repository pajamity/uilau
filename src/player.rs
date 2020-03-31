extern crate gstreamer as gst;
use gst::prelude::*;

use std::sync::{Arc, Mutex};

use crate::ui::{UI};

#[derive(Clone)]
pub struct AppInfo {
  pub playinfo: Arc<Mutex<PlayInfo>>,
  pub ui: UI,
  pub pipeline: gst::Element
}

#[derive(Clone)]
pub struct PlayInfo {
  pub is_playing: bool,
}

pub fn setup_player() {

}

pub fn toggle_playpause(info: &AppInfo) {

}