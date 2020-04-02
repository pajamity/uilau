extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;
use gio::{Menu, MenuItem};

#[derive(Clone)]
pub struct ContextMenu {
  pub menu: gtk::Menu,
}

impl ContextMenu {
  pub fn new() -> Self {
    let menu = create_menu();

    Self {
      menu,
    }
  }

  fn create_menu() -> Menu {
    let menu = Menu::new();

    append_submenu_media_object(&menu);

    menu
  }

  fn append_submenu_media_object(menu: &Menu) {
    let submenu = Menu::new();

    let open_video = MenuItem::new(Some("動画ファイル"), Some("app.timeline-open-video"));

    submenu.append_item(&open_video);

    menu.append_submenu(Some("メディアオブジェクトの追加"), &submenu);
  }
}