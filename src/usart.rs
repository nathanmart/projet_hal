use core::ptr;

// Adresses des registres pour l'USART de l'Atmega328p
const UCSRA: *mut u8 = 0xC0 as *mut u8;  // Registre de contrôle et statut A
const UCSR0B: *mut u8 = 0xC1 as *mut u8; // Registre de contrôle et statut B
const UCSR0C: *mut u8 = 0xC2 as *mut u8; // Registre de contrôle et statut C
const UBRR0L: *mut u8 = 0xC4 as *mut u8; // Registre inférieur pour le baud rate
const UBRR0H: *mut u8 = 0xC5 as *mut u8; // Registre supérieur pour le baud rate
const UDR0: *mut u8 = 0xC6 as *mut u8;   // Registre des données

pub struct Usart;

impl Usart {
    // Initialisation avec un baud rate spécifique
    pub fn init(baud_rate: u16){
        unsafe {
            // Configuration du baud rate en l'écrivant dans les registres adéquats
            ptr::write_volatile(UBRR0H, (baud_rate >> 8) as u8); // Haute
            ptr::write_volatile(UBRR0L, baud_rate as u8); // Basse

            // Activation de la transmission et de la réception
            ptr::write_volatile(UCSR0B, (1 << 3) | (1 << 4));

            // Configuration du format de données : 8 bits de données et 1 nit stop
            ptr::write_volatile(UCSR0C, (1 << 1) | (1 << 1));
        }
    }

    // Envoie d'un octet de données
    pub fn send(data: u8){
        unsafe {
            // Attend que le buffer de transmission soit prêt
            while ptr::read_volatile(UCSRA) & (1 << 5) == 00 {}

            // Ecrit
            ptr::write_volatile(UDR0, data);
        }
    }

    // Reception d'un octet de données
    pub fn receive() -> u8{
        unsafe {
            // Attend que des données soient dans le buffer
            while ptr::read_volatile(UCSRA) & (1 << 7) == 0 {}

            // Lit
            ptr::read_volatile(UDR0)
        }
    }
}