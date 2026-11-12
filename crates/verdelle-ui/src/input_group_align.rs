/// Where an [`crate::InputGroupAddon`] sits in its [`crate::InputGroup`]. The addon's place in
/// the markup does not matter; this decides it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupAlign {
    /// Before the text, on the same line.
    #[default]
    InlineStart,
    /// After the text, on the same line.
    InlineEnd,
    /// A full-width row above the text.
    BlockStart,
    /// A full-width row below the text.
    BlockEnd,
}

impl InputGroupAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupAlign::InlineStart => "inline-start",
            InputGroupAlign::InlineEnd => "inline-end",
            InputGroupAlign::BlockStart => "block-start",
            InputGroupAlign::BlockEnd => "block-end",
        }
    }
}
