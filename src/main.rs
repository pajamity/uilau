extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

extern crate gstreamer_video as gst_video;
use gst_video::prelude::*;

extern crate gstreamer_editing_services as ges;
use ges::prelude::*;

use std::os::raw::c_void;
use std::process;
use std::sync::{Arc, Mutex};

mod ui;
use ui::UI;

mod player;
use player::{PlayInfo};

mod object;
use object::{Object, ObjectKind};

mod project;
use project::Project;

mod layer;
use layer::Layer;

#[derive(Clone)]
pub struct AppInfo {
  pub playinfo: Arc<Mutex<PlayInfo>>,
  pub ui: UI,
  // pub pipeline: gst::Element,
  pub pipeline: ges::Pipeline,
  pub timeout_id: Arc<Mutex<u32>>,
  // pub timeline: ges::Timeline,
}

pub fn run() {
  gtk::init().unwrap();
  gst::init().unwrap();
  ges::init().unwrap();

  let app = gtk::Application::new(Some("net.uilau"), Default::default()).expect("Failed to initialize GTK app");
  let proj = setup_sample_project();
  let playinfo = Arc::new(Mutex::new(PlayInfo {
    is_playing: false,
  }));
  let ui = UI::new(&app, &proj);


  let info = AppInfo {
    playinfo,
    ui,
    pipeline: proj.ges_pipeline,
    timeout_id: Arc::new(Mutex::new(0))
  };

  let info_ = info.clone();
  app.connect_activate(move |app| {
    let info = &info_;
    let (ui, pipeline) = (&info.ui, &info.pipeline);
    // apctivate後にセットしないとwindow, widgetがあるのにappが終了してしまう
    app.set_menubar(Some(&ui.menu));
    app.add_window(&ui.window);

    let about_action = gio::SimpleAction::new("about", None);
    about_action.connect_activate(UI::create_about);
    app.add_action(&about_action);

    let quit_action = gio::SimpleAction::new("quit", None);
    let a = app.clone();
    quit_action.connect_activate(move |_, _| {
      a.quit();
    });
    app.add_action(&quit_action);

    let open_media_action = gio::SimpleAction::new("open-media", None);
    let info_ = info.clone();
    open_media_action.connect_activate(move |_,_| {
      let info = &info_;
      open_media(info, &proj);
    });

    app.add_action(&open_media_action);

    let timeline_open_video_action = gio::SimpleAction::new("timeline-open-video", None);
    let info_ = info.clone();
    timeline_open_video_action.connect_activate(move |_, _| {
      let info = &info_;
      timeline_open_video(info);
    });
    app.add_action(&timeline_open_video_action);

    // Add handlers for video viewer
    let overlay = pipeline
      .clone()
      .dynamic_cast::<gst_video::VideoOverlay>()
      .unwrap();
    let pipeline_ = pipeline.clone();
    ui.video.connect_draw(move |window, ctx| {
      let pipeline = &pipeline_;
      // ElementExt::get_state return value example:
      //   (Ok(Success), Playing, VoidPending)
      //   (Ok(Success), Paused, VoidPending)
      // println!("{:?}", p.get_state(gst::SECOND * 3));
        
      match pipeline.get_state(gst::SECOND * 3) {
        (_, gst::State::Playing, _) => {},
        (_, gst::State::Paused, _) => {},
        _ => {
          let alloc = window.get_allocation();
          ctx.set_source_rgb(0.0, 0.0, 0.0);
          ctx.rectangle(0.0, 0.0, alloc.width as f64, alloc.height as f64);
          ctx.fill();
        }
      }

      Inhibit(false)
    });

    // ref: https://github.com/philn/glide/blob/e90432fa5718f6caa5885571f767318b4924559d/src/channel_player.rs#L159
    ui.video.connect_realize(move |v| {
      let overlay = &overlay;
      let gdk_window = v.get_window().unwrap();

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
            overlay.set_window_handle(xid as usize);
          }
        } else {
          println!("Add support for display type '{}'", display_type_name);
          process::exit(-1);
        }
      }
    });

    // Controls
    let playpause_action = gio::SimpleAction::new("playpause", None);
    let info_ = info.clone();
    playpause_action.connect_activate(move |_, _| {
      let (playinfo, ui, pipeline) = (&info_.playinfo, &info_.ui, &info_.pipeline);
      let mut playinfo = playinfo.lock().unwrap();
      if playinfo.is_playing {
        pipeline.set_state(gst::State::Paused).unwrap();
        let image = gtk::Image::new_from_icon_name(Some("media-playback-start"), gtk::IconSize::SmallToolbar);
        ui.btn_playpause.set_image(Some(&image));
      } else {
        pipeline.set_state(gst::State::Playing).unwrap();
        let image = gtk::Image::new_from_icon_name(Some("media-playback-pause"), gtk::IconSize::SmallToolbar);
        ui.btn_playpause.set_image(Some(&image));
      }
      playinfo.is_playing = !playinfo.is_playing;
    });
    app.add_action(&playpause_action);

    let info_ = info.clone();
    let slider_update_signal_id = ui.slider.connect_value_changed(move |slider| {
      let pipeline = &info_.pipeline;
      let value = slider.get_value() as u64;
      if pipeline
        .seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, value * gst::SECOND)
        .is_err() {
        eprintln!("Seeking failed");
      }
    });

    ui.slider.set_draw_value(false);
    let info_ = info.clone();
    let id = gtk::timeout_add(500, move || {
      let (ui, pipeline) = (&info_.ui, &info_.pipeline);

      if let Some(dur) = pipeline.query_duration::<gst::ClockTime>() {
        let seconds = dur / gst::SECOND;
        // why
        ui.slider.set_range(0.0, seconds.map(|v| v as f64).unwrap_or(0.0));
      }

      if let Some(pos) = pipeline.query_position::<gst::ClockTime>() {
        let seconds = pos / gst::SECOND;
        ui.slider.block_signal(&slider_update_signal_id);
        ui.slider.set_value(seconds.map(|v| v as f64).unwrap_or(0.0));
        ui.slider.unblock_signal(&slider_update_signal_id);
      }

      if let (Some(dur), Some(pos))
        = (pipeline.query_duration::<gst::ClockTime>(), 
           pipeline.query_position::<gst::ClockTime>()) {
        ui.refresh_slider(dur, pos);
      } 

      Continue(true)
    });
    let mut timeout_id = info.timeout_id.lock().unwrap();
    *timeout_id = id.to_glib();

    let info_ = info.clone();
    ui.sel_slider.onchange(move |_, val, _| {    
      let pipeline = &info_.pipeline;
      if pipeline
        .seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT, (val as u64) * gst::MSECOND)
        .is_err() {
        eprintln!("Seeking failed");
      }
    });

    ui.window.show_all();
  });

  // info.pipeline
  //   .set_state(gst::State::Playing)
  //   .expect("Unable to set state");
  // info.playinfo.lock().unwrap().is_playing = true;
  // let image = gtk::Image::new_from_icon_name(Some("media-playback-pause"), gtk::IconSize::SmallToolbar);
  // info.ui.btn_playpause.set_image(Some(&image));
  
  info.pipeline
    .set_state(gst::State::Paused)
    .expect("Unable to set state");
  info.playinfo.lock().unwrap().is_playing = false;
  let image = gtk::Image::new_from_icon_name(Some("media-playback-play"), gtk::IconSize::SmallToolbar);
  info.ui.btn_playpause.set_image(Some(&image));


  let info_ = info.clone();
  info.ui.window.connect_delete_event(move |_, _| {
    gtk::main_quit();
    let timeout_id = info_.timeout_id.lock().unwrap();
    match *timeout_id {
      0 => {}
      id => {
        glib::source_remove(glib::SourceId::from_glib(id))
      }
    }
    Inhibit(false)
  });

  app.run(&[]);
}

fn create_sample_clip() -> ges::UriClip {
  let uri = "file:///usr/share/big-buck-bunny_trailer.webm";
  let clip = ges::UriClip::new(uri).expect("failed to create clip.");
  clip
}

fn setup_sample_project() -> Project {
  let mut proj = Project::new();
  let layer_ = proj.add_layer();
  let layer = &mut *layer_.lock().unwrap();

  let clip = create_sample_clip();
  let mut obj = Object::new_from_uri_clip("v1", "bigbunny", 10 * gst::SECOND, 0, clip);
  obj.set_layer(layer_.clone());
  layer.add_object(Arc::new(Mutex::new(obj)));

  proj.add_layer();

  // let effect = ges::Effect::new("agingtv").expect("Failed to create effect");
  // clip.add(&effect).unwrap();

  // let asset = clip.get_asset().unwrap();
  // let duration = asset
  //   .downcast::<ges::UriClipAsset>()
  //   .unwrap()
  //   .get_duration();
  //
  // clip.set_inpoint(duration / 2);
  // clip.set_duration(duration / 4);

  proj
}

fn open_media(info: &AppInfo, proj: &mut Project) {
  match info.ui.file_chooser_dialog() {
    Some(uri) => {
      println!("Opening {}", uri);
      info.pipeline.set_state(gst::State::Null).unwrap();
      info.pipeline
        .set_property("uri", &uri)
        .expect("Could not open uri");
      info.pipeline.set_state(gst::State::Playing).unwrap();
    }
    None => return
  }
}

fn timeline_open_video(info: &AppInfo) {
  match info.ui.file_chooser_dialog() {
    Some(uri) => {
      println!("Opening {}", uri);
      info.pipeline.set_state(gst::State::Null).unwrap();
      info.pipeline
        .set_property("uri", &uri)
        .expect("Could not open uri");
      info.pipeline.set_state(gst::State::Playing).unwrap();
    }
    None => return
  }
}

mod tutorial5 {
  extern crate glib;
  use self::glib::*;

  extern crate gdk;

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

    let stop_button = gtk::Button::new_from_icon_name(
      Some("media-playback-stop"),
      gtk::IconSize::SmallToolbar,
    );
    
    let controls = Box::new(Orientation::Horizontal, 0);
    controls.pack_start(&stop_button, false, false, 0);

    let video_window = DrawingArea::new();


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
      timeout_id: None
    }
  }

  fn post_app_message(playbin: &gst::Element) {
    let mbuilder = gst::Message::new_application(gst::Structure::new_empty("tags-changed"));
    let _ = playbin.post_message(&mbuilder.build());
  }

  pub fn run() {
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
