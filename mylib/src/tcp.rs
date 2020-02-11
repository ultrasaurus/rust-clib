use std::sync::Arc;
use std::ffi::c_void;
use std::ptr::NonNull;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use std::time::Duration;

pub struct TcpConnection {
  runtime: Runtime,
  conn: Option<TcpStream>,
  cb: Arc<CallbackInfo>,
}


pub type TcpStatusCallback = extern "C" fn(code: i32) -> * mut c_void;

#[derive(Copy, Clone)]
pub struct CallbackInfo {
  callback: TcpStatusCallback,
}
// this might be a bad idea, but I don't know any other option
// except to trust the caller to put locks around any mutation to this data
unsafe impl Send for CallbackInfo {}

impl CallbackInfo {
  pub fn call(&self, code: i32) {
    (self.callback)(code);
  }
}


#[no_mangle]
pub extern "C" fn create_tcp(callback: TcpStatusCallback) -> * mut TcpConnection 
  where TcpStatusCallback : 'static
{
  let runtime = Runtime::new().unwrap();
  let cb = Arc::new(CallbackInfo { callback });
  Box::into_raw(Box::new(TcpConnection { runtime, conn: None, cb }))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_connect_blocking(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return;   // should be an error, but just experimenting here
  }
  let tcp = &mut *tcp_ptr;

  let code = -1;
  let conn = TcpStream::connect("127.0.0.1:80");

  tcp.runtime.block_on(async {
    let result = conn.await;
    let code = match result {
      Ok(_) => 200,
      Err(_) => 404,
    };
  }); 
  //tcp.trigger_callback(code);
}


#[no_mangle]
pub unsafe extern "C" fn tcp_connect_async(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return;   // should be an error, but just experimenting here
  }
  let tcp = &mut *tcp_ptr;

  let conn = TcpStream::connect("127.0.0.1:80");

  let cb = &tcp.cb;
 
  let handle = tcp.runtime.spawn(async {
    let clone_cb = Arc::clone(&cb);

    let result = conn.await;
    let code = match result {
      Ok(_) => 200,
      Err(_) => 404,
    };
    clone_cb.call(code);
  });

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
