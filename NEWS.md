# r-efi - UEFI Reference Specification Protocol Constants and Definitions

## CHANGES WITH 3.2.0:

        * Add new protocols: DiskIo, DiskIo2, BlockIo, DriverBinding

        * Extend the Device-Path payload structure and add the HardDriveMedia
          payload.

        * Add HII definitions: A new top-level module `hii` with all the basic
          HII constants, as well as a handful of HII protocols (hii_database,
          hii_font, hii_string)

        * Document new `-Zbuild-std` based cross-compilation, serving as
          official rust alternative to cargo-xbuild.

        Contributions from: Alex James, Bret Barkelew, David Rheinsberg,
                            Michael Kubacki

        - Tübingen, 2020-10-23

## CHANGES WITH 3.1.0:

        * Add the basic networking types to `r_efi::base`. This includes MAC
          and IP address types.

        * Add the EFI_SIMPLE_NETWORK_PROTOCOL definitions and all required
          constants to make basic networking available.

        * Add a new uefi-cross example, which is copied from upstream rustc
          sources, so we can test local modifications to it.

        Contributions from: Alex James, David Rheinsberg

        - Tübingen, 2020-09-10

## CHANGES WITH 3.0.0:

        * Fix a missing parameter in `BootServices::locate_device_path()`. The
          prototype incorrectly had 2 arguments, while the official version
          takes 3. The final `handle` argument was missing.
          This is an API break in `r-efi`. It should have a limited impact,
          since the function was mostly useless without a handle.
          Thanks to Michael Kubacki for catching this!

        * Adjust the `device_path` parameter in a bunch of `BootServices`
          calls. This used to take a `*mut c_void` parameter, since the device
          path protocol was not implemented.
          Since we have to bump the major version anyway, we use this to also
          fix these argument-types to the correct device-path protocol type,
          which has been implemented some time ago.

        Contributions from: David Rheinsberg, Michael Kubacki

        - Tübingen, 2020-04-24

## CHANGES WITH 2.2.0:

        * Provide `as_usize()` accessor for `efi::Status` types. This allows
          accessing the raw underlying value of a status object.

        * The project moved to its new home at: github.com/r-efi/r-efi

        Contributions from: David Rheinsberg, Joe Richey

        - Tübingen, 2020-04-16

## CHANGES WITH 2.1.0:

        * Add the graphics-output-protocol.

        * Expose reserved fields in open structures, otherwise they cannot be
          instantiated from outside the crate itself.

        Contributions from: David Herrmann, Richard Wiedenhöft, Rob Bradford

        - Tübingen, 2019-03-20

## CHANGES WITH 2.0.0:

        * Add a set of UEFI protocols, including simple-text-input,
          file-protocol, simple-file-system, device-path, and more.

        * Fix signature of `BootServices::allocate_pages`.

        Contributions from: David Rheinsberg, Richard Wiedenhöft, Tom Gundersen

        - Tübingen, 2019-03-01

## CHANGES WITH 1.0.0:

        * Enhance the basic UEFI type integration with the rust ecosystem. Add
          `Debug`, `Eq`, `Ord`, ... derivations, provide converters to/from the
          core library, and document the internal workings.

        * Fix `Boolean` to use `newtype(u8)` to make it ABI compatible to UEFI.
          This now accepts any byte value that UEFI accetps without any
          conversion required.

        Contributions from: Boris-Chengbiao Zhou, David Rheinsberg, Tom
                            Gundersen

        - Tübingen, 2019-02-14

## CHANGES WITH 0.1.1:

        * Feature gate examples to make `cargo test` work on non-UEFI systems
          like CI.

        Contributions from: David Herrmann

        - Tübingen, 2018-12-10

## CHANGES WITH 0.1.0:

        * Initial release of r-efi.

        Contributions from: David Herrmann

        - Tübingen, 2018-12-10
