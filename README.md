# Cosmic-iio

Reads iio-sensor-proxy accelerometer orientation changes and transforms display in COSMIC accordingly

## Requirements

Have iio-sensor-proxy installed and running

## Installation

```sh
just build-release
sudo just install
just start
```

to uninstall:

```sh
just stop
sudo just uninstall
```

# TODO

- Arch and Deb packages
- Handle touch-screen rotation (needs cosmic support)
- More robust display output selection (don't want to transform all connected display, only the display on the device, but can't identify this display yet)

## Contributers wanted

I'm not an expert by any stretch with regards to systemd, dbus, iio, cosmic or rust for that matter.
Please roast my code and submit pull-requests :)

## Prior art

<https://github.com/okeri/iio-sway>
<https://github.com/JeanSchoeller/iio-hyprland/>
