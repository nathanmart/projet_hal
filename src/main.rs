/*
[CORRECTION SPI] (don't hesitate to remove this part)
You didn't implement the SPI feature.
*/
// Importer les modules GPIO et USART
mod gpio;
mod usart_atmega328p;
mod usart_esp8266;


// Configuration conditionnelle pour sélectionner le module USART approprié
#[cfg(target_arch = "avr")]
use crate::usart_atmega328p::Usart;

#[cfg(target_arch = "xtensa")]
use crate::usart_esp8266::Usart;

fn main() {
    println!("--- DÉBUT DES TESTS ---");

    // --- PARTIE 1 - TEST DES GPIO ---
    // Création d'une instance de GpioPin pour la pin 2 (pour Atmega328p)
    #[cfg(target_arch = "avr")]
    {
        let pin2 = GpioPin { pin: 2 };

        // Configurer la pin comme sortie et tester l'état
        pin2.configure_as_output();
        pin2.write(true);
        let state = pin2.read();
        println!("GPIO state (high expected): {:?}", state);

        pin2.write(false);
        let state = pin2.read();
        println!("GPIO state (low expected): {:?}", state);

        // Configurer la pin en entrée et activer la résistance pull-up
        pin2.configure_as_input();
        pin2.enable_pullup();
        let state = pin2.read();
        println!("GPIO state in entry with pull-up: {:?}", state);
    }

    // --- PARTIE 2 - USART ---
    println!("--- TEST USART ---");

    // Test pour Atmega328p
    #[cfg(target_arch = "avr")]
    {
        println!("Initialisation de l'USART (Atmega328p, 9600 baud, mode normal)");
        Usart::init(9600, false);
        Usart::send(b'H');
        Usart::send(b'e');
        Usart::send(b'l');
        Usart::send(b'l');
        Usart::send(b'o');
        println!("Message envoyé en mode normal : Hello");

        println!("Initialisation en mode double vitesse");
        Usart::init(9600, true);
        Usart::send(b'D');
        Usart::send(b'o');
        Usart::send(b'u');
        Usart::send(b'b');
        Usart::send(b'l');
        Usart::send(b'e');
        println!("Message envoyé en double vitesse : Double");

        let received = Usart::receive();
        println!("Caractère reçu : {}", received);
    }

    // Test pour ESP8266
    #[cfg(target_arch = "xtensa")]
    {
        println!("Initialisation de l'USART (ESP8266, 9600 baud)");
        Usart::init(9600);
        Usart::send(b'H');
        Usart::send(b'e');
        Usart::send(b'l');
        Usart::send(b'l');
        Usart::send(b'o');
        Usart::send(b' ');
        Usart::send(b'E');
        Usart::send(b'S');
        Usart::send(b'P');
        Usart::send(b'8');
        Usart::send(b'2');
        Usart::send(b'6');
        Usart::send(b'6');
        println!("Message envoyé pour ESP8266 : Hello ESP8266");
    }

    println!("--- FIN DES TESTS ---");
}
