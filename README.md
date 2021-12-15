# vmod_rs_template

This repo is a quick snapshot of https://github.com/gquintard/varnish-rs/tree/main/examples/vmod_example slightly tweaked to be easy to clone and get going in case you need to create a new vmod in Rust.

## Requirements

You'll need:
- `cargo` (and the accompanying `rust` package)
- `python3`
- the `varnish` 7.0.1 development libraries/headers ([depends on the `varnish` crate you are using](https://github.com/gquintard/varnish-rs#versions))

## Build and test

``` bash
cargo build --release
cargo test --release
```

The vmod file will be found at `target/release/libvmod_rs_template.so`.

## Making it your own

Rename the vmod:

``` bash
git grep -l rs_template | xargs sed -i 's/rs_template/new_name/g'
```

## Packages

To avoid making a mess of your system, you probably should install your vmod as a proper package. This repository also offers different templates, and some quick recipes for different distributions.

### Arch

``` bash
# if you have `jq` installed, we can retrieve the vmod version from `Cargo.toml`
VMOD_VERSION=$(cargo metadata --no-deps --format-version 1 | jq '.packages[0].version' -r)
# this depends on the `varnish` crate version we depend on (https://github.com/gquintard/varnish-rs#versions)
VARNISH_VERSION=7.0.1

# create a tarball from the last commit
git archive --output=vmod_rs_template-$VMOD_VERSION.tar.gz --format=tar.gz HEAD

# create a work directory, and add the tarball, and the PKGBUILD with some variable substituted
mkdir build
mv vmod_rs_template-$VMOD_VERSION.tar.gz build
sed -e 's/@VMOD_VERSION@/0.0.1/' -e 's/@VARNISH_VERSION@/7.0.1/' pkg/arch/PKGBUILD > build/PKGBUILD

# build!
cd build
makepkg -rsf

# your package will be the file with the `.pkg.tar.zst` extension in `build/`
```
