#![no_std]
#![no_main]

use core::panic::PanicInfo;

fn main() {
    loop {
        continue;
    }
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
