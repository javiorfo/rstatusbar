# rstatusbar
*Configurable statusbar in Rust for Xorg server using xsetroot*

## Caveats
- Dependencies: `rust`, `xorg-xsetroot`
- This library has been developed on and for Linux following open source philosophy.

## Installation
- Using cargo:
```bash
cargo install rstatusbar
```

- Using yay or paru (AUR Arch Linux):
```bash
paru -S rstatusbar
```

## Setup
- In your **~/.xinitrc** to start in every login
```bash
rstatusbar 2> rstatusbar.log &
```

## Overview
| Component | rstatusbar | NOTE |
| ------- | ------------- | ---- |
| CPU usage | :heavy_check_mark: | Percentage |
| RAM usage | :heavy_check_mark: | Percentage |
| TEMPERATURE | :heavy_check_mark: | Celcious |
| DISK USAGE | :heavy_check_mark: | Percentage |
| VOLUME LEVEL | :heavy_check_mark: | Level and Mute status |
| BLUETOOTH | :x: | |
| BATTERY LEVEL | :heavy_check_mark: | Percentage |
| CUSTOM SCRIPT | :heavy_check_mark: | Execute a custom script.sh |
| NETWORK STATUS | :heavy_check_mark: | Up or down |
| WEATHER | :heavy_check_mark: | Celcious, using [wttr](https://wttr.in/) |
| DATE | :heavy_check_mark: | Could be custimizable |

## Customizable
- By default the statusbar contains: **cpu usage, memory usage, temperature, disk usage, volume, network status and datetime**
- For a custom configuration put this file [config.toml](https://github.com/javiorfo/rstatusbar/blob/master/examples/config.toml) in your `~/.config/rstatusbar/config.toml` and edit it to change values or delete a component.
- Memory component example in config.toml:
```toml
[memory]
time = 1000  # Time in miliseconds defines how often the process runs
name = "RAM" # Name of the component. Could be empty => name = ""
icon = ""   # Icon of the component. Could be empty => icon = ""
```

## Screenshots

<img src="https://github.com/javiorfo/img/blob/master/xtatusbar/xtatusbar.png?raw=true" alt="rstatusbar" />

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G)
