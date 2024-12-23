mod core;

use core::{accelerometer::Accelerometer, randr::CosmicRandrClient, TerminationSignal};
use std::time::Duration;

// TODO: Is this correct for all laptops with accelerometer?
const OUTPUT_DISPLAY: &str = "eDP-1";

fn main() {
    let mut randr = CosmicRandrClient::connect().expect("Unable to connect to wayland backend");
    println!("Connected to wayland backend");

    let accelerometer = Accelerometer::connect()
        .expect("Unable to connect to system dbus")
        .claim()
        .expect("Unable to claim accelerometer, is iio-sensor-proxy running?");

    println!("Claimed accelerometer");

    let _ = accelerometer
        .get_transform()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .and_then(|t| randr.apply_transform(OUTPUT_DISPLAY, *t))
        .inspect_err(|e| eprintln!("{e:?}"));

    let terminator = TerminationSignal::new()
        .register()
        .expect("Unable to register signal hooks");

    while !terminator.should_terminate() {
        if let Ok(did_process) = accelerometer
            .poll_orientation_changed(Duration::from_millis(200))
            .inspect_err(|e| eprintln!("Error processing messages on dbus: {e:?}"))
        {
            if did_process {
                if let Ok(wlt) = accelerometer.get_transform() {
                    let result = randr
                        .apply_transform(OUTPUT_DISPLAY, *wlt)
                        .inspect_err(|why| {
                            eprintln!("Could not set transform {wlt}. {why}");
                        });
                    if result.is_ok() {
                        println!("Transformed display {OUTPUT_DISPLAY} with transform {wlt}",);
                    }
                } else {
                    println!("Received unknown orientation");
                }
            }
        } else {
            break;
        }
    }
    println!("Interrupted, exiting..");
}
