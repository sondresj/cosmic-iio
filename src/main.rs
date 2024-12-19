// TODO:
// - make systemd service of this thing

use core::str;
use kdl::KdlDocument;
use std::{
    fmt::Display,
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use dbus::{
    blocking::{BlockingSender, Connection},
    Error, Message,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Transform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    // flipped,
    // flipped90,
    // flipped180,
    // flipped270
    #[default]
    Unknown,
}

impl From<&str> for Transform {
    fn from(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "bottom-up" => Self::Rotate180,
            "left-up" => Self::Rotate90,
            "right-up" => Self::Rotate270,
            _ => Self::Unknown,
        }
    }
}

const fn transform_to_string(t: Transform) -> &'static str {
    match t {
        Transform::Rotate90 => "rotate90",
        Transform::Rotate180 => "rotate180",
        Transform::Rotate270 => "rotate270",
        _ => "normal",
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(transform_to_string(*self))
    }
}

// TODO: Is this correct for all laptops with accelerometer?
const OUTPUT_DISPLAY: &str = "eDP-1";
const DBUS_NAME: &str = "net.hadess.SensorProxy";
const DBUS_PATH: &str = "/net/hadess/SensorProxy";
const DBUS_PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";
const DBUS_SENSORS_INTERFACE: &str = "net.hadess.SensorProxy";

#[derive(Debug)]
struct OutputDisplay {
    output: String,
    width: String,
    height: String,
}

fn get_output() -> OutputDisplay {
    let output = Command::new("cosmic-randr")
        .args(["list", "--kdl"])
        .output()
        .expect("cosmic-randr --list --kdl call failed");
    let kdl_doc = str::from_utf8(&output.stdout)
        .expect("invalid output from cosmic-randr")
        .parse::<KdlDocument>()
        .expect("invalid kdl ouput from cosmic-randr");

    let first_display = kdl_doc
        .nodes()
        .iter()
        .find(|n| {
            n.get(0).is_some_and(|arg| {
                arg.value()
                    .as_string()
                    .is_some_and(|s| s.eq(OUTPUT_DISPLAY))
            })
        })
        .expect("Expected a display with identifier eDP-1");
    let current_mode = first_display
        .children()
        .and_then(|c| c.get("modes"))
        .and_then(|modes| modes.children())
        .and_then(|modes| modes.nodes().iter().find(|n| n.get("current").is_some()))
        .expect("Expected to find a current mode for display");
    let width: String = current_mode.get(0).expect("..").value().to_string();
    let height: String = current_mode.get(1).expect("..").value().to_string();

    OutputDisplay {
        width,
        height,
        output: "eDP-1".into(),
    }
}

fn connect_and_claim_accelerometer() -> Result<Connection, Error> {
    let connection = Connection::new_system()?;
    let message = Message::new_method_call(
        DBUS_NAME,
        DBUS_PATH,
        DBUS_SENSORS_INTERFACE,
        "ClaimAccelerometer", // TODO: Should ReleaseAccelerometer when shutting down?
    )
    .map_err(|msg| Error::new_custom("InvalidMessage", &msg))?;
    let _ = connection
        .send_with_reply_and_block(message, Duration::from_secs(10))?
        .as_result()?;
    Ok(connection)
}

fn get_transform(connection: &Connection) -> Transform {
    let reply = Message::new_method_call(DBUS_NAME, DBUS_PATH, DBUS_PROPERTIES_INTERFACE, "Get")
        .map(|msg| msg.append2(DBUS_SENSORS_INTERFACE, "AccelerometerOrientation"))
        .map_err(|msg| Error::new_custom("InvalidMessage", &msg))
        .and_then(|msg| connection.send_with_reply_and_block(msg, Duration::from_secs(10)));

    if let Ok(mut reply) = reply {
        if let Ok(checked_reply) = reply.as_result() {
            let first_arg = checked_reply
                .get1::<dbus::arg::Variant<&str>>()
                .map(|v| v.0);

            return first_arg.map_or(Transform::Unknown, std::convert::Into::into);
        }
    }
    println!("WARN: Failed to get AccelerometerOrientation response");

    Transform::Unknown
}

fn handle_transform(transform: Transform, display: &OutputDisplay) -> bool {
    if transform == Transform::Unknown {
        return true;
    }
    let status = Command::new("cosmic-randr")
        .args([
            "mode",
            "--transform",
            transform_to_string(transform),
            display.output.as_str(),
            display.width.as_str(),
            display.height.as_str(),
        ])
        .status()
        .expect("cosmic-randr call failed");
    status.success()
}

fn main() {
    let output = get_output();
    println!("Targeting display {output:?} for mode transform");

    let connection = connect_and_claim_accelerometer().expect("Unable to connect to dbus");
    println!("Claimed accelerometer");

    let initial_transform = get_transform(&connection);

    handle_transform(initial_transform, &output);

    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&terminate))
        .expect("Unable to register signal hook");
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate))
        .expect("Unable to register signal hook");

    while !terminate.load(Ordering::Relaxed) {
        if let Ok(did_process) = connection
            .process(Duration::from_millis(200))
            .inspect_err(|e| eprintln!("Error processing messages on dbus: {e:?}"))
        {
            if did_process {
                let transform = get_transform(&connection);
                let did_handle = handle_transform(transform, &output);
                if did_handle {
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
