extern crate gtk;
use gtk::prelude::*;

extern crate gio;
use gio::prelude::*;

mod ui;
use ui::UI;

pub fn run() {
  gtk::init().unwrap();

  let app = gtk::Application::new(Some("net.uilau"), Default::default()).expect("Failed to initialize GTK app");

  UI::new(&app);

  // app.connect_activate(|app| {
  //   let window = ApplicationWindow::new(app);
  //   window.set_title("uilau");
  //   window.set_default_size(480, 360);

  //   let button = Button::new_with_label("Click me");
  //   button.connect_clicked(|_| {
  //     println!("wow");
  //   });

  //   let button2 = Button::new_with_label("Click me");
  //   button.connect_clicked(|_| {
  //     println!("wow");
  //   });

  //   window.add(&button);
  //   window.add(&button2);
  //   window.show_all();
  // });

  app.run(&[]);
}

mod tutorial5 {
  use std::os::raw::c_void;
  use std::process;

  extern crate glib;
  use self::glib::object::ObjectType;
  use self::glib::*;

  extern crate gdk;
  use self::gdk::prelude::*;

  extern crate gtk;
  use self::gtk::*;

  extern crate gstreamer as gst;
  extern crate gstreamer_video as gst_video;
  use self::gst_video::prelude::*;

  use std::ops;

  struct AppWindow {
    main_window: Window,
    timeout_id: Option<glib::SourceId>,
  }

  impl ops::Deref for AppWindow {
    type Target = Window;

    fn deref(&self) -> &Window {
      &self.main_window
    }
  }

  impl Drop for AppWindow {
    fn drop(&mut self) {
      if let Some(source_id) = self.timeout_id.take() {
        glib::source_remove(source_id);
      }
    }
  }

  fn add_streams_info(playbin: &gst::Element, textbuf: &gtk::TextBuffer, stype: &str) {
    let propname: &str = &format!("n-{}", stype);
    let signame: &str = &format!("get-{}-tags", stype);

    match playbin.get_property(propname).unwrap().get() {
      Ok(Some(x)) => {
        for i in 0..x {
          let tags = playbin.emit(signame, &[&i]).unwrap();

          if let Ok(Some(tags)) = tags.unwrap().get::<gst::TagList>() {
            textbuf.insert_at_cursor(&format!("{} stream {}:\n ", stype, 1));

            if let Some(codec) = tags.get::<gst::tags::VideoCodec>() {
              textbuf.insert_at_cursor(&format!("   codec: {}\n", codec.get().unwrap()));
            }

            if let Some(codec) = tags.get::<gst::tags::AudioCodec>() {
              textbuf.insert_at_cursor(&format!("   codec: {}\n", codec.get().unwrap()));
            }

            if let Some(lang) = tags.get::<gst::tags::LanguageCode>() {
              textbuf.insert_at_cursor(&format!("   lang: {}\n", lang.get().unwrap()));
            }
          }
        }
      }
      _ => {
        eprintln!("Could not get {}", propname);
      }
    }
  }

  fn analyze_streams(playbin: &gst::Element, textbuf: &gtk::TextBuffer) {
    textbuf.set_text("");

    add_streams_info(playbin, textbuf, "video");
    add_streams_info(playbin, textbuf, "audio");
    add_streams_info(playbin, textbuf, "text");
  }

  fn create_ui(playbin: &gst::Element) -> AppWindow {
    let main_window = Window::new(WindowType::Toplevel);
    main_window.connect_delete_event(|_, _| {
      gtk::main_quit();
      Inhibit(false)
    });

    let play_button = gtk::Button::new_from_icon_name(
      Some("media-playback-start"),
      gtk::IconSize::SmallToolbar,
    );
    let pipeline = playbin.clone();
    play_button.connect_clicked(move |_| {
      let pipeline = &pipeline;
      pipeline
        .set_state(gst::State::Playing)
        .expect("Could not set state");
    });

    let pause_button = gtk::Button::new_from_icon_name(
      Some("media-playback-pause"),
      gtk::IconSize::SmallToolbar,
    );
    let pipeline = playbin.clone();
    pause_button.connect_clicked(move |_| {
      let pipeline = &pipeline;
      pipeline
        .set_state(gst::State::Paused)
        .expect("Could not set state"); 
    });

    let stop_button = gtk::Button::new_from_icon_name(
      Some("media-playback-stop"),
      gtk::IconSize::SmallToolbar,
    );
    let pipeline = playbin.clone();
    pause_button.connect_clicked(move |_| {
      let pipeline = &pipeline;
      pipeline
        .set_state(gst::State::Paused)
        .expect("Could not set state"); 
    });
    
    let slider = gtk::Scale::new_with_range(
      gtk::Orientation::Horizontal,
      0.0 as f64, 100.0 as f64, 1.0 as f64
    );
    let pipeline = playbin.clone();
    let slider_update_signal_id = slider.connect_value_changed(move |slider| {
      let pipeline = &pipeline;
      let value = slider.get_value() as u64;
      if pipeline
        .seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, value * gst::SECOND)
        .is_err() {
        eprintln!("Seeking failed");
      }
    });

    slider.set_draw_value(false);
    let pipeline = playbin.clone();
    let lslider = slider.clone();
    let timeout_id = gtk::timeout_add_seconds(1, move || {
      let pipeline = &pipeline;
      let lslider = &lslider;

      // what is if let some?
      if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
        let seconds = dur / gst::SECOND;
        // why
        lslider.set_range(0.0, seconds.map(|v| v as f64).unwrap_or(0.0));
      }

      if let Some(pos) = pipeline.query_position::<gst::ClockTime>() {
        let seconds = pos / gst::SECOND;
        lslider.block_signal(&slider_update_signal_id);
        lslider.set_value(seconds.map(|v| v as f64).unwrap_or(0.0));
        lslider.unblock_signal(&slider_update_signal_id);
      }

      Continue(true)
    });

    let controls = Box::new(Orientation::Horizontal, 0);
    controls.pack_start(&play_button, false, false, 0);
    controls.pack_start(&pause_button, false, false, 0);
    controls.pack_start(&stop_button, false, false, 0);
    controls.pack_start(&slider, true, true, 2);

    let video_window = DrawingArea::new();

    let video_overlay = playbin
      .clone()
      // why ?
      .dynamic_cast::<gst_video::VideoOverlay>()
      .unwrap();

    video_window.connect_realize(move |video_window| {
      let video_overlay = &video_overlay;
      let gdk_window = video_window.get_window().unwrap();

      if !gdk_window.ensure_native() {
        println!("Can't create nativewindow");
        process::exit(-1);
      }

      let display_type_name = gdk_window.get_display().get_type().name();
      {
        // Check if we're using X11 or ...
        if display_type_name == "GdkX11Display" {
          extern "C" {
              pub fn gdk_x11_window_get_xid(
                  window: *mut glib::object::GObject,
              ) -> *mut c_void;
          }

            #[allow(clippy::cast_ptr_alignment)]
            unsafe {
                let xid = gdk_x11_window_get_xid(gdk_window.as_ptr() as *mut _);
                video_overlay.set_window_handle(xid as usize);
            }
        } else {
            println!("Add support for display type '{}'", display_type_name);
            process::exit(-1);
        }

      }
    });

    video_window.connect_draw(|window, ctx| {
      let alloc = window.get_allocation();
      ctx.set_source_rgb(0.0, 0.0, 0.0);
      ctx.rectangle(0.0, 0.0, alloc.width as f64, alloc.height as f64);
      ctx.fill();
      Inhibit(false)
    });

    let streams_list = gtk::TextView::new();
    streams_list.set_editable(false);
    let pipeline_weak = playbin.downgrade();
    // ?
    let streams_list_weak = glib::SendWeakRef::from(streams_list.downgrade());
    let bus = playbin.get_bus().unwrap();

    #[allow(clippy::single_match)]
    bus.connect_message(move |_, msg| match msg.view () {
      gst::MessageView::Application(application) => {
        let pipeline = match pipeline_weak.upgrade() {
          Some(p) => p,
          None => return
        };

        let streams_list = match streams_list_weak.upgrade() {
          Some(s) => s,
          None => return
        };

        if application.get_structure().map(|s| s.get_name()) == Some("tags-changed") {
          let textbuf = streams_list
            .get_buffer()
            .expect("Could nto get buffer");
          analyze_streams(&pipeline, &textbuf);
        }
      }
      _ => ()
    });

    let vbox = Box::new(Orientation::Horizontal, 0);
    vbox.pack_start(&video_window, true, true, 0);
    vbox.pack_start(&streams_list, false, false, 2);

    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.pack_start(&vbox, true, true, 0);
    main_box.pack_start(&controls, false, false, 0);
    main_window.add(&main_box);
    main_window.set_default_size(640, 480);

    main_window.show_all();

    AppWindow {
      main_window,
      timeout_id: Some(timeout_id)
    }
  }

  fn post_app_message(playbin: &gst::Element) {
    let mbuilder = gst::Message::new_application(gst::Structure::new_empty("tags-changed"));
    let _ = playbin.post_message(&mbuilder.build());
  }

  pub fn run() {
    #[allow(clippy::eq_op)]
    {
      // if !cfg!(feature = "tutorial5-x11") && !cfg!(feature = "tutorial5-quartz") {
      //   eprintln!("No GDK backend selected");
      //   return;
      // }
    }

    if let Err(err) = gtk::init() {
      eprintln!("Failed to initialize GTK: {}", err);
      return;
    }

    if let Err(err) = gst::init() {
      eprintln!("Failed to initialize Gst: {}", err);
      return;
    }

   
    let playbin = gst::ElementFactory::make("playbin", Some("playbin"))
    .expect("couldn't create playbin");

    let uri = "file:///";
    playbin
      .set_property("uri", &uri)
      .expect("Could set uri"); 
    
    playbin
      .connect("video-tags-changed", false, |args| {
        let pipeline = args[0]
          .get::<gst::Element>()
          .expect("Playbin args[0]")
          .unwrap();
        post_app_message(&pipeline);
        None
      })
      .expect("Failed to create connection");

    playbin
      .connect("audio-tags-changed", false, |args| {
        let pipeline = args[0]
          .get::<gst::Element>()
          .expect("Playbin args[0]")
          .unwrap();
        post_app_message(&pipeline);
        None
      })
      .expect("Failed to create connection");
      
    playbin
      .connect("text-tags-changed", false, |args| {
        let pipeline = args[0]
          .get::<gst::Element>()
          .expect("Playbin args[0]")
          .unwrap();
        post_app_message(&pipeline);
        None
      })
      .expect("Failed to create connection");
      
    let window = create_ui(&playbin);
    let bus = playbin.get_bus().unwrap();
    bus.add_signal_watch();

    let pipeline_weak = playbin.downgrade();
    bus.connect_message(move |_, msg| {
      let pipeline = match pipeline_weak.upgrade() {
        Some(p) => p,
        None => return
      };

      match msg.view() {
        gst::MessageView::Eos(..) => {
          pipeline
            // .set_state(gst::State::Ready) // this leads to panic, why?
            .set_state(gst::State::Paused)
            .expect("unable to set state");
        }
        gst::MessageView::Error(err) => {
          eprintln!(
            "Error received from element {:?}: {}",
            err.get_src().map(|s| s.get_path_string()),
            err.get_error()
          );
          eprintln!("Debugging information: {:?}", err.get_debug());
        }
        gst::MessageView::StateChanged(state_changed) => {
          if state_changed.get_src().map(|s| s == pipeline).unwrap_or(false) {
            println!("State set to {:?}", state_changed.get_current());
          }
        }
        _ => ()
      }
    });

    // playbin
    //   .set_state(gst::State::Playing)
    //   .expect("Failed to set state");

    gtk::main();
    window.hide();
    playbin
      .set_state(gst::State::Null)
      .expect("Failed to set state");

    bus.remove_signal_watch();
  }
}

fn main() {
  run();
}
