use core::ptr;

// Adresses des registres UART pour l'ESP8266
const UART_BASE: *mut u32 = 0x60000000 as *mut u32; // Base UART
const UART_FIFO: *mut u32 = (UART_BASE as usize + 0x0) as *mut u32; // FIFO Register
const UART_INT_ST: *mut u32 = (UART_BASE as usize + 0x8) as *mut u32; // Interrupt Status
const UART_INT_ENA: *mut u32 = (UART_BASE as usize + 0xC) as *mut u32; // Interrupt Enable
const UART_CLKDIV: *mut u32 = (UART_BASE as usize + 0x14) as *mut u32; // Baud Rate

pub struct UsartEsp8266;

impl UsartEsp8266 {
    // Initialisation UART pour l'ESP8266
    pub fn init(baud_rate: u32) {
        unsafe {
            // Configure le baud rate
            let clkdiv = 80000000 / baud_rate; // Fréquence d'horloge par défaut
            ptr::write_volatile(UART_CLKDIV, clkdiv);

            // Active les interruptions si nécessaire
            ptr::write_volatile(UART_INT_ENA, 0x00); // Désactive toutes les interruptions
        }
    }

    // Envoie un octet
    pub fn send(data: u8) {
        unsafe {
            // Attendre que le FIFO soit prêt
            while ptr::read_volatile(UART_INT_ST) & (1 << 1) == 0 {}
            // Écrire dans le registre FIFO
            ptr::write_volatile(UART_FIFO, data as u32);
        }
    }

    // Réception d'un octet
    pub fn receive() -> u8 {
        unsafe {
            // Attendre qu'il y ait des données dans le FIFO
            while ptr::read_volatile(UART_INT_ST) & (1 << 0) == 0 {}
            // Lire depuis le registre FIFO
            ptr::read_volatile(UART_FIFO) as u8
        }
    }
}
