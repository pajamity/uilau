extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

use super::objectview::ObjectView;
use super::super::super::object::{Object, ObjectKind};

use std::collections::HashMap;
use gdk::Atom;

#[derive(Clone)]
pub struct LayersView {
  pub layout: gtk::Layout,
  pub layer_height: Arc<f64>,
  pub width_per_sec: Arc<Mutex<f64>>,
  pub objects: Arc<Mutex<HashMap<String, Object>>>,
  pub object_views: Arc<Mutex<HashMap<String, ObjectView>>>,
}

impl LayersView {
  pub fn new(builder: &gtk::Builder, width_per_sec: f64) -> Self {
    let layout: gtk::Layout = builder.get_object("timeline-layers").unwrap();
    
    let entries = gtk::TargetEntry::new("text/plain", gtk::TargetFlags::SAME_APP, 0);
    layout.drag_dest_set(gtk::DestDefaults::ALL, &[entries], gdk::DragAction::MOVE);

    // FIXME: sample objects for testing purpose
    let obj1 = Object::new("video1", "hogehoge", ObjectKind::Video, 20 * gst::SECOND, 10 * gst::SECOND, 0);
    let obj2 = Object::new("audio1", "piyopiyo", ObjectKind::Audio, 10 * gst::SECOND, 15 * gst::SECOND, 1);

    // s.add_object(0, 10 * gst::SECOND, "video1", "hogehoge",ObjectType::Video, 20 * gst::SECOND);
    // s.add_object(1, 15 * gst::SECOND, "audio1", "piyopiyo", ObjectType::Audio, 10 * gst::SECOND);

    let objects = HashMap::new();
    let object_views = HashMap::new();

    let mut s = Self {
      layout,
      layer_height: Arc::new(30.0),
      width_per_sec: Arc::new(Mutex::new(width_per_sec)),
      objects: Arc::new(Mutex::new(objects)),
      object_views: Arc::new(Mutex::new(object_views))
    };

    s.add_object(obj1);
    s.add_object(obj2);

    s.set_draw_handler();
    s.set_drop_handler();

    for widget in s.layout.get_children() {
      println!("Child: {:?}", widget);
    }

    s
  }

  pub fn add_object(&mut self, obj: Object) {
    {
      let wps = *self.width_per_sec.lock().unwrap();
      let len = *obj.length.lock().unwrap();
      let name = &*obj.name.lock().unwrap();
      let start = *obj.start.lock().unwrap();
      let layer_id = *obj.layer_id.lock().unwrap();
      let id = &*obj.id.lock().unwrap();

      let view = ObjectView::new(id, name, *obj.kind, (len.seconds().unwrap() as f64) * wps, *self.layer_height);
      self.layout.put(&view.drawing_area, (start.seconds().unwrap() as i32) * (wps as i32), (*self.layer_height * layer_id as f64) as i32);
      {
        let obj_views = self.object_views.clone();
        obj_views.lock().unwrap().insert(String::from(id), view);
      }
    }
    let id_ = obj.id.clone();
    let id = &*id_.lock().unwrap();
    self.objects.lock().unwrap().insert(String::from(id), obj);
  }

  // pub fn create_and_add_object(&self, layer_id: i32, start: gst::ClockTime, id: &str, name: &str, obj_type: ObjectType, length: gst::ClockTime, ) -> Object {
  //   let wps = *self.width_per_sec.lock().unwrap();
  //
  //   let view = ObjectView::new(id, name, obj_type, (length.seconds().unwrap() as f64) * wps, self.layer_height);
  //
  //   self.layout.put(&view.drawing_area, (start.seconds().unwrap() as i32) * (wps as i32), (self.layer_height * layer_id as f64) as i32);
  // }

  pub fn remove_object(&self) {

  }

  fn set_draw_handler(&self) {
    let layer_height = self.layer_height.clone();
    self.layout.connect_draw(move |layout, ctx| {

      // FIXME: needs to be flexible
      for i in 1..5 {
        // boundaries for layers
        ctx.set_source_rgb(0.2, 0.2, 0.2);
        ctx.set_line_width(1.0);
        ctx.move_to(0.0, *layer_height * (i as f64));
        ctx.line_to(1000.0, *layer_height * (i as f64)); // FIXME
        ctx.stroke();
      }

      Inhibit(false)
    });
  }

  fn set_drop_handler(&self) {
    let object_views_ = self.object_views.clone();
    let objects_ = self.objects.clone();
    // let wps_ = self.width_per_sec.clone();
    let layer_height = self.layer_height.clone();
    self.layout.connect_drag_data_received(move |layout, _ctx, x, y, data, _info, _time| {
      let object_views = &*object_views_.lock().unwrap();
      let objects = &*objects_.lock().unwrap();
      // let wps = *wps_.lock().unwrap();

      let id = &data.get_text().expect("No text attached to selection data");

      let layer_id = ((y as f64) / *layer_height).floor();
      // let layer_id = objects[id.as_str()].layer_id.lock().unwrap();
      layout.move_(&object_views[id.as_str()].drawing_area, x, (*layer_height * layer_id as f64) as i32);

      // update object's start etc
    });
  }
}