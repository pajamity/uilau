extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

use super::objectview::ObjectView;
use super::objectview::ObjectType;

#[derive(Clone)]
pub struct LayersView {
  pub layout: gtk::Layout,
  pub layer_height: f64,
  pub width_per_sec: Arc<Mutex<f64>>,
}

impl LayersView {
  pub fn new(builder: &gtk::Builder, width_per_sec: f64) -> Self {
    let layout = builder.get_object("timeline-layers").unwrap();
    
    let s = Self {
      layout,
      layer_height: 30.0,
      width_per_sec: Arc::new(Mutex::new(width_per_sec)),
    };

    s.add_object(0, 30 * gst::SECOND, "hogehoge",ObjectType::Video);

    for widget in s.layout.get_children() {
      println!("Child: {:?}", widget);
    }

    s
  }

  pub fn add_object(&self, layer_id: i32, time: gst::ClockTime, name: &str, obj_type: ObjectType, ) {
    let view = ObjectView::new(name, obj_type, 100.0, self.layer_height);

    // let xpos = time
    let xpos = 30;
    self.layout.put(&view.drawing_area, 0, 0);

    // self.layout.put(&obj.drawing_area, xpos, (self.layer_height * layer_id as f64) as i32);
  }

  pub fn remove_object(&self) {

  }
}