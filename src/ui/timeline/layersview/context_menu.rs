extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;
// use gio::{Menu, MenuItem};
use gtk::{Menu, MenuItem};

#[derive(Clone)]
pub struct ContextMenu {
  pub menu: gtk::Menu,
}

impl ContextMenu {
  pub fn new() -> Self {
    let menu = Self::create_menu();

    Self {
      menu,
    }
  }

  fn create_menu() -> Menu {
    let xml = include_str!("../../xml/timeline/layersview/context_menu.xml");
    let builder = gtk::Builder::new_from_string(xml);
    let menu: gtk::Menu = builder.get_object("menu").unwrap();

    menu
  }
}