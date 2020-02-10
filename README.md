## rust-clib 

An example project that uses Rust to create a library with C API.

Build rust lib and run C tests
```
(cd mylib && cargo build)
make test
```

successful output looks like this:
```
   Compiling add v0.2.0 (~/src/rust/clib/mylib)
    Finished dev [unoptimized + debuginfo] target(s) in 4.21s
cp mylib/target/debug/libmylib.a .
gcc  libmylib.a -o test test.o libmylib.a 
./test
PASSED
Tests run: 4
```



## Docs I found helpful

* Rust Embedded book: [A little Rust with your C](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
* https://svartalf.info/posts/2019-03-01-exposing-ffi-from-the-rust-library/
* Cargo book: [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

## The Rust part

in the `my` directory is a Rust crate, which has tests along with a very
simple example that calls a function from Rust (that is also callable from C)

```
cd mylib
cargo test                  # test that we can call the function in Rust
cargo run --example stdin   # interactive example
```

## references that might be helpful

* https://doc.rust-lang.org/nomicon/ffi.html
* https://karroffel.gitlab.io/post/2019-05-15-rust/
* https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/
* https://www.joshmatthews.net/blog/2015/10/creating-a-c-api-for-a-rust-library/
* https://www.reddit.com/r/rust/comments/6u7y3q/rust_wrappers_and_c_callbacks/
* https://users.rust-lang.org/t/stubbing-extern-c-functions-for-tests/15976/2


## Changelog

- v0.1: exporting a Rust function that can be called from C, generating C header ([blog post](https://www.ultrasaurus.com/2020/01/writing-c-library-in-rust/))
- v0.2: export struct, functions that create/destroy, call functions that
        accept struct with C callback function