extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum ObjectType {
  Video,
  Audio,
  Text,
  Shape,
  Filter
}

#[derive(Clone)]
pub struct ObjectView {
  pub drawing_area: gtk::DrawingArea,
  pub name: Arc<Mutex<String>>,
  pub obj_type: Arc<ObjectType>,
  pub width: Arc<Mutex<f64>>,
  pub height: Arc<Mutex<f64>>,
}

impl ObjectView {
  pub fn new(name: &str, obj_type: ObjectType, width: f64, height: f64) -> Self {
    let drawing_area = gtk::DrawingAreaBuilder::new()
      .height_request(height as i32)
      .width_request(width as i32)
      .build();

    let s = Self {
      drawing_area,
      name: Arc::new(Mutex::new(name.to_string())),
      obj_type: Arc::new(obj_type),
      width: Arc::new(Mutex::new(width)),
      height: Arc::new(Mutex::new(height)),
    };

    let name = s.name.clone();
    let obj_type = s.obj_type.clone();
    let width = s.width.clone();
    let height = s.height.clone();
    s.drawing_area.connect_draw(move |area, ctx| {
      let name = &*name.lock().unwrap(); // why String can't be copied?
      let width = *width.lock().unwrap();
      let height = *height.lock().unwrap();
      let alloc = area.get_allocation();
      // println!("alloc: ({}, {})", alloc.width, alloc.height);

      // Fill background
      // todo: gradation
      let background = match *obj_type {
        ObjectType::Video => (0.0, 0.3, 1.0),
        _ => (0.0, 0.0, 0.0)
      };

      ctx.set_source_rgb(background.0, background.1, background.2);
      ctx.rectangle(0.0, 0.0, width, height);
      ctx.fill();

      // Put on the text
      ctx.set_source_rgb(1.0, 1.0, 1.0);
      // ctx.select_font_face("")
      ctx.set_font_size(13.0);

      ctx.move_to(5.0, height / 2.0);
      ctx.show_text(&name);

      Inhibit(false)
    });

    s
  }
}