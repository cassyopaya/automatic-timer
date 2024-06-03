# Automatic timer
Simple embedded application to switch an GPIO on/off based on an internal timer while running. The timeouts can be changed via an internal webserver.

## Prerequisites

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

## 🛠️  Build
```shell
cargo build
```


## ⚡Flash
```shell
espflash flash target/<mcu-target>/debug/<your-project-name>
```
