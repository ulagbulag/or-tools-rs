# Google Or-Tools

[![Current Crates.io Version](https://img.shields.io/crates/v/or-tools.svg)](https://crates.io/crates/or-tools)

`or-tools` is a Rust library that binds to certain specific features of the [Google Or-Tools C++ library](https://github.com/google/or-tools).

This repository will dedicate itself to improve the library's content.

These include:

- TBD # TODO: to be implemented!

## Building

### Minimum Rust Version

Please udjust your Rust compiler to version `1.73` or higher.

### Supported Platforms

- Linux { aarch64, x86_64 }
  - Alpine Edge
  - [Arch Linux (AUR)](https://aur.archlinux.org/packages/or-tools)
  - CentOS 7 LTS
  - Debian 11
  - Fedora 37, 38
  - OpenSuse Leap
  - Ubuntu 20.04, 22.04, 23.04
- MacOS { aarch64, x86_64 }
  - macOS Intel
  - macOS M1
- Windows { x86_64 }
  - Visual Studio 2022

Your package manager may not support deployment on platforms marked as `Supported` above.

For better maintenance, please let us know whether the other platforms support it.
Besides, you may claim us whether the specific platform should support it through `Issues` .

### Dependencies

- C++20 Compiler (GCC 10 or above)
- `cmake >= 3.18`
- \[optional\] `Or-Tools C++`

#### Building Dependencies

- git

### Building Native library

`or-tools` requires `Or-Tools` to be installed. You can either provide a existing system-wide installation, or build it with this library.

- To build it in compile-time:
  - ```sh
    cargo build --features build-native
    ```
- To use a system-wide dependency:
  - ```sh
    cargo build
    ```

The C++ library `Or-Tools` will be installed via `or-tools-sys` when the `build-native` feature flag is enabled.

For the build, this library uses `cmake`, so please make sure to have [ `cmake` ](https://cmake.org/install/) .

The `build-native` flag is **disabled by default**, offering increased build times.

### Building Rust package

`or-tools` includes a `solver-all` feature flag **enabled by default**.

`solver-all` will enable all kinds of supported **open-source** solvers.
The latest information about the solvers can be found here: https://github.com/google/or-tools/blob/stable/cmake/README.md#solvers-supported

The `solver-all` flag can be disabled with `cargo build --no-default-features`.

#### GPL/Proprietary Solvers

`or-tools` includes a `solver-all-nonfree` feature flag that can be used with `cargo build --features solver-all-nonfree` .

`solver-all` will enable all kinds of supported solvers including **GPL** and/or **proprietary** ones.
The latest information about the solvers can be found here: https://github.com/google/or-tools/blob/stable/cmake/README.md#solvers-supported

For detailed instructions on embedding proprietary solvers, please follow to the link below:

- CPLEX: https://github.com/google/or-tools/blob/stable/cmake/README.md#enabling-cplex-support
- XPRESS: https://github.com/google/or-tools/blob/stable/cmake/README.md#enabling-xpress-support

The `solver-all-nonfree` flag is disabled by default, offering increased build times.
