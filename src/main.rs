use esp_idf_svc::{
    hal::{
        gpio::{PinDriver},
        prelude::*,
    },
    http::server::EspHttpServer,
};
use embedded_svc::{http::Method, io::Write};
use std::sync::{Arc, Mutex};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_svc::wifi::{
    self,
    EspWifi,
    AccessPointConfiguration,
};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use anyhow::anyhow;
use std::str;

struct State {
    on_time_in_ms: u32,
    off_time_in_ms: u32,
}

impl State {
    fn new(on_time_in_ms: u32, off_time_in_ms: u32) -> Self {
        State { on_time_in_ms, off_time_in_ms }
    }
}

fn main() -> anyhow::Result<()> {
    // Link some ESP-IDF patches to the executable to make the rust code work.
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Setup Peripherals...");
    let peripherals = Peripherals::take().unwrap();

    log::info!("Setup GPIOs...");
    let mut io_on = PinDriver::output(peripherals.pins.gpio1)?;
    let mut io_off = PinDriver::output(peripherals.pins.gpio2)?;
    let mut io_power = PinDriver::output(peripherals.pins.gpio42)?;

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

    let state = Arc::new(Mutex::new(State::new(10 * 1000, 10 * 1000)));
    let state_clone = Arc::clone(&state);

    // Add default route
    log::info!("Setup routes...");
    server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())
    })?;
    server.fn_handler("/update", Method::Post, move |mut request| {
        let mut state = state_clone.lock().unwrap();

        let mut buffer: [u8; 16] = [0; 16];
        request.read(&mut buffer)?;

        // convert buffer to string
        let input = str::from_utf8(&buffer).unwrap();

        // trim \0 values and split post values
        let times: Vec<&str> = input.trim_matches(char::from(0)).split('&').collect();

        let on_str = times[0].replace("on=", "");
        let off_str = times[1].replace("off=", "");
        log::info!("on: {on_str}, off: {off_str}");

        // parse values to INT
        let result: &str = if let (Ok(on_time_in_seconds), Ok(off_time_in_seconds)) = (on_str.parse::<u32>(), off_str.parse::<u32>()) {
            *state = State::new(on_time_in_seconds * 1000, off_time_in_seconds * 1000);

            "timer values updated.."
        } else {
            log::error!("Invalid timer values!");
            "Invalid timer values!"
        };

        let html = post_html(result);
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())
    })?;

    log::info!("Application started");
    loop {
        let on_time = state.lock().unwrap().on_time_in_ms;
        let off_time = state.lock().unwrap().off_time_in_ms;
        log::info!("Turn on {}", on_time);
        io_power.set_high()?;
        io_on.set_high()?;
        FreeRtos::delay_ms(500);
        io_on.set_low()?;

        // turn off after 4 minutes
        FreeRtos::delay_ms(on_time);

        log::info!("Turn off {}", off_time);
        io_power.set_low()?;
        io_off.set_high()?;
        FreeRtos::delay_ms(500);
        io_off.set_low()?;

        // turn on after 8 minutes
        FreeRtos::delay_ms(off_time);
    }
}

fn index_html() -> String {
    r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>automatic timer</title>
    </head>

	<body>

	<p>Set the timeouts for the ON and OFF State:</p>
	<form action="/update" method="post">
	  <label for="on">ON (seconds)</label>
	  <input type="number" id="on" name="on" min="1" max="59">
	  <br / >
	  <label for="off">OFF (seconds)</label>
	  <input type="number" id="off" name="off" min="1" max="59">
	  <br / >
	  <input type="submit">
	</form>
	</body>
</html>
"#.to_string()
}

fn post_html(result: &str) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>automatic timer</title>
        <meta http-equiv="refresh" content="5; url=/" />
    </head>

	<body>

	<p>{}</p>
	</body>
</html>
"#,
        result
    )
}
