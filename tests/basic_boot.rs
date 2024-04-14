#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tiny_os::println;

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tiny_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

// Ideas for future integration tests:
// 1. CPU exceptions
// 2. Page tables
// 3. Userspace programs
