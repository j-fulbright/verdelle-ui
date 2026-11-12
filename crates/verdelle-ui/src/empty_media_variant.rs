/// How an [`crate::EmptyMedia`] is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EmptyMediaVariant {
    /// Bare: any content (an image, a stack of avatars) at its own size.
    #[default]
    Default,
    /// A 32px muted rounded tile centring a 16px icon.
    Icon,
}

impl EmptyMediaVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            EmptyMediaVariant::Default => "default",
            EmptyMediaVariant::Icon => "icon",
        }
    }
}
