#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // let vga_buffer = 0xb8000 as *mut u8; // mutable raw pointer to a single byte
    println!("Hello World{}", "!");
    panic!("I am panicking huhuhuhu!!!");

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte; // write the byte to the correct location
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // set the color byte
    //     }
    // }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
