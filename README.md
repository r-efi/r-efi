r-efi
=====

UEFI Reference Specification Protocol Constants and Definitions

The r-efi project provides the protocol constants and definitions of the
UEFI Reference Specification as native rust code. The scope of this project is
limited to those protocol definitions. The protocols are not actually
implemented. As such, this project serves as base for any UEFI application that
needs to interact with UEFI, or implement (parts of) the UEFI specification.

## Project

 * Website: [@r-util](https://r-util.github.io/r-efi)
 * Bug Tracker: [@github](https://github.com/r-util/r-efi/issues)
 * Git Web: [@github](https://github.com/r-util/r-efi)

## Requirements

The requirements for r-efi are:

 * rustc >= 1.31.0-nightly

## License

 * Apache Software License 2.0
 * Lesser General Public License 2.1+

See AUTHORS for details.

## Build

No special requirements exist to compile for UEFI targets. Native compilations
work out of the box without any adjustments. In case of cross-compilation, you
need a target-configuration as input to the rust compiler. These are provided
alongside this project.

Our recommended way to cross-compile this project is to use `cargo-xbuild`. It
then becomes as simple as the following command to build the example
applications shipped with this project:

```
cargo xbuild --target src/x86_64-unknown-uefi.json --examples
```
