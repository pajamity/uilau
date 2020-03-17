extern crate gtk;
use gtk::prelude::*;

extern crate gdk;
use gdk::prelude::*;

extern crate gio;
use gio::prelude::*;

use std::sync::{Arc, Mutex};


// #[derive(Clone)]
// pub struct ExtSlider {
//   pub start: f64,
//   pub end: f64,
//   pub step: f64,
//   pub value: Vec<(f64, f64)>, // (start, end) = a selection
//   pub drawing_area: gtk::DrawingArea,
// }

// impl ExtSlider {
//   pub fn new(start: f64, end: f64, step: f64, drawing_area: gtk::DrawingArea) -> Self {
//     Self {
//       start,
//       end,
//       step,
//       value: vec![(start, start)],
//       drawing_area
//     }
//   }

//   // ref: https://users.rust-lang.org/t/referencing-self-from-within-closure-in-a-method/19189/2
//   fn set_handler(&self) {
//     let s = Arc::new(self);
//     let s = Arc::downgrade(&s);

//     // drawing_areaをどこかに飛ばす？
//     self.drawing_area.connect_draw(move |area, ctx| {
//       let s = match s.upgrade() {
//         Some(x) => x,
//         None => return Inhibit(false)
//       };

//       let alloc = area.get_allocation();

//       ctx.set_source_rgb(0.3, 0.3, 0.3);
//       ctx.rectangle(0.0, 0.0, alloc.width as f64, alloc.height as f64);
//       ctx.fill();

//       let (vstartx, vendx) = s.get_position(0);
//       println!("{}, {}", vstartx, vendx);
//       println!("{} {} {} {}", s.start, s.end, s.value[0].0, s.value[0].1);

//       ctx.set_source_rgb(0.8, 0.8, 0.8);
//       ctx.rectangle(vstartx, 0.0, vendx - vstartx, alloc.height as f64);

//       Inhibit(false)
//     });
//   }

//   // ref: https://users.rust-lang.org/t/referencing-self-from-within-closure-in-a-method/19189/2
//   // "self" can't be used within closures in methods (not associated functions). 
//   fn create_handler() {
    
//   }

//   fn get_position(&self, idx: usize) -> (f64, f64) {
//     let alloc = self.drawing_area.get_allocation();
//     // let (start,)
//     let (vstart, vend) = self.value[idx];
    
//     // 左端~vstart/vendの全体に占める割合 * スライダーの長さ
//     let vstartx = (vstart - self.start) / (self.end - self.start) * (alloc.width as f64);
//     let vendx = (vend - self.start) / (self.end - self.start) * (alloc.width as f64);

//     (vstart, vend)
//   }

//   // WIP, should align to step
//   // fn get_aligned_position(&self, idx: usize) -> (f64, f64) {

//   //   (vstartx, vendx)
//   // }

//   pub fn add_value(&mut self) {

//   }
// }


#[derive(Clone)]
pub struct ExtSlider {
  pub start: Arc<Mutex<f64>>,
  pub end: Arc<Mutex<f64>>,
  pub step: Arc<Mutex<f64>>,
  pub value: Arc<Mutex<Vec<(f64, f64)>>>, // (start, end) = a selection
  pub drawing_area: gtk::DrawingArea,
}

impl ExtSlider {
  pub fn new(start: f64, end: f64, step: f64, drawing_area: gtk::DrawingArea) -> Self {
    Self {
      start: Arc::new(Mutex::new(start)),
      end: Arc::new(Mutex::new(end)),
      step: Arc::new(Mutex::new(step)),
      value: Arc::new(Mutex::new(vec![(start, start)])),
      drawing_area
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

      println!("{}, {}", vstartx, vendx);
      println!("{} {} {} {}", start, end, value[0].0, value[0].1);

      ctx.set_source_rgb(0.8, 0.8, 0.8);
      ctx.rectangle(vstartx, 0.0, vendx - vstartx, alloc.height as f64);
      ctx.fill();

      Inhibit(false)
    });
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