extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};

// Making Box<Fn> property clonable.
// ref: https://users.rust-lang.org/t/can-i-make-a-boxed-fnmut-cloneable/17111/2
// ref: https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
// ref: https://stackoverflow.com/posts/30353928/revisions The first revision will be applied to the Box<Fn> below. It's simple. Be sure to use newer implementations when reusing impl like this.
// ??? probably I need deeper understanding of trait objects etc.

// dyn Fn(f64, f64, Vec<(f64, f64)>) + 'static
// 

// ClonableFn is a "subtrait" of BoxFnClone
// trait ClonableFn<F>: BoxFnClone<F> {
// }

// impl<F> Clone for Box<dyn ClonableFn<F>> {
//   fn clone(&self) -> Box<dyn ClonableFn<F>> {
//     // ClonableFnはBoxFnCloneのsubtraitだから、ClonableFnが自らclone_boxを定義していなくても使える
//     self.clone_box()
//   }
// }

// trait BoxFnClone<F> {
//   fn clone_box(&self) -> Box<dyn ClonableFn<F>>;
// }

// // what does "+ Clone +" mean here?
// impl<F, T: ClonableFn<F> + Clone + 'static> BoxFnClone<F> for T {
//   fn clone_box(&self) -> Box<dyn ClonableFn<F>> {
//     Box::new(self.clone())
//   }
// }

#[derive(Clone)]
pub struct ExtSlider {
  pub start: Arc<Mutex<f64>>,
  pub end: Arc<Mutex<f64>>,
  pub step: Arc<Mutex<f64>>,
  pub value: Arc<Mutex<Vec<(f64, f64)>>>, // (start, end) = a selection
  pub drawing_area: gtk::DrawingArea,
  // Arc+Mutex+& reference: don't know what lifetime to use. 'static might be good for this because we own an instance of UI until the termination of this program. And this can't be cloned.
  onchange_cb: Arc<Mutex<Option<&'static dyn Fn(f64, f64, Vec<(f64, f64)>)>>>,
  // boxed trait object (couldn't make this work):AccelLabelExt
  // onchange: Option<Box<dyn ClonableFn<Fn(f64, f64, Vec<(f64, f64)>)>>>,

  // sharing pointer(Box) using Arc and Mutex
  onchange: Arc<Mutex<Option<Box<dyn Fn(f64, f64, Vec<(f64, f64)>) + 'static>>>>,
  // onchange2: Arc<Mutex<dyn Fn(f64, f64, Vec<(f64, f64)>) + 'static>>

}

impl ExtSlider {
  pub fn new(start: f64, end: f64, step: f64, drawing_area: gtk::DrawingArea) -> Self {
    Self {
      start: Arc::new(Mutex::new(start)),
      end: Arc::new(Mutex::new(end)),
      step: Arc::new(Mutex::new(step)),
      value: Arc::new(Mutex::new(vec![(start, start)])),
      drawing_area,
      onchange_cb: Arc::new(Mutex::new(None)),
      onchange: Arc::new(Mutex::new(None))
    }
  }

  // ref: https://users.rust-lang.org/t/referencing-self-from-within-closure-in-a-method/19189/2
  pub fn set_handler(&self) {
    let start = Arc::clone(&self.start);
    let end = Arc::clone(&self.end);
    let value = Arc::clone(&self.value);
    self.drawing_area.connect_draw(move |area, ctx| {
      let start = *start.lock().unwrap();
      let end = *end.lock().unwrap();
      let value = value.lock().unwrap();
      let alloc = area.get_allocation();

      ctx.set_source_rgb(0.3, 0.3, 0.3);
      ctx.rectangle(0.0, 0.0, alloc.width as f64, alloc.height as f64);
      ctx.fill();

      // ref: https://users.rust-lang.org/t/referencing-self-from-within-closure-in-a-method/19189/2
      // "self" can't be used within closures in methods (not associated functions). so the procedure of get_position() was moved here
      // let (vstartx, vendx) = s.get_position(0);
      let (vstart, vend) = value[0];
      
      // 左端~vstart/vendが全体に占める割合 * スライダーの長さ
      let vstartx = (vstart - start) / (end - start) * (alloc.width as f64);
      let mut vendx = (vend - start) / (end - start) * (alloc.width as f64);

      if vstart == vend {
        vendx += 3.0; // make the line distinct. 
      }

      ctx.set_source_rgb(0.8, 0.8, 0.8);
      ctx.rectangle(vstartx, 0.0, vendx - vstartx, alloc.height as f64);
      ctx.fill();

      Inhibit(false)
    });

    let start = Arc::clone(&self.start);
    let end = Arc::clone(&self.end);
    let value = Arc::clone(&self.value);
    let onchange_cb = Arc::clone(&self.onchange_cb);
    let onchange = Arc::clone(&self.onchange);
    // Docs say get_coords() "Extracts the event surface relative x/y coordinates from an event". So do other get_*() methods.
    self.drawing_area.connect_button_press_event(move |area, event_button| {
      let start = *start.lock().unwrap();
      let end = *end.lock().unwrap();    
      let mut value = value.lock().unwrap();

      match event_button.get_button() {
        1 => {
          println!("left button")
        }
        3 => {
          println!("right button")
        }
        _ => return Inhibit(false)
      }

      match event_button.get_event_type() {
        gdk::EventType::ButtonPress => {
          let (x, y) = event_button.get_position();
          let alloc = area.get_allocation();
          println!("Clicked: {} {}", x, y);

          // get value from position
          // 左端での値 + 左端~クリック位置が全体の長さに占める割合
          println!("len = {} - {}  = {}; width = {}", end, start, end-start, alloc.width);
          let val = start + (end - start) * x / alloc.width as f64;
          *value = vec![(val, val)];

          // notify delegates
          let opt = &*onchange.lock().unwrap();
          match opt {
            Some(boxx) => {
              (*boxx)(x, val, vec![(val, val)]);
            }
            None => {}
          }

          match *onchange_cb.lock().unwrap() {
            Some(cb) => {
              // cb(x, *value); // causes error
              // FIXME: below won't work when it comes about dealing with two or more selections.
              cb(x, val, vec![(val, val)]);
            }
            None => {}
          }
        }
        _ => return Inhibit(false)
      }

      Inhibit(false)
    });
  }

  // Is 'static lifetime appropriate for this function? maybe. this callback is used in closure with 'static which GTK calls.
  // Fn(x, val, values) -> ()
  // where x: coordination of x-axis, val: value which x corresponds to, values: vec that represents all the selection
  pub fn on_change<F: Fn(f64, f64, Vec<(f64, f64)>) + 'static>(&self, cb: &'static F) {
    *self.onchange_cb.lock().unwrap() = Some(cb);
  }

  pub fn onchange(&self, cb: impl Fn(f64, f64, Vec<(f64, f64)>) + 'static) {
    *self.onchange.lock().unwrap() = Some(Box::new(cb));
  }


  // fn get_position(&self, idx: usize) -> (f64, f64) {
  //   let alloc = self.drawing_area.get_allocation();
  //   let (start, end) = (*self.start.lock().unwrap(), *self.end.lock().unwrap());
  //   let (vstart, vend) = self.value.lock().unwrap()[idx];
    
  //   // 左端~vstart/vendが全体に占める割合 * スライダーの長さ
  //   let vstartx = (vstart - start) / (end - start) * (alloc.width as f64);
  //   let vendx = (vend - start) / (end - start) * (alloc.width as f64);

  //   (vstartx, vendx)
  // }

  // WIP, should align to step
  // fn get_aligned_position(&self, idx: usize) -> (f64, f64) {

  //   (vstartx, vendx)
  // }

  pub fn add_value(&mut self) {

  }
}