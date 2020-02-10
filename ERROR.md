

````
error[E0500]: closure requires unique access to `tcp` but it is already borrowed
  --> src/tcp.rs:69:40
   |
69 |     let handle = tcp.runtime.spawn(async {
   |  ________________-----------_-----_______^
   | |                |           |
   | |                |           first borrow later used by call
   | |                borrow occurs here
70 | |     let result = conn.await;
71 | |     let code = match result {
72 | |       Ok(_) => 200,
...  |
75 | |     tcp.trigger_callback(code);
   | |     --- second borrow occurs due to use of `tcp` in generator
76 | |   });
   | |___^ generator construction occurs here

error[E0597]: `tcp` does not live long enough
  --> src/tcp.rs:75:5
   |
69 |      let handle = tcp.runtime.spawn(async {
   |   __________________________________-_____-
   |  |__________________________________|
   | ||
70 | ||     let result = conn.await;
71 | ||     let code = match result {
72 | ||       Ok(_) => 200,
...  ||
75 | ||     tcp.trigger_callback(code);
   | ||     ^^^ borrowed value does not live long enough
76 | ||   });
   | ||   -
   | ||___|
   | |____value captured here by generator
   |      argument requires that `tcp` is borrowed for `'static`
77 | 
78 |    }
   |    - `tcp` dropped here while still borrowed

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0500, E0597.
For more information about an error, try `rustc --explain E0500`.
error: could not compile `mylib`.

```