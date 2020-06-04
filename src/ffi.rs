extern crate gstreamer_sys;
extern crate gobject_sys;

// I couldn't avoid using global state...
// lazy_static (static Mutex) can't be used as *const usize "cannot be sent between threads safely" according to the compiler
pub static mut VIDEO_ITEM: *const usize = 0 as *const usize;

#[no_mangle]
pub unsafe fn set_video_item_pointer(video_item: *const usize) {
  VIDEO_ITEM = video_item;
  println!("Pointer for videoItem: {:?}", video_item);
}