extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use super::object::{ObjectContent, Object};
use super::layer::Layer;
use crate::util;

#[derive(Clone)]
pub struct Project {
  pub ges_timeline: ges::Timeline,
  pub layers: Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>,
  pub objects: Arc<Mutex<Vec<Arc<Mutex<Object>>>>>,
  pub ges_pipeline: ges::Pipeline,
}

impl Project {
  pub fn new() -> Self {
    let ges_timeline = ges::Timeline::new_audio_video();
    let ppl = ges::Pipeline::new();
    ppl.set_timeline(&ges_timeline).unwrap();

    let s = Self {
      ges_timeline,
      layers: Arc::new(Mutex::new(vec![])),
      objects: Arc::new(Mutex::new(vec![])),
      ges_pipeline: ppl,
    };

    s
  }

  pub fn add_layer(&mut self) -> Arc<Mutex<Layer>> {
    let name = util::random_name_for_layer();

    let ges_layer = self.ges_timeline.append_layer();
    let layer = Arc::new(Mutex::new(Layer::new(&name, ges_layer)));
    let layers = &mut *self.layers.lock().unwrap();
    let ret = layer.clone();
    &layers.push(layer);

    ret
  }

  // Layers
  pub fn get_layer(&self, layer_idx: usize) -> Arc<Mutex<Layer>> {
    let layers = &*self.layers.lock().unwrap();
    layers[layer_idx].clone()
  }

  pub fn find_layer_idx(&self, given: &Arc<Mutex<Layer>>) -> Option<usize> {
    let name = {
      let given = &*given.lock().unwrap();
      given.name.clone()
    };

    let name = &*name.lock().unwrap().clone();
    let layers = &*self.layers.lock().unwrap();
    for (i, layer) in layers.iter().enumerate() {
      let layer = &*layer.lock().unwrap();
      let lname = &*layer.name.lock().unwrap();
      if lname == name {
        return Some(i)
      }
    }

    None
  }

  // Objects
  pub fn add_object(&mut self, object: &Arc<Mutex<Object>>) {
    let obj_ = object.clone();
    let objs = &mut *self.objects.lock().unwrap();
    objs.push(obj_);
  }

  pub fn remove_object(&mut self, object: &Arc<Mutex<Object>>) {
    let object = &*object.lock().unwrap();
    let object_name = &*object.name.lock().unwrap();

    let objs = &mut *self.objects.lock().unwrap();
    objs.retain(|o| {
      let o = &*o.lock().unwrap();
      let o_name = &*o.name.lock().unwrap();
      object_name != o_name
    });
  }

  pub fn remove_object_by_name(&mut self, name: &str) {
    let objs = &mut *self.objects.lock().unwrap();
    println!("looping");
    objs.retain(|o| {
      let o = &*o.lock().unwrap();
      let o_name = &*o.name.lock().unwrap();
      name != o_name
    });
  }

  pub fn remove_object_by_index(&mut self, idx: usize) {
    let objs = &mut *self.objects.lock().unwrap();
    objs.remove(idx); // todo: using swap_remove? O(n)->O(1)
  }

  pub fn find_object_index(&self, given: &Arc<Mutex<Object>>) -> Option<usize> {
    let name = {
      let given = &*given.lock().unwrap();
      given.name.clone()
    };

    let name = &*name.lock().unwrap().clone();
    let objs = &*self.objects.lock().unwrap();
    for (i, obj) in objs.iter().enumerate() {
      let obj = &*obj.lock().unwrap();
      let obj_name = &*obj.name.lock().unwrap();
      if obj_name == name {
        return Some(i)
      }
    }

    None
  }

  // Layers + Objects
  // pub fn get_objects_of_layer(&self, layer: &Arc<Mutex<Layer>>) -> Vec<Arc<Mutex<Object>>> {
  //   let objs = *self.objects.lock().unwrap();
  //   objs.retain(|obj| {
  //
  //   })
  // }
  //
  // pub fn get_objects_of_layer_by_index(&self, idx: usize) -> Vec<Arc<Mutex<Object>>> {
  //   let layer = self.get_layer(idx);
  //   self.get_objects_of_layer(&layer)
  // }

  pub fn get_object_by_name(&self, name: &str) -> Option<Arc<Mutex<Object>>> {
    let objs = &*self.objects.lock().unwrap();

    for obj in objs {
      let ob = &*obj.lock().unwrap();
      let ob_name = &*ob.name.lock().unwrap();
      if ob_name == name {
        return Some(obj.clone())
      }
    }

    None
  }

  pub fn add_object_to_layer(&mut self, obj: &Arc<Mutex<Object>>, layer_idx: usize) {
    self.add_object(obj);

    let obj = &mut *obj.lock().unwrap();
    let layer = self.get_layer(layer_idx);
    let l = &*layer.lock().unwrap();

    obj.set_layer(&layer);
    match &obj.content {
      ObjectContent::Clip { clip } => {
        l.ges_layer.add_clip(clip).unwrap();
      },
      ObjectContent::Text { clip } => {
        l.ges_layer.add_clip(clip).unwrap();
      },
      ObjectContent::Filter { clip } => {
        l.ges_layer.add_clip(clip).unwrap();
      },
      _ => {}
    }
  }

  pub fn move_object_to_layer(&mut self, obj_name: &str, layer_idx: usize) {
    let obj = self.get_object_by_name(obj_name).unwrap();
    let obj = &mut *obj.lock().unwrap();

    let src = &obj.layer.as_ref().unwrap().upgrade().unwrap();

    let src_idx = self.find_layer_idx(&src).unwrap();
    if src_idx == layer_idx { return } // No need to move the layer

    let src = &*src.lock().unwrap();

    let dst_layer = self.get_layer(layer_idx);
    obj.set_layer(&dst_layer);

    match &obj.content {
      ObjectContent::Clip { clip } => {
        let dst = &*dst_layer.lock().unwrap();
        src.ges_layer.remove_clip(clip).unwrap();
        dst.ges_layer.add_clip(clip).unwrap();
      },
      ObjectContent::Text { clip } => {
        let dst = &*dst_layer.lock().unwrap();
        src.ges_layer.remove_clip(clip).unwrap();
        dst.ges_layer.add_clip(clip).unwrap();
      },
      ObjectContent::Filter { clip } => {
        let dst = &*dst_layer.lock().unwrap();
        src.ges_layer.remove_clip(clip).unwrap();
        dst.ges_layer.add_clip(clip).unwrap();
      },
      _ => {}
    }
  }
}