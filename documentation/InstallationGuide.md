# Installation guide

MIRAI is a Rust static analyzer that runs through `cargo mirai`. This fork uses
`nightly-2026-10-09`. Use that nightly both to install MIRAI and to run it.

## Dependencies

Install Rust through [rustup](https://rustup.rs/). MIRAI uses private compiler
libraries, so an ordinary stable Rust installation is insufficient. Install the
pinned compiler and its components:

```bash
rustup toolchain install nightly-2026-10-09 --profile minimal \
  --component rustc-dev,rust-src,llvm-tools-preview
```

Keep this toolchain installed after building MIRAI. The installed analyzer still
loads its compiler libraries at runtime. The pin is recorded in
[`rust-toolchain.toml`](../rust-toolchain.toml). Use its version when updating
these commands for a later checkout.

You also need a native C++ linker and Clang with its libclang shared library.
On Linux, the download dependencies use pkg-config and OpenSSL development
headers. Having the `clang` executable alone does not guarantee that bindgen can
find libclang.

The default `bundled` feature downloads and statically links Z3 4.15.2 release
binaries from GitHub. This installation path does not build Z3 with CMake.
The Linux binaries require glibc 2.39 on x86-64 or glibc 2.34 on ARM64.

## Installation from a checkout

Clone this fork and enter its directory so rustup can read the toolchain pin:

```bash
git clone --branch main https://github.com/huitseeker/mirai-endorlabs.git
cd mirai-endorlabs
cargo install --locked --path ./checker --target-dir target/install
```

`--locked` uses the repository's dependency versions. `--target-dir` keeps build
files in the checkout so you can resume an interrupted build. It also avoids
Cargo's temporary build directory, which can exhaust a small `/tmp` filesystem.
Allow several gigabytes of free space for the build.

Cargo installs `cargo-mirai` and `mirai` into `$CARGO_HOME/bin`, or `~/.cargo/bin`
when `CARGO_HOME` is unset. Make sure that directory is on `PATH`.

## Installation directly from Git

You can install without keeping a source checkout. Select the nightly explicitly
because a Git dependency's `rust-toolchain.toml` does not select the compiler for
the Cargo command you launch:

```bash
cargo +nightly-2026-10-09 install --locked \
  --git https://github.com/huitseeker/mirai-endorlabs --branch main \
  --target-dir "$HOME/.cache/mirai-build" mirai
```

For a reproducible installation, replace `--branch main` with `--rev` followed by
a full commit SHA. Keep the nightly matched to that revision.

Cargo also fetches the repository's legacy `vcpkg` submodule during a Git
installation, even with the default solver feature. Its URL uses GitHub SSH,
so this route requires working GitHub SSH access and downloads extra history.
The checkout installation above avoids this fetch because it does not initialize
submodules.

## Verification and use

Check the installed package version:

```bash
cargo +nightly-2026-10-09 mirai --version
cargo install --list
```

For this revision, Cargo reports `mirai 1.1.12`. From a Rust project's directory,
run the analyzer with the same nightly:

```bash
cargo +nightly-2026-10-09 mirai
```

The project must compile with that compiler. You can keep a different default
Rust toolchain and select this nightly only when invoking MIRAI.

For a smoke test, create a project outside the checkout. This avoids adding
the example to MIRAI's Cargo workspace:

```bash
cd "$(mktemp -d)"
cargo +nightly-2026-10-09 new --bin --vcs none mirai-smoke
cd mirai-smoke
cat > src/main.rs <<'RS'
fn divide(x: i32) -> i32 { 100 / x }
fn main() { println!("{}", divide(0)); }
RS
cargo +nightly-2026-10-09 mirai
```

Expect a `[MIRAI] attempt to divide by zero` warning with a related location in
`divide`. The command succeeds while reporting this warning. A zero exit status alone
does not establish that the program has no reported issues.

## Installation quirks

On Linux, running `mirai` directly can fail with a missing
`librustc_driver-*.so` error. Plain `cargo mirai` can produce the same error when
launched through a different default toolchain, even if `cargo mirai --version`
works. Use `cargo +nightly-2026-10-09 mirai` to supply the matching compiler's
library paths. For direct analyzer commands, rustup can supply those paths:

```bash
rustup run nightly-2026-10-09 mirai --help
```

The standalone analyzer's `--version` currently prints the legacy `MIRAI v1.1.7`
banner. Use `cargo +nightly-2026-10-09 mirai --version` or `cargo install --list`
to check the installed package version.

Cargo may warn about an unused `proc-macro2` dependency during installation.
This warning occurred in a successful installation of this revision. Confirm
that Cargo finishes with `Installed package` and lists both executables.

The installation commands and smoke test were checked on Linux x86-64 using
fork commit `3c4f8568a1c6bf288c6e2ef95a33fc5f0c1e9324` on 2026-10-26.

## Contributing

See the [developer guide](DeveloperGuide.md) for build and debugging instructions.
