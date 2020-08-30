extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate libloading;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use super::object::{ObjectContent, Object};
use super::layer::Layer;
use crate::util;

pub struct PluginManager {
  pub plugins: Arc<Mutex<Vec<Arc<Mutex<Plugin>>>>>,
}

impl PluginManager {
  // find available plugins

  // load a plugin

  // 
}