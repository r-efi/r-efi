//
// Example: Hello World!
//
// This is an example UEFI application that prints "Hello World!", then waits for key input before
// it exits. It serves as base example how to write UEFI applications without any helper modules
// other than the UEFI protocol definitions.
//
// The `efi_main` function serves as entry-point. Depending on your target-configuration, this
// entry point might be called differently. If you use the target-configuration shipped with
// r-efi, then `efi_main` is the selected PE/COFF entry point.
//
// Additionally, a panic handler is provided. This is executed by rust on panic. For simplicity,
// we simply end up in an infinite loop. For real applications, this method should probably call
// into `SystemTable->boot_services->exit()` to exit the UEFI application. Note, however, that
// UEFI applications are likely to run in the same address space as the entire firmware. Hence,
// halting the machine might be a viable alternative. All that is out-of-scope of this example,
// though.
//
// Lastly, note that UEFI uses UTF-16 strings. Since rust literals are UTF-8, we have to use an
// open-coded, zero-terminated, UTF-16 array as argument to `output_string()`. Similarly to the
// panic handler, real applications should rather use UTF-16 modules.
//
// Note that as of rust-1.31.0, all features used here are stabilized. Not unstable features are
// required, nor do we rely on nightly compilers.
//

#![no_main]
#![no_std]

use r_efi::efi;

#[panic_handler]
fn rust_panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn efi_main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let mut s = [
        0x0048u16, 0x0065u16, 0x006cu16, 0x006cu16, 0x006fu16,              // "Hello"
        0x0020u16,                                                          // " "
        0x0057u16, 0x006fu16, 0x0072u16, 0x006cu16, 0x0064u16,              // "World"
        0x0021u16,                                                          // "!"
        0x000au16,                                                          // "\n"
        0x0000u16,                                                          // NUL
    ];

    // Print "Hello World!".
    let r = unsafe {
        ((*(*st).con_out).output_string)((*st).con_out, s.as_mut_ptr())
    };
    if r.is_error() {
        return r;
    }

    // Wait for key input, by waiting on the `wait_for_key` event hook.
    let r = unsafe {
        let mut x: usize = 0;
        ((*(*st).boot_services).wait_for_event)(1, &mut (*(*st).con_in).wait_for_key, &mut x)
    };
    if r.is_error() {
        return r;
    }

    efi::Status::SUCCESS
}
