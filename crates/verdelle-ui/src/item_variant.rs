/// Visual variant of an [`crate::Item`]. The same set as shadcn/ui's Item.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemVariant {
    /// No fill, a transparent border.
    #[default]
    Default,
    /// A visible border.
    Outline,
    /// A faint muted fill.
    Muted,
}

impl ItemVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            ItemVariant::Default => "default",
            ItemVariant::Outline => "outline",
            ItemVariant::Muted => "muted",
        }
    }
}
