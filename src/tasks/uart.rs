use esp_hal::peripherals;
use esp_println::println;


#[embassy_executor::task()]
pub async fn init(peripherals: peripherals::Peripherals) {
    println!("UART task started");
}

// Why can't I pub(crate) here instead of pub?
#[embassy_executor::task()]
pub async fn run() {
    println!("UART task started");
}