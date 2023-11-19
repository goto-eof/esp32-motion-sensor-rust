use esp_idf_sys as _;

use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() {
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let dp = Peripherals::take().unwrap();

    let sensor = PinDriver::input(dp.pins.gpio3).unwrap();
    // sensor.set_pull(Pull::Up).unwrap();

    let mut detection = false;

    loop {
        if sensor.is_low() && detection == true {
            println!("No detection!");
            detection = false;
        } else if sensor.is_high() && detection == false {
            println!("Movement detected");
            detection = true;
        }
    }
}
