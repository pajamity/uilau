extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate libloading;
extern crate mlua;

use std::path::Path;
use std::sync::{Arc, Mutex};

use super::object::{ObjectContent, Object};
use super::layer::Layer;
use crate::util;

pub enum PluginKind {
  Animation,
  Object,
  Scene,
  Camera,
  Filter,
  Input,
  Output,
  ColorSpace,
  // Note that we don't implement .aul. i18n should be done on Qt.
}

pub trait Plugin {
  fn new(&mut self, path: &str) -> Self;
  fn execute(&mut self, obj: &Arc<Mutex<Object>>);

  fn get_name(&self) -> String;
  fn get_kind(&self) -> PluginKind;
}

pub struct LuaPlugin {
  name: String,
  path: Path,
}

impl LuaPlugin {

}

impl Plugin for LuaPlugin {
  fn new(&mut self, path: Path) -> Self {
    // parse Lua

    // todo: make use of @

    let name = path.file_stem().unwrap().to_str().unwrap().to_string();


    Self {
      name,
      path,

    }
  }

  fn execute(&mut self, obj: &Arc<Mutex<Object>>) -> mlua::Result<()> {
    let lua = mlua::Lua::new();
    let globals = lua.globals();

    let obj = &*obj.lock().unwrap();

    let obj_table = lua.create_table()?;


    // objtbl.set("x",

    // todo: writable properties such as ox

    // todo: methods

    // todo: other globals e.g. debug_print

    // globals.set()

  }

  fn get_name(&self) -> String {

  }

  fn get_kind(&self) -> PluginKind {
    let ext = self.path.extension().unwrap().to_str().unwrap();
    match ext {
      ".anm" => PluginKind::Animation,
      ".obj" => PluginKind::Object,
      ".scn" => PluginKind::Scene,
      ".cam" => PluginKind::Camera,
      _ => panic!("Unsupported Extension")
    }
  }
}

// todo: is it possible to use DLLs on Linux?
pub struct DllPlugin {

}