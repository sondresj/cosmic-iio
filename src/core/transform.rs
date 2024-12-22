use std::fmt::Display;

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

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(to_transform_string(*self))
    }
}
