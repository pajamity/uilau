extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

use std::os::raw::c_void;
use std::process;
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