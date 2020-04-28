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
    let menu = Menu::new();

    Self::append_submenu_media_object(&menu);

    menu
  }

  fn append_submenu_media_object(menu: &Menu) {
    let submenu = Menu::new();

    // // let open_video = MenuItem::new(Some("動画ファイル"), Some("app.timeline-open-video"));
    let open_video = MenuItem::new_with_label("動画ファイル");
    open_video.set_action_name(Some("app.timeline-open-video"));
    //
    // // submenu.append_item(&open_video);
    // submenu.append(&open_video);

    // // menu.append_submenu(Some("メディアオブジェクトの追加"), &submenu);
    // menu.set_submenu(Some("メディアオブジェクトの追加"), &submenu);

    // let media_obj_menuitem = MenuItem::new_with_label("メディアオブジェクトの追加");
    // media_obj_menuitem.set_submenu(Some(&submenu));
    //
    // menu.append(&media_obj_menuitem);
    menu.append(&open_video);
  }
}