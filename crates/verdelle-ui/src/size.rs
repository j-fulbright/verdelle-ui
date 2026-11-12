/// Component size, shared by every sized component.
///
/// The `data-size` attribute carries it to CSS; sizes the CSS does not style
/// (for example `Medium`) fall through to the base rule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Smallest (24px tall on a Button).
    XSmall,
    /// Small (28px on a Button).
    Small,
    /// Medium, the default (32px on a Button).
    #[default]
    Medium,
    /// Large (36px on a Button).
    Large,
}

impl Size {
    /// The value of the `data-size` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            Size::XSmall => "xs",
            Size::Small => "sm",
            Size::Medium => "md",
            Size::Large => "lg",
        }
    }
}
