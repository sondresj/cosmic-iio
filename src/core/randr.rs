use std::process::Command;

use kdl::KdlDocument;

use super::transform::{to_transform_string, Transform};

#[derive(Debug)]
pub struct OutputDisplay {
    pub output: String,
    pub width: String,
    pub height: String,
    pub transform: Transform,
}

pub fn get_output(identifier: &str) -> OutputDisplay {
    let output = Command::new("cosmic-randr")
        .args(["list", "--kdl"])
        .output()
        .expect("cosmic-randr --list --kdl call failed");
    let kdl_doc = ::core::str::from_utf8(&output.stdout)
        .expect("invalid output from cosmic-randr")
        .parse::<KdlDocument>()
        .expect("invalid kdl ouput from cosmic-randr");

    let first_display = kdl_doc
        .nodes()
        .iter()
        .find(|n| {
            n.get(0)
                .is_some_and(|arg| arg.value().as_string().is_some_and(|s| s.eq(identifier)))
        })
        .expect("Expected a display with identifier eDP-1");
    let display_props = first_display
        .children()
        .expect("Expected properties for display");
    let current_mode = display_props
        .get("modes")
        .and_then(|modes| modes.children())
        .and_then(|modes| modes.nodes().iter().find(|n| n.get("current").is_some()))
        .expect("Expected to find a current mode for display");
    let width: String = current_mode.get(0).expect("..").value().to_string();
    let height: String = current_mode.get(1).expect("..").value().to_string();
    let transform_str: &str = display_props
        .get("transform")
        .and_then(|p| p.get(0))
        .and_then(|v| v.value().as_string())
        .expect("Expected display to have a transform property");

    OutputDisplay {
        width,
        height,
        output: identifier.into(),
        transform: Transform::from_transform(transform_str),
    }
}

pub fn set_transform(display: &mut OutputDisplay, transform: Transform) -> bool {
    if transform == Transform::Unknown || display.transform == transform {
        return true;
    }
    let status = Command::new("cosmic-randr")
        .args([
            "mode",
            "--transform",
            to_transform_string(transform),
            display.output.as_str(),
            display.width.as_str(),
            display.height.as_str(),
        ])
        .status()
        .expect("cosmic-randr call failed");

    if status.success() {
        display.transform = transform;
    }
    status.success()
}
