use esp_println::println;
use esp_hal::{assign_resources, peripherals};
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_radio_preempt_baremetal;

assign_resources! {
    Resources<'d> {
        wifi: WifiResources<'d> {
            radio: WIFI,
            timg0: TIMG0,
            rng: RNG,
            systimer: SYSTIMER,
        },
    }
}
// pub struct Wifi<'a> {
//     pub radio: peripherals::WIFI<'a>,
//     pub timg0: peripherals::TIMG0<'a>,
//     pub rng: peripherals::RNG<'a>,
//     pub systimer: peripherals::SYSTIMER<'a>,
// }

#[embassy_executor::task()]
pub async fn init(wifi: WifiResources<'static>) {
    println!("Wifi initialization started");

    let timg0 = TimerGroup::new(wifi.timg0);
    esp_radio_preempt_baremetal::init(timg0.timer0);

    let timer0 = SystemTimer::new(wifi.systimer);
    esp_hal_embassy::init(timer0.alarm0);

    let wifi_init =
        esp_radio::init().expect("Failed to initialize WIFI/BLE controller");
    let (mut _wifi_controller, _interfaces) = esp_radio::wifi::new(&wifi_init, wifi.radio)
        .expect("Failed to initialize WIFI controller");
}

#[embassy_executor::task()]
pub async fn run(_wifi: WifiResources<'static>) {
    println!("Wifi task started");
}