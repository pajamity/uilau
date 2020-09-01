extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

extern crate libloading;
extern crate mlua;

use std::path::{Path, PathBuf};
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
  // fn new(&mut self, path: &str) -> Self;
  fn execute(&mut self, obj: &Arc<Mutex<Object>>);

  fn get_name(&self) -> String;
  fn get_kind(&self) -> PluginKind;
}

pub type BoxedPlugin = Box<dyn Plugin>;

pub struct LuaPlugin {
  pub name: String,
  pub path: PathBuf,
  pub source: String,
}

impl LuaPlugin {
  fn execute_lua(&mut self, obj: &Arc<Mutex<Object>>) -> mlua::Result<()> {
    let lua = mlua::Lua::new();
    let globals = lua.globals();

    let obj = &*obj.lock().unwrap();

    let obj_table = lua.create_table()?;


    // objtbl.set("x",

    // todo: writable properties such as ox

    // todo: methods

    // todo: other globals e.g. debug_print

    let name = self.name.clone();
    let debug_print = lua.create_function(move|_, (s): (String)| {
      println!("Debug Message from {}: {}", name, s);
      Ok(())
    })?;
    globals.set("debug_print", debug_print)?;

    // todo: consult mlua doc to find out which Rust type to represent Lua numbers
    let shift = lua.create_function(|_, (a, n): (i32, i32)| {
      Ok((a << n))
    })?;
    globals.set("SHIFT", shift);

    let or = lua.create_function(|_, (a,b): (i32, i32)| {
      Ok((a | b))
    })?;
    globals.set("OR", or);

    let and = lua.create_function(|_, (a,b): (i32, i32)| {
      Ok((a & b))
    })?;
    globals.set("AND", and);

    let xor = lua.create_function(|_, (a,b): (i32, i32)| {
      Ok((a ^ b))
    })?;
    globals.set("XOR", xor);

    let rgb = lua.create_function(|_, (r,g,b): (i32, i32, i32)| {
      let hex = r << 16 | g << 8 | b;
      Ok((hex))
    })?;
    globals.set("RGB", rgb);

    let irgb = lua.create_function(|_, (col): (i32)| {
      let r = (col & 0xff0000) >> 16;
      let g = (col & 0x00ff00) >> 8;
      let b = (col & 0x00ffff);
      Ok((r, g, b))
    })?;
    globals.set("RGB", irgb);

    lua.load(&self.source)
      .set_name(&self.name)?
      .exec()?;

    Ok(())
  }
}

impl Plugin for LuaPlugin {
  fn execute(&mut self, obj: &Arc<Mutex<Object>>) {
    match self.execute_lua(obj) {
      Ok(_) => {
        println!("lua script was executed successfully");
      },
      Err(err) => {
        println!("lua execution error: {}", err);
      }
    }
  }

  fn get_name(&self) -> String {
    self.name.clone()
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