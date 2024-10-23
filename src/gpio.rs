// Module pour la gestion des GPIO
pub struct GpioPin {
    pin_number: u8,
    is_output: bool,
}

impl GpioPin {
    /// Configure la broche comme une sortie.
    pub fn new_output(pin_number: u8) -> Self {
        // Configuration du registre pour une sortie.
        // À compléter avec les registres spécifiques à l'Atmega328p.
        GpioPin {
            pin_number,
            is_output: true,
        }
    }

    /// Configure la broche comme une entrée.
    pub fn new_input(pin_number: u8) -> Self {
        // Configuration du registre pour une entrée.
        // À compléter avec les registres spécifiques à l'Atmega328p.
        GpioPin {
            pin_number,
            is_output: false,
        }
    }

    /// Écrit une valeur sur une broche configurée en sortie.
    pub fn write(&self, state: bool) {
        if self.is_output {
            // Code pour écrire l'état sur le registre.
            // À compléter pour l'Atmega328p.
            println!(
                "Writing {} to pin {}",
                if state { "HIGH" } else { "LOW" },
                self.pin_number
            );
        } else {
            println!("Pin {} is not configured as output", self.pin_number);
        }
    }

    /// Lit l'état de la broche configurée en entrée.
    pub fn read(&self) -> bool {
        if !self.is_output {
            // Code pour lire l'état depuis le registre.
            // À compléter pour l'Atmega328p.
            println!("Reading state from pin {}", self.pin_number);
            true // Remplacer par la valeur lue
        } else {
            println!("Pin {} is not configured as input", self.pin_number);
            false
        }
    }
}
