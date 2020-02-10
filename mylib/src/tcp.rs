use std::ffi::c_void;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use std::time::Duration;

pub struct TcpConnection {
  runtime: Runtime,
  conn: Option<TcpStream>,
  callback: TcpStatusCallback,
  user_data: * mut c_void,
}

pub type TcpStatusCallback = extern "C" fn(code: i32, user_data: * mut c_void) -> ();

impl TcpConnection {
  pub fn trigger_callback(&self, code: i32) {
    (self.callback)(code, self.user_data);
  }
}

#[no_mangle]
pub extern "C" fn create_tcp(callback: TcpStatusCallback, user_data: * mut c_void) -> * mut TcpConnection {
  assert!(!(callback as *mut c_void).is_null());
  let runtime = Runtime::new().unwrap();

  Box::into_raw(Box::new(TcpConnection { runtime, conn: None, callback, user_data }))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_connect_blocking(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return;   // should be an error, but just experimenting here
  }
  let tcp = &mut *tcp_ptr;

  let mut code = -1;
  let conn = TcpStream::connect("127.0.0.1:80");

  tcp.runtime.block_on(async {
    let result = conn.await;
    code = match result {
      Ok(_) => 200,
      Err(_) => 404,
    };
  });
  tcp.trigger_callback(code);

}


#[no_mangle]
pub unsafe extern fn destroy_tcp(tcp_ptr: *mut TcpConnection) {
    if tcp_ptr.is_null() {
        return;
    }
    let _tcp = Box::from_raw(tcp_ptr);
    // attempt to shutdown gracefully, but this API must have changed
    // tcp.runtime.shutdown_timeout(Duration::from_millis(100));
    //                ^^^^^^^^^^^^^^^^ method not found in `tokio::runtime::Runtime`
    

    // memory associated with `tcp` that was allocated by Rust freed on fn exit
}
