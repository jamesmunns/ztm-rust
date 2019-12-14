#![allow(dead_code)]

// A not-very-safe abstraction of GPIOs in Rust
pub mod gpio {
    #[derive(Copy, Clone)]
    pub struct Pin(u8);

    pub const P0_31: Pin = Pin(31);

    #[derive(Copy, Clone)]
    pub enum Level {
        Low,
        High,
    }

    const REG_P0_PIN_CNF: [*mut u32; 32] = [
        0x5000_0700 as *mut u32,
        0x5000_0704 as *mut u32,
        0x5000_0708 as *mut u32,
        0x5000_070C as *mut u32,
        0x5000_0710 as *mut u32,
        0x5000_0714 as *mut u32,
        0x5000_0718 as *mut u32,
        0x5000_071C as *mut u32,
        0x5000_0720 as *mut u32,
        0x5000_0724 as *mut u32,
        0x5000_0728 as *mut u32,
        0x5000_072C as *mut u32,
        0x5000_0730 as *mut u32,
        0x5000_0734 as *mut u32,
        0x5000_0738 as *mut u32,
        0x5000_073C as *mut u32,
        0x5000_0740 as *mut u32,
        0x5000_0744 as *mut u32,
        0x5000_0748 as *mut u32,
        0x5000_074C as *mut u32,
        0x5000_0750 as *mut u32,
        0x5000_0754 as *mut u32,
        0x5000_0758 as *mut u32,
        0x5000_075C as *mut u32,
        0x5000_0760 as *mut u32,
        0x5000_0764 as *mut u32,
        0x5000_0768 as *mut u32,
        0x5000_076C as *mut u32,
        0x5000_0770 as *mut u32,
        0x5000_0774 as *mut u32,
        0x5000_0778 as *mut u32,
        0x5000_077C as *mut u32,
    ];
    const REG_P0_OUT_SET: *mut u32 = 0x5000_0508 as *mut u32;
    const REG_P0_OUT_CLR: *mut u32 = 0x5000_050C as *mut u32;

    const PIN_CNF_DIR_OUTPUT: u32 = 0x0000_0001;
    const PIN_CNF_INPUT_CONNECT: u32 = 0x0000_0000;
    const PIN_CNF_PULL_DISABLED: u32 = 0x0000_0000;
    const PIN_CNF_DRIVE_S0S1: u32 = 0x0000_0000;
    const PIN_CNF_SENSE_DISABLED: u32 = 0x0000_0000;

    impl Pin {
        pub fn set_push_pull_output(&mut self, level: Level) {
            // set level
            match level {
                Level::High => self.set_high(),
                Level::Low => self.set_low(),
            }

            let set_val = PIN_CNF_DIR_OUTPUT
                | PIN_CNF_INPUT_CONNECT
                | PIN_CNF_PULL_DISABLED
                | PIN_CNF_DRIVE_S0S1
                | PIN_CNF_SENSE_DISABLED;

            unsafe {
                core::ptr::write_volatile(REG_P0_PIN_CNF[self.0 as usize], set_val);
            }
        }

        pub fn set_low(&mut self) {
            unsafe { core::ptr::write_volatile(REG_P0_OUT_SET, 1 << (self.0 as u32)) }
        }

        pub fn set_high(&mut self) {
            unsafe { core::ptr::write_volatile(REG_P0_OUT_CLR, 1 << (self.0 as u32)) }
        }
    }
}
