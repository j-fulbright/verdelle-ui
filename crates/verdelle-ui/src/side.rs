/// Which side of its trigger a floating element (popover, menu, tooltip) opens on.
///
/// It is the preferred side: when there is no room the element flips to the opposite
/// side. The side it actually used is in the `data-side` attribute of the content.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger, the default.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

impl Side {
    /// The value of the `data-side` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Left => "left",
        }
    }
}
