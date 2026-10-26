# Installation guide

MIRAI is a static analyzer for Rust. It uses private compiler libraries, so it
must be installed and run with the Rust nightly named in
[`rust-toolchain.toml`](../rust-toolchain.toml).

## Dependencies

Install Rust with [rustup](https://rustup.rs/). You also need a C++ compiler and
Clang with its libclang shared library. On Linux, install `pkg-config` and the
OpenSSL development package if they are not already present.

## Installation

Install MIRAI from a checkout so rustup can use the repository's toolchain file:

```bash
git clone --branch main https://github.com/huitseeker/mirai-endorlabs.git
cd mirai-endorlabs

MIRAI_TOOLCHAIN="$(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml)"
rustup toolchain install "$MIRAI_TOOLCHAIN" --profile minimal \
  --component rustc-dev,rust-src,llvm-tools-preview
cargo +"$MIRAI_TOOLCHAIN" install --locked --path checker \
  --target-dir target/install
```

The target directory keeps the build files in the checkout. MIRAI takes several
gigabytes to build, and a temporary build can fill a small `/tmp` filesystem.

Cargo installs `cargo-mirai` and `mirai` in `$CARGO_HOME/bin`. The default path
is `~/.cargo/bin`, which must be on `PATH`.

## Running MIRAI

Use the same nightly when you run MIRAI:

```bash
cargo +"$MIRAI_TOOLCHAIN" mirai --version
cd /path/to/rust/project
cargo +"$MIRAI_TOOLCHAIN" mirai
```

The Rust project must compile with that nightly. If you start a new shell, set
`MIRAI_TOOLCHAIN` from the checkout again or replace it with the channel from
`rust-toolchain.toml`.

Running `cargo mirai` through another toolchain may fail because the loader
cannot find `librustc_driver`. The explicit `+"$MIRAI_TOOLCHAIN"` selector gives
the installed analyzer the compiler libraries it needs.

## Contributing

See the [developer guide](DeveloperGuide.md) for build and debugging instructions.
