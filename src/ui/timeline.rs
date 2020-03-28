extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

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
  pub timescale: Arc<TimeScale>,
  pub view: LayersView,
  pub layer_sel: LayerSelector, 

  timeout_id: Arc<Mutex<u32>>,
}

impl Timeline {
  pub fn new(builder: &gtk::Builder) -> Self {
    let window: gtk::Window = builder.get_object("timeline").unwrap();
    let layers_window: gtk::ScrolledWindow = builder.get_object("timeline-layers-scroll").unwrap();
    let layers_window = Arc::new(layers_window);

    let wps = 10.0;

    // We wanna each widget to be independent of its parent
    // let layout: gtk::Layout = builder.get_object("timeline-timescale").unwrap();
    let layout: gtk::DrawingArea = builder.get_object("timeline-timescale").unwrap();

    let timescale = TimeScale::new(layout, 0 * gst::SECOND, 100 * gst::SECOND, wps);
    let timescale = Arc::new(timescale);
  
    let view = LayersView::new(&builder, wps);

    let layer_sel = LayerSelector::new(&builder);

    // FIXME: can disabling time-based redraw make this more efficient? (change to signal-based?)
    let layout = timescale.layout.clone();
    let layers_window_ = layers_window.clone();
    let timescale_ = timescale.clone();
    let id = gtk::timeout_add(100, move || {
      // sync position with LayersView
      let adj = layers_window_.get_hadjustment().unwrap().get_value();
      // println!("Offset: {} {} {}", adj, adj/wps, (adj / wps) as u64);
      timescale_.set_xoff(adj);
      // timescale_.set_xoff_time(((adj / wps * 1000.0) as u64) * gst::MSECOND);

      layout.queue_draw();
      Continue(true)
    });
    let timeout_id = Arc::new(Mutex::new(0));
    *timeout_id.lock().unwrap() = id.to_glib();

    let timeout_id_ = timeout_id.clone();
    timescale.layout.connect_delete_event(move |_, _| {
      match *timeout_id_.lock().unwrap() {
        0 => {}
        id => {
          glib::source_remove(glib::SourceId::from_glib(id))
        }
      }
      Inhibit(false)
    });


    let s = Self {
      window,
      layers_window,
      timescale,
      view,
      layer_sel,
      timeout_id,

    };


    s
  }

  pub fn show(&self) {
    self.window.show_all();
  }


}