# bvmod_rs_example

This repo is a quick snapshot of https://github.com/varnish-rs/varnish-rs/tree/main/examples/vmod_example slightly tweaked to be easy to clone and get going in case you need to create a new vmod in Rust.

## Requirements

You'll need:
- `cargo` (and the accompanying `rust` package)
- a recent `varnish` installed, with its development libraries/headers

## Build and test

``` bash
cargo build --release
cargo test --release
```

The vmod file will be found at `target/release/libbvmod_rs_example.so`.

## Making it your own

Rename the vmod:

``` bash
git grep -l bvmod_rs_example | xargs sed -i 's/rs_example/new_name/g'
```
