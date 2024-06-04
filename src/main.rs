use esp_idf_svc::{
    hal::{
        gpio::{PinDriver},
        prelude::*,
    },
    http::server::EspHttpServer,
};
use embedded_svc::{http::Method, io::Write};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::wifi::{
    self, 
    EspWifi, 
    AccessPointConfiguration
};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    // Link some ESP-IDF patches to the executable to make the rust code work.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();


    log::info!("Setup Pheripherals...");
    let peripherals = Peripherals::take().unwrap();

    log::info!("Setup gpiois...");
    let mut io_on = PinDriver::output(peripherals.pins.gpio2)?;
    let mut io_off = PinDriver::output(peripherals.pins.gpio3)?;

    let sys_loop = EspSystemEventLoop::take()?;

    log::info!("Initialize NVS in an UNSAFE call!!");
    unsafe {
        esp_idf_sys::nvs_flash_init();
    }

    log::info!("Setup wifi...");
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), None)?;

    log::info!("Setup access point...");
    let ap_config = embedded_svc::wifi::Configuration::AccessPoint(
        AccessPointConfiguration {
            ssid: "esp32".try_into().or(Err(anyhow!("Invalid SSID config.")))?,
            password: "12345678".try_into().or(Err(anyhow!("Invalid SSID config.")))?,
            channel: 1,
            secondary_channel: Some(2),
            auth_method: wifi::AuthMethod::WPA2Personal,
            ssid_hidden: false,
            max_connections: 4,
            ..Default::default()
        }
    );
    wifi.set_configuration(&ap_config)?;

    log::info!("Start wifi...");
    wifi.start()?;

    log::info!("Setup webserver...");
    let mut server = EspHttpServer::new(&Default::default())?;

    let timer_values = Arc::new(Mutex::new((Duration::from_secs(240), Duration::from_secs(480)))); 
    // Default: 4 minutes on, 8 minutes off
    let timer_values_clone = Arc::clone(&timer_values);

    // Add default route
    log::info!("Setup routes...");
    server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())
    })?;
    server.fn_handler("/update", Method::Post, move |request| {
        log::info!("TODO: Read values from POST request");
        // let body = request.into_body();
        // let html = "index_html()";
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())
        // request.into_ok_response()?.write_all(html.as_bytes())
    })?;

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

    log::info!("Application started");
    loop {
        log::info!("Turn on");
        io_on.set_high()?;
        FreeRtos::delay_ms(500);
        io_on.set_low()?;

        // turn off after 4 minutes
        FreeRtos::delay_ms(60000);

        log::info!("Turn off");
        io_off.set_high()?;
        FreeRtos::delay_ms(500);
        io_off.set_low()?;

        // turn on after 8 minutes
        FreeRtos::delay_ms(60000);
    }
}

fn index_html() -> String {
        format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>automatic timer</title>
    </head>

	<body>

	<p>Set the timeouts for the ON and OFF State:</p>
	<form action="/update" method="post">
	  <label for="on">ON (minutes)</label>
	  <input type="number" id="on" name="on" min="1" max="59" value="4">
	  <br / >
	  <label for="off">OFF (minutes)</label>
	  <input type="number" id="off" name="off" min="1" max="59" value="8">
	  <input type="submit">
	</form>
	</body>
</html>
"#,
    )
}
