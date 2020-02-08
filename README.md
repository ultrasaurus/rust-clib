## rust-clib 

Build rust lib and run tests from C code
```
(cd mylib && cargo build)
gcc main.c mylib/target/debug/libmylib.a -o app && ./app
```

successful output looks like this:
```
   Compiling add v0.1.0 (~/src/rust/clib/add)
    Finished dev [unoptimized + debuginfo] target(s) in 4.21s
Saving session...
...saving history...truncating history files...
...completed.
$ gcc main.c add/target/debug/libadd.a -o app && ./app
PASSED
Tests run: 2
```



## Docs I found helpful

* Rust Embedded book: [A little Rust with your C](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
* https://svartalf.info/posts/2019-03-01-exposing-ffi-from-the-rust-library/
* Cargo book: [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)

## The Rust part

in the `add` directory is a Rust crate

```
cd add
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
