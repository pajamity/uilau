extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct LayerSelector {
  pub buttons: Vec<gtk::Button>,
}

impl LayerSelector {
  pub fn new(builder: &gtk::Builder) -> Self {
    let buttons: Vec<gtk::Button> = Vec::new();

    let boxx: gtk::Box = builder.get_object("timeline-layer-selector-box").unwrap();

    for i in 1..5 {
      let label = format!("Layer {}", i);
      let btn = gtk::Button::new_with_label(&label);

      btn.connect_clicked(move |_| {

      });

      boxx.pack_start(&btn, false, false, 0);
    }

    Self {
      buttons,
    }
  }
}