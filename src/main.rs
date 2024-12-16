/*
[CORRECTION USART] (Don't hesitate to remove this part)
Even if you don't use the external crates embedded-hal and avr-device, I recommand to remove them from your project with the ```cargo remove your_dependencies```, it could generate some errors.
One target is missing (one of your choice).
You could try implementing the different USART mode (asynchrone double speed for example) for your Atmega target.
*/
// Importer le module GPIO
mod gpio;
mod usart_atmega328p;
mod usart_esp8266;
mod usar;

use gpio::GpioPin;
use crate::usart_atmega328p::Usart;

fn main() {
    // --- PARTIE 1 - TEST DES GPIO ---
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


    // --- PARTIE 2 - USART ---
    // Initialisation de l'USART avec baud rate = 9600
    Usart::init(9600);

    // ENvoie d'un message, caractère par caractère
    Usart::send(b'H');
    Usart::send(b'e');
    Usart::send(b'l');
    Usart::send(b'l');
    Usart::send(b'o');
    Usart::send(b' ');
    Usart::send(b'W');
    Usart::send(b'o');
    Usart::send(b'r');
    Usart::send(b'l');
    Usart::send(b'd');

    // Recevoir un caractère
    let received = Usart::receive();
    println!("Caractère reçu: {}", received);
}
