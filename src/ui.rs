extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;

use gio::{Menu, MenuItem};

use std::sync::{Arc, Mutex};

mod extslider;
use extslider::ExtSlider;

#[derive(Clone)]
pub struct UI {
  pub window: gtk::ApplicationWindow,
  pub menu: gio::Menu,
  pub video: gtk::DrawingArea,
  // Controls
  pub btn_playpause: gtk::Button,
  pub slider: gtk::Scale,
  pub sel_slider: ExtSlider
}

impl UI {
  pub fn new(app: &gtk::Application) -> Self {
    let glade_src = include_str!("main.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("appwindow").unwrap();
    window.set_title("uilau");
    window.set_default_size(480, 360);

    let menu = Self::create_menu(); // associated function
    
    let video: gtk::DrawingArea = builder.get_object("video").unwrap();

    // Controls
    let btn_playpause = builder.get_object("control-playpause").unwrap();
    let slider: gtk::Scale = builder.get_object("position-control").unwrap();

    let area = builder.get_object("test-scale").unwrap();
    let sel_slider = ExtSlider::new(0.0, 0.0, 0.0, area);
    sel_slider.set_handler();

    Self {
      window,
      menu,
      video,
      btn_playpause,
      slider,
      sel_slider,
    }
  }

  fn create_menu() -> gio::Menu {
    let menu = Menu::new();

    Self::append_submenu_misc(&menu);
    Self::append_submenu_file(&menu);

    menu
  }

  fn append_submenu_file(menu: &gio::Menu) {
    let submenu = Menu::new();
    
    let open_media = MenuItem::new(Some("開く…"), Some("app.open-media"));
    let quit = MenuItem::new(Some("終了"), Some("app.quit"));

    submenu.append_item(&open_media);
    submenu.append_item(&quit);

    menu.append_submenu(Some("ファイル"), &submenu);
  }

  fn append_submenu_misc(menu: &gio::Menu) {
    let submenu_misc = Menu::new();
    let about = MenuItem::new(Some("uilauについて"), Some("app.about"));
    submenu_misc.append_item(&about);

    menu.append_submenu(Some("その他"), &submenu_misc);
  }

  pub fn create_about(_: &gio::SimpleAction, _: Option<&glib::Variant>) {
    let dialog = gtk::AboutDialog::new();
    dialog.set_title("uilau");
    dialog.set_comments(Some("WIP"));
    dialog.run();
    dialog.destroy();
  }
  
  // ref: https://github.com/philn/glide/blob/master/src/ui_context.rs#L290
  pub fn file_chooser_dialog(&self) -> Option<glib::GString> {
    let dialog = gtk::FileChooserDialog::with_buttons(
      Some("ファイルを選択してください"),
      Some(&self.window),
      gtk::FileChooserAction::Open,
      &[("開く", gtk::ResponseType::Ok), ("キャンセル", gtk::ResponseType::Cancel)],
    );

    dialog.set_select_multiple(true);
    let result = match dialog.run() {
      gtk::ResponseType::Ok => {
        dialog.get_uri()
      }
      _ => None
    };
    dialog.destroy();
    
    result
  }

  pub fn refresh_slider(&self, duration: gst::ClockTime, position: gst::ClockTime) {
    let slider = &self.sel_slider;
    *slider.start.lock().unwrap() = 0.0;
    *slider.end.lock().unwrap() = duration.mseconds().unwrap() as f64;
    *slider.value.lock().unwrap() = vec![(position.mseconds().unwrap() as f64, position.mseconds().unwrap() as f64)];

    slider.drawing_area.queue_draw();
  }

}