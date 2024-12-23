pub mod gpio {
    use core::ptr;

    pub enum PinMode {
        Input,
        Output,
    }

    pub struct GpioPin {
        ddr: *mut u8,   // Data Direction Register
        port: *mut u8,  // PORT Register
        pin: *const u8, // PIN Register
        pin_number: u8, // Pin number (0-7)
    }

    impl GpioPin {
        /// Creates a new GPIO pin abstraction
        pub const fn new(ddr: *mut u8, port: *mut u8, pin: *const u8, pin_number: u8) -> Self {
            GpioPin {
                ddr,
                port,
                pin,
                pin_number,
            }
        }

        /// Configures the pin as input or output
        pub fn set_mode(&self, mode: PinMode) {
            unsafe {
                match mode {
                    PinMode::Input => {
                        ptr::write_volatile(self.ddr, ptr::read_volatile(self.ddr) & !(1 << self.pin_number));
                    }
                    PinMode::Output => {
                        ptr::write_volatile(self.ddr, ptr::read_volatile(self.ddr) | (1 << self.pin_number));
                    }
                }
            }
        }

        /// Writes a high or low value to the pin
        pub fn write(&self, value: bool) {
            unsafe {
                if value {
                    ptr::write_volatile(self.port, ptr::read_volatile(self.port) | (1 << self.pin_number));
                } else {
                    ptr::write_volatile(self.port, ptr::read_volatile(self.port) & !(1 << self.pin_number));
                }
            }
        }

        /// Reads the pin value
        pub fn read(&self) -> bool {
            unsafe { ptr::read_volatile(self.pin) & (1 << self.pin_number) != 0 }
        }
    }
}
