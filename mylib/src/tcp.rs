use std::ffi::c_void;
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use std::time::Duration;
use tokio::task;

pub struct TcpConnection {
  runtime: Runtime,
  handle: Option<task::JoinHandle<()>>,
  conn: Option<TcpStream>,
  cb: CallbackInfo,
}

pub type TcpStatusCallback = extern "C" fn(code: i32, user_data: * const c_void) -> ();

#[derive(Copy, Clone)]
pub struct CallbackInfo {
  callback: TcpStatusCallback,
  user_data: * const c_void,
}
// this might be a bad idea, but I don't know any other option
// except to trust the caller to put locks around any mutation to this data
unsafe impl Send for CallbackInfo {}

impl CallbackInfo {
  pub fn call(&self, code: i32) {
    (self.callback)(code, self.user_data);
  }
}


#[no_mangle]
pub extern "C" fn create_tcp(callback: TcpStatusCallback, user_data: * const c_void) -> * mut TcpConnection
  where TcpStatusCallback : 'static
{
  let runtime = Runtime::new().unwrap();
  let cb = CallbackInfo { callback, user_data };
  Box::into_raw(Box::new(TcpConnection { runtime,
                                         handle: None,
                                         conn: None, cb }))
}

#[no_mangle]
pub unsafe extern "C" fn tcp_connect_blocking(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return;   // should be an error, but just experimenting here
  }
  let tcp = &mut *tcp_ptr;

  let conn = TcpStream::connect("127.0.0.1:80");
  let cb = tcp.cb;

  tcp.runtime.block_on(async {
    let result = conn.await;
    let code = match result {
      Ok(_) => 200,
      Err(_) => 404,
    };
    cb.call(code);
  });
}


#[no_mangle]
pub unsafe extern "C" fn tcp_connect_async(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return ()
  }
  let tcp = &mut *tcp_ptr;
  let conn = TcpStream::connect("127.0.0.1:80");
  let cb = tcp.cb;

  let handle = tcp.runtime.spawn(async move {

    let result = conn.await;
    let code = match result {
      Ok(_cn) => {
        // tcp.conn = Some(cn);
        200
      },
      Err(_) => 404,
    };
    cb.call(code);
  });
  tcp.handle = Some(handle);
}

pub unsafe extern "C" fn tcp_connect_wait(tcp_ptr: *mut TcpConnection) -> () {
  if tcp_ptr.is_null() {
    return ()
  }
  let tcp = &mut *tcp_ptr;
  if let Some(h) = tcp.handle {
    tcp.runtime.block_on(h);
  }
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
