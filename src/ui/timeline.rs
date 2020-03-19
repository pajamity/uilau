extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

mod timescale;
use timescale::TimeScale;

#[derive(Clone)]
pub struct Timeline {
  pub window: gtk::Window,
  pub timescale: TimeScale,
}

impl Timeline {
  pub fn new(builder: &gtk::Builder) -> Self {
    let window: gtk::Window = builder.get_object("timeline").unwrap();
    // We wanna each widget to be independent of its parent
    // let layout: gtk::Layout = builder.get_object("timeline-timescale").unwrap();
    let layout: gtk::DrawingArea = builder.get_object("timeline-timescale").unwrap();

    let timescale = TimeScale::new(layout, 0 * gst::SECOND, 100 * gst::SECOND, 10);
  
    let s = Self {
      window,
      timescale,

    };


    s
  }

  pub fn show(&self) {
    self.window.show_all();
  }


}