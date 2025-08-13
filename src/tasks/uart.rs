//use esp_hal::{assign_resources, peripherals};
use esp_hal::peripherals;
use esp_println::println;

// assign_resources! {
//     Resources<'d> {
//         uart: UartResources<'d> {
//             uart0: UART0,
//         },
//     }
// }

pub struct UartResources<'a> {
    pub uart: peripherals::UART0<'a>
}

impl<'a> UartResources<'static> {
    pub fn new(uart: peripherals::UART0<'static>) -> Self {
        Self { uart }
    }
}

#[embassy_executor::task()]
pub async fn init(uart: UartResources<'static>) {
    println!("UART task started");
}

#[embassy_executor::task()]
pub async fn run(uart: UartResources<'static>) {
    println!("UART task started");
}