# intel-onemkl-prebuild

[![Latest version](https://img.shields.io/crates/v/intel-onemkl-prebuild.svg)](https://crates.io/crates/intel-onemkl-prebuild)
[![docs.rs](https://img.shields.io/docsrs/intel-onemkl-prebuild)](https://docs.rs/intel-onemkl-prebuild)
[![GitHub License](https://img.shields.io/github/license/jkawamoto/intel-onemkl-prebuild)](https://github.com/jkawamoto/intel-onemkl-prebuild/blob/main/LICENSE)
[![Build](https://github.com/jkawamoto/intel-onemkl-prebuild/actions/workflows/build.yaml/badge.svg)](https://github.com/jkawamoto/intel-onemkl-prebuild/actions/workflows/build.yaml)

`intel-onemkl-prebuild` is a Rust crate that automatically sets up and links the
[Intel oneAPI Math Kernel Library (oneMKL)](https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html).
It downloads and installs the prebuilt oneMKL binaries, and exposes them to your Rust project.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
intel-onemkl-prebuild = "0.1.1"
```

Then add the extern declaration to your `main.rs` or `lib.rs` to ensure the libraries are properly linked:

```rust
extern crate intel_onemkl_prebuild;
```

This crate also exports the `DEP_MKL_ROOT` environment variable,
which points to the root directory of the installed MKL.
You can use this in your `build.rs` to locate the library.

## Features

You can control the integer size and threading model via Cargo features:

- `ilp64` — use the 64-bit integer interface.
- `lp64` — use the 32-bit integer interface.
- `openmp` — use the Intel OpenMP runtime.
- `sequential` — use sequential (single-threaded) execution.

Default features are: `ilp64` and `sequential`.

## Prerequisites

- Windows: The path to nuget.exe must be available in your PATH.
- Linux: No special tools are required beyond a standard build environment.

This crate will automatically download and install Intel oneMKL for the supported platforms.

### Rust Compatibility

- Edition: Rust 2021
- MSRV: 1.77 (Minimum Supported Rust Version)

## License

This application is released under the MIT License. For details, see the [LICENSE](LICENSE) file.

By using this crate, you agree to the Intel MKL
[End User License Agreement](https://www.intel.com/content/www/us/en/developer/articles/license/end-user-license-agreement.html).
