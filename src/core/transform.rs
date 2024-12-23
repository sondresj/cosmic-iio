use std::{fmt::Display, ops::Deref};

#[allow(clippy::module_name_repetitions)]
pub use wayland_client::protocol::wl_output::Transform as WlTransform;

#[derive(Copy, Clone, Debug)]
pub struct Transform(WlTransform);

impl Default for Transform {
    fn default() -> Self {
        Self(WlTransform::Normal)
    }
}

impl Deref for Transform {
    type Target = WlTransform;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Transform {
    // pub fn from_transform(value: &str) -> Self {
    //     match value {
    //         // "normal" => Self(WlTransform::Normal),
    //         "rotate90" => Self(WlTransform::_90),
    //         "rotate180" => Self(WlTransform::_180),
    //         "rotate270" => Self(WlTransform::_270),
    //         _ => Self(WlTransform::Normal),
    //     }
    // }

    pub fn from_orientation(value: &str) -> Self {
        match value {
            // "normal" => Self(WlTransform::Normal),
            "left-up" => Self(WlTransform::_90),
            "bottom-up" => Self(WlTransform::_180),
            "right-up" => Self(WlTransform::_270),
            _ => Self(WlTransform::Normal),
        }
    }
}

pub const fn to_transform_string(t: Transform) -> &'static str {
    match t.0 {
        WlTransform::_90 => "rotate90",
        WlTransform::_180 => "rotate180",
        WlTransform::_270 => "rotate270",
        _ => "normal",
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(to_transform_string(*self))
    }
}
