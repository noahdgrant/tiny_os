#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tiny_os::println;

// This function is the entry point, since the linker looks for a function
// name '_start' by default.
#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    tiny_os::init();

    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tiny_os::test_panic_handler(info)
}

#[test_case]
fn test_trivial_assertion() {
    assert_eq!(1, 1);
}
