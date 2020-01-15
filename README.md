

Compile and run the C main with test (no rust for now)
```
gcc *.c -o app && ./app
```
## Docs I found helpful


* Rust Embedded book: [A little Rust with your C](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)

## The Rust part

in the `add` directory is a Rust crate

```
cd add
cargo test                  # test that we can call the function in Rust
cargo run --example stdin   # interactive example
```


## references that might be helpful

* https://doc.rust-lang.org/nomicon/ffi.html
* https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/
* https://www.joshmatthews.net/blog/2015/10/creating-a-c-api-for-a-rust-library/
* https://www.reddit.com/r/rust/comments/6u7y3q/rust_wrappers_and_c_callbacks/
* https://users.rust-lang.org/t/stubbing-extern-c-functions-for-tests/15976/2