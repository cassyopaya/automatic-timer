# Automatic timer
Simple embedded application to control GPIO based on an internal timer. 

> [Defaults: 10 seconds on, 10 seconds off]

Timeouts can be changed via web.  
The device provides an Access point for easy usage.

- GPIO2: Turn on for 500ms, wait for 20seconds
- GPIO3  Wait for 10seconds, turn on for 500ms, wait again for 10seconds
- GPIO42: Turn on for 10seconds, wait for 10seconds

## ☘️ Usage

1. Plugin the device
2. Connect to the Access Point named *ESP32*
3. Enter the access point password: *12345678*
4. Open the website: http://192.168.71.1 in a browser
5. Enter the timeouts you prefer and press save


## ⚙️ Development
### Prerequisites

[IDF Toolchain Setup](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/get-started/linux-macos-setup.html)

Install the matching IDF Version to build the 
```shell
mkdir -p ~/esp
cd ~/esp
git clone -b v5.1.3 --recursive https://github.com/espressif/esp-idf.git
```

Run the IDF setup
```shell
cd ~/esp/esp-idf
./install.sh esp32
```

### 🛠️ Build
```shell
cargo build
```

### ⚡ Flash
```shell
espflash flash target/<mcu-target>/debug/<your-project-name> --flash-size 8mb
```

### 🖥️ Monitor
```shell
espflash monitor
```
