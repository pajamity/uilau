extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

mod timescale;
use timescale::TimeScale;

mod layersview;
use layersview::LayersView;

mod layer_selector;
use layer_selector::LayerSelector;

mod objectview;
use objectview::ObjectView;

#[derive(Clone)]
pub struct Timeline {
  pub window: gtk::Window,
  pub layers_window: Arc<gtk::ScrolledWindow>,
  pub timescale: TimeScale,
  pub view: LayersView,
  pub layer_sel: LayerSelector, 
}

impl Timeline {
  pub fn new(builder: &gtk::Builder) -> Self {
    let window: gtk::Window = builder.get_object("timeline").unwrap();
    let layers_window: gtk::ScrolledWindow = builder.get_object("timeline-layers-scroll").unwrap();
    let layers_window = Arc::new(layers_window);

    // We wanna each widget to be independent of its parent
    // let layout: gtk::Layout = builder.get_object("timeline-timescale").unwrap();
    let layout: gtk::DrawingArea = builder.get_object("timeline-timescale").unwrap();

    let timescale = TimeScale::new(layout, 0 * gst::SECOND, 100 * gst::SECOND, 10.0, layers_window.clone());
  
    let view = LayersView::new(&builder, 10.0);

    let layer_sel = LayerSelector::new(&builder);

    let s = Self {
      window,
      layers_window,
      timescale,
      view,
      layer_sel,
    };


    s
  }

  pub fn show(&self) {
    self.window.show_all();
  }


}