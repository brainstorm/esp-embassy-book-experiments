use esp_hal::peripherals;
use esp_println::println;

pub struct Uart<'a> {
    pub uart: peripherals::UART0<'a>
}

#[embassy_executor::task()]
pub async fn init(_uart: Uart<'static>) {
    println!("UART task started");
}

#[embassy_executor::task()]
pub async fn run(_uart: Uart<'static>) {
    println!("UART task started");
}