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
    pub fn init(baud_rate: u16, double_speed: bool) {
        unsafe {
            if double_speed {
                // Active le mode double vitesse
                ptr::write_volatile(UCSRA, ptr::read_volatile(UCSRA) | (1 << 1));
            } else {
                // Désactive le mode double vitesse
                ptr::write_volatile(UCSRA, ptr::read_volatile(UCSRA) & !(1 << 1));
            }

            // Calcul du baud rate en utilisant u32 pour éviter le dépassement
            let ubrr: u16 = if double_speed {
                ((16_000_000u32 / (8 * baud_rate as u32)) - 1) as u16 // Mode double vitesse
            } else {
                ((16_000_000u32 / (16 * baud_rate as u32)) - 1) as u16 // Mode normal
            };

            // Configure le baud rate
            ptr::write_volatile(UBRR0H, (ubrr >> 8) as u8); // Partie haute
            ptr::write_volatile(UBRR0L, ubrr as u8);        // Partie basse

            // Active la transmission et la réception
            ptr::write_volatile(UCSR0B, (1 << 3) | (1 << 4));

            // Configuration du format : 8 bits de données, 1 stop bit
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