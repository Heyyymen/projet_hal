mod gpio;

use gpio::GpioPin;

fn main() {
    // Exemple d'utilisation de la HAL pour les GPIO

    // Configure la pin 2 comme sortie et écrit une valeur.
    let pin2 = GpioPin::new_output(2);
    pin2.write(true);  // Met la pin à HIGH.

    // Configure la pin 3 comme entrée et lis sa valeur.
    let pin3 = GpioPin::new_input(3);
    let state = pin3.read();
    println!("State of pin 3: {}", state);
}
//test de maj
//test