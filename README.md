

Compile and run the C main with test (no rust for now)
```
gcc *.c -o app && ./app
```


## The Rust part

in the `add` directory is a Rust crate

```
cd add
cargo test                  # test that we can call the function in Rust
cargo run --example stdin   # interactive example
```


## references that might be helpful

* https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/