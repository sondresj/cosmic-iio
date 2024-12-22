use std::fmt::Display;

use super::randr::WlTransform;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Transform {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    // TODO: Handle these?
    // Flipped,
    // Flipped90,
    // Flipped180,
    // Flipped270
    #[default]
    Unknown,
}

impl Transform {
    pub fn from_transform(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "rotate90" => Self::Rotate90,
            "rotate180" => Self::Rotate180,
            "rotate270" => Self::Rotate270,
            _ => Self::Unknown,
        }
    }

    pub fn from_orientation(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "left-up" => Self::Rotate90,
            "bottom-up" => Self::Rotate180,
            "right-up" => Self::Rotate270,
            _ => Self::Unknown,
        }
    }
}

pub const fn to_transform_string(t: Transform) -> &'static str {
    match t {
        Transform::Rotate90 => "rotate90",
        Transform::Rotate180 => "rotate180",
        Transform::Rotate270 => "rotate270",
        _ => "normal",
    }
}

impl From<WlTransform> for Transform {
    fn from(value: WlTransform) -> Self {
        match value {
            WlTransform::Normal => Self::Normal,
            WlTransform::_90 => Self::Rotate90,
            WlTransform::_180 => Self::Rotate180,
            WlTransform::_270 => Self::Rotate270,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct ConvertError(pub String);
impl Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl std::error::Error for ConvertError {}

impl TryFrom<Transform> for WlTransform {
    type Error = ConvertError;

    fn try_from(value: Transform) -> Result<Self, Self::Error> {
        match value {
            Transform::Normal => Ok(Self::Normal),
            Transform::Rotate90 => Ok(Self::_90),
            Transform::Rotate180 => Ok(Self::_180),
            Transform::Rotate270 => Ok(Self::_270),
            Transform::Unknown => Err(ConvertError(
                "Cannot convert Unknown to a wl_output Transform".into(),
            )),
        }
    }
}

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(to_transform_string(*self))
    }
}
