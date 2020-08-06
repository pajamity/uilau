extern crate gstreamer as gst;
extern crate gstreamer_editing_services as ges;

use gst::prelude::*;
use ges::prelude::*;

use std::sync::{Arc, Mutex, Weak};

use crate::interface::*;
use crate::project::*;

use crate::layer::Layer;

pub struct Layers {
  emit: LayersEmitter,
  model: LayersList,
  
  layers: Option<Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>>,
  project: Option<Weak<Mutex<Project>>>,
}

impl Layers {
  pub fn set_layers(&mut self, layers: &Arc<Mutex<Vec<Arc<Mutex<Layer>>>>>) {
    self.layers = Some(layers.clone());
  }

  pub fn set_project(&mut self, project: &Arc<Mutex<Project>>) {
    self.project = Some(Arc::downgrade(project));
  }
}

impl LayersTrait for Layers {
  fn new(emit: LayersEmitter, model: LayersList) -> Self {
    Self {
      emit,
      model,
      layers: None,
      project: None,
    }
  }

  fn emit(&mut self) -> &mut LayersEmitter {
    &mut self.emit
  }

  fn row_count(&self) -> usize {
    if let Some(layers) = &self.layers {
      let layers = &*layers.lock().unwrap();
      layers.len()
    } else {
      0
    }
  }
}