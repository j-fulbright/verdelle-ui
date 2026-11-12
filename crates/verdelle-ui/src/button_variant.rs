/// Visual variant of a [`crate::Button`]. The same set as shadcn/ui's Button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Solid primary colour.
    #[default]
    Default,
    /// Bordered, with the page background.
    Outline,
    /// Muted fill.
    Secondary,
    /// No fill until hovered.
    Ghost,
    /// A tinted fill with destructive-coloured text.
    Destructive,
    /// Looks like a text link.
    Link,
}

impl ButtonVariant {
    /// The value of the `data-variant` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonVariant::Default => "default",
            ButtonVariant::Outline => "outline",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Destructive => "destructive",
            ButtonVariant::Link => "link",
        }
    }
}
