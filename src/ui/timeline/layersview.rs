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
    for (layer_id, layer) in layers_.iter().enumerate() {
      let layer_ = &*layer.lock().unwrap();
      let objs = &*layer_.objects.lock().unwrap();
      for (_id, obj) in objs.iter() {
        s.add_object(obj.clone(), layer_id);
      }
    }

    s.set_draw_handler();
    s.set_drop_handler();

    for widget in s.layout.get_children() {
      println!("Child: {:?}", widget);
    }

    s
  }

  pub fn add_object(&mut self, obj_: Arc<Mutex<Object>>, layer_id: usize) {
    let obj = &*obj_.lock().unwrap();

    let wps = *self.width_per_sec.lock().unwrap();
    let len = *obj.length.lock().unwrap();
    let name = &*obj.name.lock().unwrap();
    let start = *obj.start.lock().unwrap();

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
    let layers_ = self.layers.clone();
    let wps_ = self.width_per_sec.clone();
    let layer_height = self.layer_height.clone();
    self.layout.connect_drag_data_received(move |layout, _ctx, x, y, data, _info, _time| {
      let object_views = &*object_views_.lock().unwrap();
      let wps = *wps_.lock().unwrap();

      let id = &data.get_text().expect("No text attached to selection data");
      println!("receiving...: {}", id);
      let layer_id = ((y as f64) / *layer_height).floor() as i32;

      let layers = &mut *layers_.lock().unwrap();

      let obj_to_move = &object_views[id.as_str()].object;
      let obj_to_move_arc = match obj_to_move.upgrade() {
        Some(o) => o,
        None => panic!("No object found"),
      };

      {
        let obj_to_move_arc_ = obj_to_move_arc.clone();
        let ptr = obj_to_move_arc_.clone();
        let obj_to_move = &mut *obj_to_move_arc_.lock().unwrap();
        obj_to_move.set_layer(layers[layer_id as usize].clone());

        if let Some(src_layer) = &obj_to_move.layer {
          match src_layer.upgrade() {
            Some(s) => {
              let src_layer = &mut *s.lock().unwrap();
              // can't lock the source layer if the object was moved in the same layer
              if let Ok(ref mut dest_layer) = layers[layer_id as usize].try_lock() {
                // move object between layers
                src_layer.remove_object(obj_to_move);
                dest_layer.add_object(ptr);
              }
            }
            None => panic!("No layer was found")
          }
        }
      }

      let obj_to_move_arc_ = obj_to_move_arc.clone();
      let obj_to_move = &mut *obj_to_move_arc_.lock().unwrap();
      obj_to_move.set_start(((x as f64 / wps) * 1000.0) as u64 * gst::MSECOND);

      layout.move_(&object_views[id.as_str()].drawing_area, x, (*layer_height * layer_id as f64) as i32);
    });

    let layer_height = self.layer_height.clone();
    let layers = self.layers.clone();
    self.layout.connect_drag_motion(move |_layout, _ctx, x, y, _time| {
      let layers = &*layers.lock().unwrap();

      let is_droppable = (layers.len() as f64) * *layer_height >= y as f64;
      println!("Droppable: {}", is_droppable);
      Inhibit(!is_droppable) // FIXME: drop area restriction using drag_motion handler does not work
    });
  }
}

// todo: ドラッグしても最初から再生されていない? timelineの