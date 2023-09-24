use anyhow::Result;
use log::info;
use std::thread::sleep;
use std::time::Duration;

use esp_idf_sys as _;

use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> Result<(), anyhow::Error> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().ok_or(anyhow::anyhow!("Failed to take peripherals"))?;
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;

    
    loop {
        led.set_high()?;
        info!("Led on");

        sleep(Duration::from_millis(500));

        led.set_low()?;
        info!("Led off");

        sleep(Duration::from_millis(500));
    }
}
