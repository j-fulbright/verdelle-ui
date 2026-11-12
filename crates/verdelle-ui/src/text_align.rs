/// Horizontal alignment of a table column's content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign {
    /// `None` for the default, so the attribute is omitted.
    pub fn attr(self) -> Option<&'static str> {
        match self {
            TextAlign::Left => None,
            TextAlign::Center => Some("center"),
            TextAlign::Right => Some("right"),
        }
    }
}
