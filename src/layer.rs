extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

use std::sync::{Arc, Mutex};

use super::object::{ObjectKind, Object};

#[derive(Clone)]
pub struct Layer {
  pub objects: Arc<Mutex<Vec<Arc<Mutex<Object>>>>>,
}

impl Layer {
  pub fn new() -> Self {
    let s = Self {
      objects: Arc::new(Mutex::new(vec![])),
    };

    s
  }

  // pub fn
}