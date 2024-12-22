mod core;

use core::{
    accelerometer::Accelerometer,
    randr::{get_output, set_transform},
    TerminationSignal,
};
use std::time::Duration;

// TODO: Is this correct for all laptops with accelerometer?
const OUTPUT_DISPLAY: &str = "eDP-1";

fn main() {
    let mut output = get_output(OUTPUT_DISPLAY);
    println!("Targeting display {output:?} for mode transform");

    let accelerometer = Accelerometer::connect()
        .expect("Unable to connect to system dbus")
        .claim()
        .expect("Unable to claim accelerometer, is iio-sensors-proxy running?");

    println!("Claimed accelerometer");

    let initial_transform = accelerometer.get_transform();
    set_transform(&mut output, initial_transform);
    let terminator = TerminationSignal::new()
        .register()
        .expect("Unable to register signal hooks");

    while !terminator.should_terminate() {
        if let Ok(did_process) = accelerometer
            .poll_orientation_changed(Duration::from_millis(200))
            .inspect_err(|e| eprintln!("Error processing messages on dbus: {e:?}"))
        {
            if did_process {
                let transform = accelerometer.get_transform();
                let did_transform = set_transform(&mut output, transform);
                if did_transform {
                    println!(
                        "Transformed display {} with transform {transform}",
                        output.output
                    );
                } else {
                    println!("WARN: Could not set transform {transform}, cosmic-randr call was not a success");
                }
            }
        } else {
            break;
        }
    }
    println!("Interrupted, exiting..");
}
