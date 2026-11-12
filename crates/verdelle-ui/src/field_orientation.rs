/// How a [`crate::Field`] lays out its label, control and text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldOrientation {
    /// Label above the control (the default).
    #[default]
    Vertical,
    /// Label beside the control, as for a checkbox or a switch.
    Horizontal,
    /// Vertical in a narrow [`crate::FieldGroup`], horizontal once the group is 28rem wide.
    Responsive,
}

impl FieldOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            FieldOrientation::Vertical => "vertical",
            FieldOrientation::Horizontal => "horizontal",
            FieldOrientation::Responsive => "responsive",
        }
    }
}
