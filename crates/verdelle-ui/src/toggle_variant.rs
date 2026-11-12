/// The look of a [`crate::Toggle`] or [`crate::ToggleGroup`]: shadcn's two variants.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToggleVariant {
    /// No border; only the pressed state is filled.
    #[default]
    Default,
    /// A border and a faint shadow.
    Outline,
}

impl ToggleVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            ToggleVariant::Default => "default",
            ToggleVariant::Outline => "outline",
        }
    }
}
