use crate::Thing as Thing;

#[no_mangle]
pub extern "C" fn create_thing(num: i32) -> * mut Thing {
    Box::into_raw(Box::new(Thing::new(num)))
}

#[no_mangle]
pub unsafe extern "C" fn thing_num(thing_ptr: *mut Thing) -> i32 {
  assert!(!thing_ptr.is_null());
  let thing = &*thing_ptr;

  thing.num()
}

#[no_mangle]
pub unsafe extern fn destroy_thing(thing_ptr: *mut Thing) {
    if thing_ptr.is_null() {
        return;
    }

    Box::from_raw(thing_ptr);
}
