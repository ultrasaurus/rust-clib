

Compile and run the C main with test (no rust for now)
```
gcc *.c -o app && ./app
```

Run pure Rust example
```
cd add
cargo run --example stdin
```


## How I made this:

```
cargo new add --lib
...
```


## references that might be helpful

* https://www.greyblake.com/blog/2017-08-10-exposing-rust-library-to-c/