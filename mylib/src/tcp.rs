use std::ffi::c_void;


pub struct TcpConnection {
  callback: TcpStatusCallback,
  user_data: * mut c_void,
}

pub type TcpStatusCallback = extern "C" fn(code: i32, user_data: * mut c_void) -> ();


#[no_mangle]
pub extern "C" fn create_tcp(callback: TcpStatusCallback, user_data: * mut c_void) -> * mut TcpConnection {
  assert!(!(callback as *mut c_void).is_null());
  Box::into_raw(Box::new(TcpConnection { callback, user_data }))
}


#[no_mangle]
pub unsafe extern fn destroy_tcp(tcp_ptr: *mut TcpConnection) {
    if tcp_ptr.is_null() {
        return;
    }
    Box::from_raw(tcp_ptr);
}
