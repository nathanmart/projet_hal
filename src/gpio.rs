use core::ptr;

// Définition des adresses des registres DDRB, PORTB et PINB
const DDRB: *mut u8 = 0x24 as *mut u8; // Registre de direction des données
const PORTB: *mut u8 = 0x25 as *mut u8; // Registre pour l'écriture des données sur les pins
const PINB: *mut u8 = 0x23 as *mut u8; // Registre pour la lecture des données des pins

// Definition de la structure GpioPin qui représente une pin GPIO
pub struct GpioPin {
    pub pin: u8,
}

impl GpioPin {
    // Fonction pour créer une nouvelle instance de GpioPin avec un numéro de pin soécifié
    // Cette fonction prend un numéro de pin en entrée et renvoie une instance de GpioPin
    pub fn new(pin: u8) -> Self {
        GpioPin { pin } // Initialise une structure GpioPin avec le numéro de pin donné
    }

    // Configuration d'une pin comme sortie
    pub fn configure_as_output(&self) {
        unsafe {
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << self.pin));
        }
    }

    // Configuration d'une pin comme entré
    pub fn configure_as_input(&self) {
        unsafe {
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << self.pin));
        }
    }

    // Lire l'état d'une pin
    // Renvoie true sir état haut, false si état bas
    pub fn read(&self) -> bool {
        unsafe {
            (ptr::read_volatile(PINB) & (1 << self.pin)) !=0
        }
    }

    // Changer l'état d'une pin
    pub fn write(&self, state: bool) {
        unsafe {
            if state {
                // Si l'état est 'true', on met la pin en état haut
                ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << self.pin));
            } else {
                // Sinon on met la pin en état bas
                ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) & !(1 << self.pin));
            }
        }
    }

    // Fonctionnalité optionnel : Activer la résitance pull-up
    pub fn enable_pullup(&self) {
        unsafe {
            ptr::write_volatile(PORTB, ptr::read_volatile(PORTB) | (1 << self.pin));
        }
    }

}