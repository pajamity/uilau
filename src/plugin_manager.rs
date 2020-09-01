extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate libloading;

extern crate regex;
use regex::Regex;

use std::io;
use std::fs::{self, DirEntry, File};
use std::sync::{Arc, Mutex};
use std::path::Path;

use super::object::{ObjectContent, Object};
use super::layer::Layer;
use crate::plugin::*;
use crate::util;
use std::io::{Error, ErrorKind, Read};

pub struct PluginManager {
  pub dir: String,
  pub plugins: Vec<BoxedPlugin>,
}

impl PluginManager {
  pub fn new(dir: &str) -> Self {
    Self {
      dir: dir.to_string(),
      plugins: vec![]
    }
  }

  // find available plugins
  pub fn find_plugins(&mut self) -> io::Result<()> {
    let dir = self.dir.clone();
    let path = Path::new(&dir);
    if path.is_dir() {
      self.search_dir(path);
      Ok(())
    } else {
      Err(Error::new(ErrorKind::Other, "Please specify a directory"))
    }
  }

  fn search_dir(&mut self, path: &Path) -> io::Result<()> {
    if path.is_dir() {
      for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
          self.search_dir(&path);
        } else {
          self.parse_file(&path);
        }
      }
    }
    Ok(())
  }

  fn parse_file(&mut self, path: &Path) -> io::Result<()> {
    // todo: parse comments
    // todo: dependencies etc... metafile

    let mut file = File::open(path)?;
    let mut source = String::new();
    file.read_to_string(&mut source)?;

    // todo: make use of @
    // let re =


    let name = path.file_stem().unwrap().to_str().unwrap().to_string();

    let plugin = LuaPlugin {
      name,
      path: path.to_path_buf(),
      source,
    };

    self.plugins.push(Box::new(plugin));

    Ok(())

  }

  // load a plugin
  pub fn load_plugin(&mut self) {

  }

  // 
}