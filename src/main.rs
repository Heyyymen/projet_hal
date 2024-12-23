#![no_std]
#![no_main]

extern crate panic_halt;

mod gpio;
mod usart;
mod spi;

use gpio::{Gpio, GpioAtmega328p, GpioEsp32};
use usart::{Usart, UsartAtmega328p, UsartEsp32};
use spi::{Spi, SpiAtmega328p, SpiEsp32};

#[cfg(target = "avr-atmega328p")]
fn setup_gpio() -> GpioAtmega328p {
    GpioAtmega328p
}

#[cfg(target = "esp32")]
fn setup_gpio() -> GpioEsp32 {
    GpioEsp32
}

#[cfg(target = "avr-atmega328p")]
fn setup_usart() -> UsartAtmega328p {
    UsartAtmega328p
}

#[cfg(target = "esp32")]
fn setup_usart() -> UsartEsp32 {
    UsartEsp32
}

#[cfg(target = "avr-atmega328p")]
fn setup_spi() -> SpiAtmega328p {
    SpiAtmega328p
}

#[cfg(target = "esp32")]
fn setup_spi() -> SpiEsp32 {
    SpiEsp32
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    // GPIO Example
    let gpio = setup_gpio();
    gpio.init();
    gpio.write(0); // Set GPIO low
    gpio.write(1); // Set GPIO high
    
    // USART Example
    let usart = setup_usart();
    usart.init();
    usart.send(0x41); // Send 'A' (ASCII 65)
    let received = usart.receive(); // Receive data
    usart.send(received); // Echo received data

    // SPI Example
    let spi = setup_spi();
    spi.init();
    spi.send(0x42); // Send data
    let received_data = spi.receive(); // Receive data
    let transfer_result = spi.transfer(0x43); // Transfer data
    // `transfer_result` contains the data received after sending 0x43

    loop {}
}
