#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// This function is the entry point, since the linker looks for a function
// name '_start' by default.
#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
