#![no_std]
#![no_main]

use core::panic::PanicInfo;

// function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static WELCOME: &[u8] = b"Welcome to NicOS!";

#[no_mangle]
pub extern  "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in WELCOME.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}