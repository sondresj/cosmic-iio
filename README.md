y# Cosmic-iio

> [!NOTE]
> I'm too busy to maintain this service at the moment.
> Feel free to fork/steal/use/submit-prs, for the moment this solves my personal use-case, I may or may not get around to do bugfixes or other features

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
- ~~Handle touch-screen rotation (needs cosmic support)~~ seems to work as expected now
- More robust display output selection (don't want to transform all connected display, only the display on the device, but can't identify this display yet)

## Contributers wanted

I'm not an expert by any stretch with regards to systemd, dbus, iio, cosmic or rust for that matter.
Please roast my code and submit pull-requests :)

## Prior art

<https://github.com/okeri/iio-sway>
<https://github.com/JeanSchoeller/iio-hyprland/>
