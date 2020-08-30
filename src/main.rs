extern crate libc;
extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

extern crate glib;

mod qt_impl;
mod interface;
mod ffi;
mod project;
mod object;
mod layer;
mod util;
mod plugin;
mod plugin_manager;

use gst::prelude::*;
use ges::prelude::*;

use qt_impl::*;
use interface::*;
use object::{Object, ObjectContent};
use layer::Layer;

// functions in main.cpp
extern {
  fn main_cpp(app: *const ::std::os::raw::c_char) -> *const usize;
}

fn main() {
  // Load GStreamer plugin for qmlglsink beforehand
  gst::init().unwrap();
  let _ = gst::ElementFactory::make("qmlglsink", Some("qmlglsink"));

  // Call Qt via FFI
  use std::ffi::CString;
  let app_name = ::std::env::args().next().unwrap();
  let app_name = CString::new(app_name).unwrap();
  
  unsafe {
    main_cpp(app_name.as_ptr());
  }
}