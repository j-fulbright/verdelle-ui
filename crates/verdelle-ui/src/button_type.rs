/// The HTML `type` of a [`crate::Button`] that renders a `<button>`: what pressing it does inside a
/// `<form>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonType {
    /// Does nothing by itself (the default, so a button in a form does not submit it by accident).
    #[default]
    Button,
    /// Submits the form it is in.
    Submit,
    /// Resets the form it is in.
    Reset,
}

impl ButtonType {
    /// The value of the `type` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}
