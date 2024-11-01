// Importer le module GPIO
mod gpio;

use embedded_hal::digital::v2::OutputPin;
/* [CORRECTION GPIO] 
   You should not use an external HAL. What you did in gpio.rs is more what is expecting
(Don't hesitate to remove this comment)*/
use gpio::GpioPin;

fn main() {
    // Création d'une instance de GpioPin pour la pin 2
    let pin2 = GpioPin { pin: 2 };

    // Pin en mode sortie
    pin2.configure_as_output();

    // Met pin à l'état haut
    pin2.write(true);

    // Lecture de l'état
    let state = pin2.read();
    println!("GPIO state (high expeted): {:?}", state);

    // Changer l'état du pin et lire à nouveau l'état
    pin2.write(false);
    let state = pin2.read();
    println!("GPIO state (low expeted): {:?}", state);

    // Configurer pin en entré et activer la pull-up
    pin2.configure_as_input();
    pin2.enable_pullup();

    // Lire l'état du pin
    let state = pin2.read();
    println!("GPIO state in entry with pull-up: {:?}", state);

}
