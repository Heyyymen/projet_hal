mod gpio;

fn main() {
    // Configure la broche 2 en sortie et la met à HIGH puis à LOW
    gpio::set_pin_mode(2, gpio::Mode::Output);
    gpio::write_pin(2, true);  // Met la broche 2 à HIGH
    gpio::write_pin(2, false); // Met la broche 2 à LOW

    // Configure la broche 3 en entrée et lit sa valeur
    gpio::set_pin_mode(3, gpio::Mode::Input);
    let value = gpio::read_pin(3);
    println!("The value of pin 3 is: {}", value);
}
    