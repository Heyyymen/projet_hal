pub mod usart {
    use core::ptr;

    const UDR0: *mut u8 = 0xC6 as *mut u8; // USART Data Register
    const UCSR0A: *mut u8 = 0xC0 as *mut u8; // USART Control and Status Register A
    const UCSR0B: *mut u8 = 0xC1 as *mut u8; // USART Control and Status Register B
    const UCSR0C: *mut u8 = 0xC2 as *mut u8; // USART Control and Status Register C
    const UBRR0H: *mut u8 = 0xC5 as *mut u8; // USART Baud Rate Register High
    const UBRR0L: *mut u8 = 0xC4 as *mut u8; // USART Baud Rate Register Low

    pub struct Usart;

    impl Usart {
        pub fn new() -> Self {
            Usart
        }

        /// Initializes USART with a given baud rate
        pub fn init(&self, baud_rate: u16) {
            let ubrr = (16_000_000 / (16 * baud_rate as u32) - 1) as u16;
            unsafe {
                ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8);
                ptr::write_volatile(UBRR0L, ubrr as u8);

                ptr::write_volatile(UCSR0B, (1 << 3) | (1 << 4)); // Enable TX and RX
                ptr::write_volatile(UCSR0C, (1 << 1) | (1 << 2)); // Set 8-bit data
            }
        }

        /// Transmits a single byte
        pub fn write(&self, data: u8) {
            unsafe {
                while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {} // Wait for TX buffer to be empty
                ptr::write_volatile(UDR0, data); // Send the byte
            }
        }

        /// Receives a single byte
        pub fn read(&self) -> u8 {
            unsafe {
                while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {} // Wait for RX data to be available
                ptr::read_volatile(UDR0) // Return received byte
            }
        }
    }
}
