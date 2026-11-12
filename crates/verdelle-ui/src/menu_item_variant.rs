/// Visual variant of a [`crate::DropdownMenuItem`]. The same set as shadcn/ui's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuItemVariant {
    /// Normal text.
    #[default]
    Default,
    /// Destructive-coloured text and icon, with a tinted highlight.
    Destructive,
}

impl MenuItemVariant {
    /// The value of the `data-variant` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            MenuItemVariant::Default => "default",
            MenuItemVariant::Destructive => "destructive",
        }
    }
}
