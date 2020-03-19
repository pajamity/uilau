extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

// Note that GtkLayout can be used in the similar way to GtkDrawingArea despite its name.

#[derive(Clone)]
pub struct TimeScale {
  // pub layout: gtk::Layout,
  pub layout: gtk::DrawingArea,
  pub start: Arc<Mutex<gst::ClockTime>>,
  pub end: Arc<Mutex<gst::ClockTime>>,
  pub width_per_sec: Arc<Mutex<u64>>,

  // drawn_start: Arc<Mutex<u64>>,
  pub drawn_end: Arc<Mutex<u64>>,
}

impl TimeScale {
  pub fn new(layout: gtk::DrawingArea, start: gst::ClockTime, end: gst::ClockTime, width_per_sec: u64) -> Self {
    
    let s = Self {
      layout,
      start: Arc::new(Mutex::new(start)),
      end: Arc::new(Mutex::new(end)),
      width_per_sec: Arc::new(Mutex::new(width_per_sec)),

      drawn_end: Arc::new(Mutex::new(0))
    };

    s.set_draw_handler();

    s
  }


  fn set_draw_handler(&self) {
    let start = self.start.clone();
    let end = self.end.clone();
    let width_per_sec = self.width_per_sec.clone();
    let drawn_end = self.drawn_end.clone();
    self.layout.connect_draw(move |layout, ctx| {
      let start = *start.lock().unwrap();
      let end = *end.lock().unwrap();
      let width_per_sec = *width_per_sec.lock().unwrap();
      let drawn_end = *drawn_end.lock().unwrap();
      // Assuming our request for height (40px) is adopted
      let alloc = layout.get_allocation();

      ctx.set_source_rgb(0.0, 0.0, 0.0);
      ctx.set_line_width(0.5);
      ctx.move_to(10.0, 30.0);
      ctx.line_to(alloc.width as f64 - 10.0, 30.0);

      ctx.stroke();

      Inhibit(false)
    });
  }

}