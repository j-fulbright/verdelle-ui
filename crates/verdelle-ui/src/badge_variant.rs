/// Visual variant of a [`crate::Badge`]. The same set as shadcn/ui's Badge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BadgeVariant {
    /// Solid primary colour.
    #[default]
    Default,
    /// Muted fill.
    Secondary,
    /// A tinted fill with destructive-coloured text.
    Destructive,
    /// Bordered, no fill.
    Outline,
}

impl BadgeVariant {
    /// The value of the `data-variant` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Secondary => "secondary",
            BadgeVariant::Destructive => "destructive",
            BadgeVariant::Outline => "outline",
        }
    }
}
