#[derive(Debug)]
pub enum Mode {
    Input,
    Output,
}

pub fn set_pin_mode(pin: u8, mode: Mode) {
    match mode {
        Mode::Output => {
            println!("Configuring pin {} as Output", pin);
            unsafe {
                let ddrd = 0x2A as *mut u8; // Remplace avec l'adresse correcte de DDRD
                *ddrd |= 1 << pin; // Configure la broche `pin` en sortie
            }
        }
        Mode::Input => {
            println!("Configuring pin {} as Input", pin);
            unsafe {
                let ddrd = 0x2A as *mut u8; // Remplace avec l'adresse correcte de DDRD
                *ddrd &= !(1 << pin); // Configure la broche `pin` en entrée
            }
        }
    }
}

pub fn write_pin(pin: u8, value: bool) {
    unsafe {
        let portd = 0x2B as *mut u8; // Remplace avec l'adresse correcte de PORTD
        if value {
            *portd |= 1 << pin; // Met la broche `pin` à HIGH
        } else {
            *portd &= !(1 << pin); // Met la broche `pin` à LOW
        }
    }
}

pub fn read_pin(pin: u8) -> bool {
    unsafe {
        let pind = 0x29 as *const u8; // Remplace avec l'adresse correcte de PIND
        (*pind & (1 << pin)) != 0 // Retourne `true` si la broche est HIGH, `false` sinon
    }
}
