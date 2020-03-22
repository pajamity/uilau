extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LayersView {

}

impl LayersView {
  pub fn new(builder: &gtk::Builder) -> Self {
    Self {
      
    }
  }
}