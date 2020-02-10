use std::ffi::c_void;
use crate::{StatusCallback, Thing, ThingStatus};

pub struct ExtThingStatus {
  callback: StatusCallbackFunction,
  userdata: * mut c_void,
}

impl StatusCallback for ExtThingStatus {
  fn call(&self, status: ThingStatus) -> () {
    (self.callback)(status.code, self.userdata);
  }
}
// pub type status_callback_t = extern "C" fn(code: i32, user_data: * mut c_void) -> ();

pub type StatusCallbackFunction = extern "C" fn(code: i32, user_data: * mut c_void) -> ();

#[no_mangle]
pub extern "C" fn create_thing(num: i32, callback_or_null: StatusCallbackFunction, userdata: * mut c_void) -> * mut Thing<ExtThingStatus> {
  let mut callback_info = None;
  if !(callback_or_null as *mut c_void).is_null() {
    let callback = callback_or_null;
    callback_info = Some(ExtThingStatus {
      callback,
      userdata
    });
  }

  Box::into_raw(Box::new(Thing::new(num, callback_info)))
}

#[no_mangle]
pub unsafe extern "C" fn thing_num(thing_ptr: *mut Thing<ExtThingStatus>) -> i32 {
  assert!(!thing_ptr.is_null());
  let thing = &*thing_ptr;

  thing.num()
}

#[no_mangle]
pub unsafe extern "C" fn thing_something(thing_ptr: *mut Thing<ExtThingStatus>) -> () {
  assert!(!thing_ptr.is_null());
  let thing = &*thing_ptr;

  thing.something()
}

#[no_mangle]
pub unsafe extern fn destroy_thing(thing_ptr: *mut Thing<ExtThingStatus>) {
    if thing_ptr.is_null() {
        return;
    }

    Box::from_raw(thing_ptr);
}
