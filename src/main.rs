#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

//use embassy_sync::channel::Channel;
use embassy_executor::Spawner;
use esp_hal::{assign_resources, clock::CpuClock};
use esp_hal_embassy;
use esp_embassy_channels::tasks;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    //let uart = static_cell::StaticCell::new();

    // let uart = tasks::uart::Uart {
    //     uart: peripherals.UART0
    // };

    // let wifi = tasks::wifi::Wifi {
    //     radio: peripherals.WIFI,
    //     timg0: peripherals.TIMG0,
    //     rng: peripherals.RNG,
    //     systimer: peripherals.SYSTIMER,
    // };

    // Spawn all tasks
    spawner.spawn(tasks::uart::init(uart)).unwrap();
    spawner.spawn(tasks::uart::run(uart)).unwrap();

    spawner.spawn(tasks::wifi::init(wifi)).unwrap();
    spawner.spawn(tasks::wifi::run(wifi)).unwrap();
}
