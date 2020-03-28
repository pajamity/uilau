extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

use std::sync::{Arc, Mutex};

// Note that GtkLayout can be used in the similar way to GtkDrawingArea despite its name.

#[derive(Clone)]
pub struct TimeScale {
  // pub layout: gtk::Layout,
  pub layout: Arc<gtk::DrawingArea>,
  pub start: Arc<Mutex<gst::ClockTime>>,
  pub end: Arc<Mutex<gst::ClockTime>>,
  pub width_per_sec: Arc<Mutex<f64>>,

  // drawn_start: Arc<Mutex<u64>>,
  pub drawn_end: Arc<Mutex<f64>>,
  pub xoff: Arc<Mutex<f64>>, // = horizontal adjustment
}

impl TimeScale {
  pub fn new(layout: gtk::DrawingArea, start: gst::ClockTime, end: gst::ClockTime, width_per_sec: f64,) -> Self {
    
    let s = Self {
      layout: Arc::new(layout),
      start: Arc::new(Mutex::new(start)),
      end: Arc::new(Mutex::new(end)),
      width_per_sec: Arc::new(Mutex::new(width_per_sec)),

      // layers_window,

      drawn_end: Arc::new(Mutex::new(0.0)),
      xoff: Arc::new(Mutex::new(0.0)),
    };

    s.set_draw_handler();

    s
  }


  fn set_draw_handler(&self) {
    let start = self.start.clone();
    let end = self.end.clone();
    let width_per_sec = self.width_per_sec.clone();
    let drawn_end = self.drawn_end.clone();
    let xoff = self.xoff.clone();
    // let layers_window = self.layers_window.clone();
    self.layout.connect_draw(move |layout, ctx| {
      let start = *start.lock().unwrap();
      let end = *end.lock().unwrap();
      let width_per_sec = *width_per_sec.lock().unwrap();
      let drawn_end = *drawn_end.lock().unwrap();
      let xoff = *xoff.lock().unwrap();
      // Assuming our request for height (40px) is adopted
      let alloc = layout.get_allocation();

      // bottom line
      ctx.set_source_rgb(0.0, 0.0, 0.0);
      ctx.set_line_width(1.0);
      ctx.move_to(10.0, 30.0);
      ctx.line_to(alloc.width as f64 - 10.0, 30.0);
      ctx.stroke();

      let mut k = 0;

      // 1   2   3   4   5   6(end)
      // |   |   |   |   |   |     
      // -----------------------
      //       ^xoff         
      // then display:
      //         3   4   5   6(end) 
      //         |   |   |   |     
      //       -----------------------
      // <---> ^xoff   
      //  offset

      let mut pos = xoff % width_per_sec + 0.0; // 10px = offset
      while pos < (alloc.width as f64 - 10.0) { // -10.0 is for right margin
        match k % 5 {
          0 => ctx.move_to(pos, 10.0),
          _ => ctx.move_to(pos, 20.0)
        }
        ctx.line_to(pos, 30.0);
        ctx.stroke();

        pos += width_per_sec;
        k += 1;

        // position to time
        let time = (((xoff + pos) / width_per_sec) as u64) * gst::SECOND;

        // TODO: deal with start
      
        if time > end { break }
      }

      Inhibit(false)
    });
  }

  // pos = relative coordination within widget (x-axis)
  // pub fn position_to_time(&self, pos: f64) -> gst::ClockTime {
  //   ((*self.xoff.lock().unwrap() + pos) / *self.width_per_sec.lock().unwrap()) * gst::SECOND
  // }

  pub fn set_start(&self, val: gst::ClockTime) {
    *self.start.lock().unwrap() = val;
  }

  pub fn set_xoff_time(&self, time: gst::ClockTime) {
    let xoff = (time.seconds().unwrap() as f64) * *self.width_per_sec.lock().unwrap();
    println!("XOFF: {}", xoff);
    *self.xoff.lock().unwrap() = xoff;
  }

  pub fn set_xoff(&self, xoff: f64) {
    *self.xoff.lock().unwrap() = xoff;
  }

}