extern crate gtk;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

use super::objectview::ObjectView;
use super::super::super::object::{Object, ObjectKind};
use super::super::super::layer::Layer;

use std::collections::HashMap;
use gdk::Atom;

#[derive(Clone)]
pub struct LayersView {
  pub layout: gtk::Layout,
  pub layer_height: Arc<f64>,
  pub width_per_sec: Arc<Mutex<f64>>,
  // pub objects: Arc<Mutex<HashMap<String, Object>>>,
  pub object_views: Arc<Mutex<HashMap<String, ObjectView>>>,
  pub layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>,
}

impl LayersView {
  pub fn new(builder: &gtk::Builder, width_per_sec: f64, layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>) -> Self {
    let layout: gtk::Layout = builder.get_object("timeline-layers").unwrap();
    
    let entries = gtk::TargetEntry::new("text/plain", gtk::TargetFlags::SAME_APP, 0);
    layout.drag_dest_set(gtk::DestDefaults::ALL, &[entries], gdk::DragAction::MOVE);

    let object_views = HashMap::new();

    let mut s = Self {
      layout,
      layer_height: Arc::new(30.0),
      width_per_sec: Arc::new(Mutex::new(width_per_sec)),
      object_views: Arc::new(Mutex::new(object_views)),
      layers,
    };

    // Add objects and create views
    let layers_ = s.layers.clone();
    let layers_ = &*layers_.lock().unwrap();
    for layer in layers_ {
      let layer_ = &*layer.lock().unwrap();
      let objs = &*layer_.objects.lock().unwrap();
      for (_id, obj) in objs.iter() {
        s.add_object(obj.clone());
      }
    }

    s.set_draw_handler();
    s.set_drop_handler();

    for widget in s.layout.get_children() {
      println!("Child: {:?}", widget);
    }

    s
  }

  pub fn add_object(&mut self, obj_: Arc<Mutex<Object>>) {
    let obj = &*obj_.lock().unwrap();

    let wps = *self.width_per_sec.lock().unwrap();
    let len = *obj.length.lock().unwrap();
    let name = &*obj.name.lock().unwrap();
    let start = *obj.start.lock().unwrap();
    let layer_id = *obj.layer_id.lock().unwrap();

    let view = ObjectView::new(obj_.clone(), &obj.id, name, obj.kind, (len.seconds().unwrap() as f64) * wps, *self.layer_height);
    self.layout.put(&view.drawing_area, (start.seconds().unwrap() as i32) * (wps as i32), (*self.layer_height * layer_id as f64) as i32);
    {
      let obj_views = self.object_views.clone();
      obj_views.lock().unwrap().insert(String::from(&obj.id), view);
    }
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

  pub fn set_layers(&mut self, layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>) {
    self.layers = layers;
  }

  fn set_draw_handler(&self) {
    let layers = self.layers.clone();
    let layer_height = self.layer_height.clone();
    self.layout.connect_draw(move |layout, ctx| {
      let layers = &*layers.lock().unwrap();

      // FIXME: needs to be flexible
      for i in 1..(layers.len()+1) {
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
    // let objects_ = self.objects.clone();
    let layers_ = self.layers.clone();
    let wps_ = self.width_per_sec.clone();
    let layer_height = self.layer_height.clone();
    self.layout.connect_drag_data_received(move |layout, _ctx, x, y, data, _info, _time| {

      let object_views = &*object_views_.lock().unwrap();
      // let objects = &*objects_.lock().unwrap();
      let wps = *wps_.lock().unwrap();
      let layers = &mut *layers_.lock().unwrap();

      let id = &data.get_text().expect("No text attached to selection data");
      println!("receiving...: {}", id);
      let layer_id = ((y as f64) / *layer_height).floor() as i32;

      // move to dest layer
      let mut obj_to_move = None;
      let mut src_layer = None;

      let layers___ = layers_.clone();
      let layers__ = &mut *layers___.lock().unwrap();
      let dest_layer = &mut *layers__[layer_id as usize].lock().unwrap();

      for layer in layers {
      // for layer in &*layers_.clone().lock().unwrap() {
        let layer_ = &mut *layer.lock().unwrap();
        let objs_ = &*layer_.objects.lock().unwrap();
        let obj = objs_.get(id.as_str());
        if let Some(obj) = obj { // if object with specific id is found
          obj_to_move = Some(obj.clone());
          src_layer = Some(layer.clone());
          break
        }
      }

      if let Some(obj_to_move) = obj_to_move {
        let obj_to_move_ = &*obj_to_move.lock().unwrap();
        src_layer.unwrap().lock().unwrap().remove_object(&obj_to_move_.id); // obtain mutable lock separately since the lock used for layer_ is immutable

        *obj_to_move_.start.lock().unwrap() = ((x as f64 / wps) * 1000.0) as u64 * gst::MSECOND;
        *obj_to_move_.layer_id.lock().unwrap() = layer_id;
        dest_layer.add_object(obj_to_move.clone());
      } else {
        eprintln!("Object to be moved is not found.");
      }

      // let layer_id = objects[id.as_str()].layer_id.lock().unwrap();
      layout.move_(&object_views[id.as_str()].drawing_area, x, (*layer_height * layer_id as f64) as i32);

      // update object's start etc
      // let obj = &objects[id.as_str()];
    });
  }
}