#[cfg(target_arch = "avr")]
mod usart_atmega328p;
#[cfg(target_arch = "avr")]
pub use usart_atmega328p::UsartAtmega328p as Usart;

#[cfg(target_arch = "xtensa")]
mod usart_esp8266;
#[cfg(target_arch = "xtensa")]
pub use usart_esp8266::UsartEsp8266 as Usart;
