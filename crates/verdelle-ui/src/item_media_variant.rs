/// How an [`crate::ItemMedia`] is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ItemMediaVariant {
    /// Bare content at its own size.
    #[default]
    Default,
    /// For a 16px icon.
    Icon,
    /// A square, rounded, clipped box (40px, 32px in a small item, 24px in an extra small one)
    /// for an `<img>` or an avatar.
    Image,
}

impl ItemMediaVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            ItemMediaVariant::Default => "default",
            ItemMediaVariant::Icon => "icon",
            ItemMediaVariant::Image => "image",
        }
    }
}
