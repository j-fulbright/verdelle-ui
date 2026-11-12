/// How a floating element lines up with its trigger along the side it opens on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    /// Edges meet at the start (left edge for top/bottom, top edge for left/right).
    Start,
    /// Centred on the trigger, the default.
    #[default]
    Center,
    /// Edges meet at the end.
    End,
}

impl Align {
    /// The value of the `data-align` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            Align::Start => "start",
            Align::Center => "center",
            Align::End => "end",
        }
    }
}
