extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;

pub struct UI {
  window: gtk::ApplicationWindow,
}

impl UI {
  pub fn new(app: &gtk::Application) -> Self {
    let glade_src = include_str!("main.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("appwindow").unwrap();
    
    window.connect_delete_event(move |_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    let menu = Self::create_menu(); // associated function
    
    let w = window.clone();
    let m = menu.clone();
    app.connect_activate(move |app| {
      // activate後にセットしないとwindow, widgetがあるのにappが終了してしまう
      app.set_menubar(Some(&m));
      app.add_window(&w);
      
      let about_action = gio::SimpleAction::new("about", None);
      about_action.connect_activate(move |_, _| {
        println!("Hey");
        Self::create_about();
      });
      app.add_action(&about_action);

      w.show_all();
    });
    
    Self {
      window: window,
    }
  }

  fn create_menu() -> gio::Menu {
    use gio::{Menu, MenuItem};

    let menu = Menu::new();

    let submenu_misc = Menu::new();
    let about = MenuItem::new(Some("uilauについて"), Some("app.about"));
    submenu_misc.append_item(&about);

    menu.append_submenu(Some("その他"), &submenu_misc);

    menu
  }

  fn create_about() {
    let dialog = gtk::AboutDialog::new();
    dialog.set_title("uilau");
    dialog.set_comments(Some("WIP"));
    dialog.run();
    dialog.destroy();
  }
}