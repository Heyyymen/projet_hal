// src/spi.rs
pub trait Spi {
    fn init(&self);
    fn send(&self, data: u8);
    fn receive(&self) -> u8;
    fn transfer(&self, data: u8) -> u8;
}

#[cfg(target = "avr-atmega328p")]
pub struct SpiAtmega328p;

#[cfg(target = "avr-atmega328p")]
impl Spi for SpiAtmega328p {
    fn init(&self) {
        // SPI initialization for Atmega328p
        // Enable SPI, Set as Master, Set clock rate, etc.
        unsafe {
            // Example of SPI initialization for Atmega328p (Arduino Uno)
            // Set MOSI, SCK as Output
            let _ = 0x20; // For example, DDRB |= (1 << DDB3) for MOSI, DDB5 for SCK (hardware register manipulation)
            
            // Enable SPI, Master mode, Clock rate fck/16 (for example)
            // SPCR |= (1 << SPE) | (1 << MSTR) | (1 << SPR0);
        }
    }

    fn send(&self, data: u8) {
        // Send data via SPI (blocking)
        unsafe {
            // Write data to SPDR (SPI Data Register)
            // SPDR = data;
            // Wait for transmission to complete
            while (SPSR & (1 << SPIF)) == 0 {}
        }
    }

    fn receive(&self) -> u8 {
        // Receive data via SPI (blocking)
        unsafe {
            // Wait for data to be received
            while (SPSR & (1 << SPIF)) == 0 {}
            // Return the received data from SPDR
            let data = SPDR;
            data
        }
    }

    fn transfer(&self, data: u8) -> u8 {
        self.send(data);
        self.receive()
    }
}

#[cfg(target = "esp32")]
pub struct SpiEsp32;

#[cfg(target = "esp32")]
impl Spi for SpiEsp32 {
    fn init(&self) {
        // SPI initialization for ESP32
        // Set up SPI bus with appropriate settings (example: SPI mode, frequency, etc.)
        // This might involve calling platform-specific APIs from the ESP32 SDK
    }

    fn send(&self, data: u8) {
        // Send data via SPI (using ESP32 specific API)
    }

    fn receive(&self) -> u8 {
        // Receive data via SPI (using ESP32 specific API)
        0 // Placeholder for received data
    }

    fn transfer(&self, data: u8) -> u8 {
        self.send(data);
        self.receive()
    }
}
