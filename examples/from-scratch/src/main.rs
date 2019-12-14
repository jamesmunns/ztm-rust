#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: fn() -> ! = reset_handler;

pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut __sbss: u32; // Start of .bss section
        static mut __ebss: u32; // End of .bss section
        static mut __sdata: u32; // Start of .data section
        static mut __edata: u32; // End of .data section
        static __sidata: u32; // Start of .rodata section
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = &mut __sbss;
        let ebss: *mut u32 = &mut __ebss;

        while sbss < ebss {
            write_volatile(sbss, core::mem::zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = &mut __sdata;
        let edata: *mut u32 = &mut __edata;
        let mut sidata: *const u32 = &__sidata;

        while sdata < edata {
            write_volatile(sdata, read(sidata));
            sdata = sdata.offset(1);
            sidata = sidata.offset(1);
        }
    }

    // Call user main
    main()
}

fn main() -> ! {
    loop {
        continue;
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On a panic, loop forever
    loop {
        continue;
    }
}
