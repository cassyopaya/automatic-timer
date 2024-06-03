use anyhow::Result;
use embedded_svc::http::Method;
use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::{Gpio2, OutputPin, PinDriver};
use esp_idf_svc::http::server::{EspHttpServer, Configuration};
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::eventloop::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut led = PinDriver::output(pins.gpio2)?;

    // let wifi = setup_wifi()?;
    let mut server = EspHttpServer::new(&Default::default())?;

    let timer_values = Arc::new(Mutex::new((Duration::from_secs(240), Duration::from_secs(480)))); // 4 minutes on, 8 minutes off
    let timer_values_clone = Arc::clone(&timer_values);

    // Define the web server routes
    // server.fn_handler("/set_times", Method::Post, move |request| {
        // let body = request.into_body();
        // let times: Vec<&str> = body.split(',').collect();
        // if times.len() == 2 {
        //     if let (Ok(on_time), Ok(off_time)) = (times[0].parse::<u64>(), times[1].parse::<u64>()) {
        //         let mut timer_values = timer_values_clone.lock().unwrap();
        //         *timer_values = (Duration::from_secs(on_time), Duration::from_secs(off_time));
        //         request.into_ok_response()?
        //             .write_all("Timer values updated".into())?;
        //     } else {
        //         response.bad_request("Invalid timer values".into())?;
        //     }
        // } else {
        //     response.bad_request("Invalid input format. Use 'on_time,off_time'".into())?;
        // }
        // Ok(())
    // })?;

    Ok(())
}
