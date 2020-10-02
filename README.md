r-efi
=====

UEFI Reference Specification Protocol Constants and Definitions

The r-efi project provides the protocol constants and definitions of the
UEFI Reference Specification as native rust code. The scope of this project is
limited to those protocol definitions. The protocols are not actually
implemented. As such, this project serves as base for any UEFI application that
needs to interact with UEFI, or implement (parts of) the UEFI specification.

### Project

 * **Website**: <https://github.com/r-efi/r-efi/wiki>
 * **Bug Tracker**: <https://github.com/r-efi/r-efi/issues>

### Requirements

The requirements for this project are:

 * `rustc >= 1.31.0`

### Build

To build this project, run:

```sh
cargo build
```

Available configuration options are:

 * **examples**: This feature-selector enables compilation of examples. This
                 is disabled by default, since they will only compile
                 successfully on UEFI targets.

No special requirements exist to compile for UEFI targets. Native compilations
work out of the box without any adjustments. For cross-compilations, you have
to use either `cargo-xbuild` or the nightly versions of `cargo` and `rustc`. In
both cases, you need a nightly toolchain and the compiler sources:

```sh
rustup toolchain install nightly
# OR
rustup update

rustup component add --toolchain nightly rust-src
```

Be sure to update all components to the most recent version.

##### Build via: cargo/rustc nightly

```sh
cargo +nightly build \
    -Zbuild-std=core,compiler_builtins,alloc \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-uefi \
    --features examples \
    --examples
```

##### Build via: cargo-xbuild

```sh
cargo install --force cargo-xbuild

cargo \
    +nightly \
    xbuild \
    --target x86_64-unknown-uefi \
    --features examples \
    --examples
```

### Repository:

 - **web**:   <https://github.com/r-efi/r-efi>
 - **https**: `https://github.com/r-efi/r-efi.git`
 - **ssh**:   `git@github.com:r-efi/r-efi.git`

### License:

 - **Apache-2.0** OR **LGPL-2.1-or-later**
 - See AUTHORS file for details.
