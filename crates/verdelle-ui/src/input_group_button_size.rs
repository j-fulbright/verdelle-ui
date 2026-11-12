/// Size of an [`crate::InputGroupButton`]. The same set as shadcn/ui's.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputGroupButtonSize {
    /// 24px tall with a text label.
    #[default]
    Xs,
    /// 28px tall with a text label.
    Sm,
    /// A 24px square, for an icon.
    IconXs,
    /// A 32px square, for an icon.
    IconSm,
}

impl InputGroupButtonSize {
    pub fn as_str(self) -> &'static str {
        match self {
            InputGroupButtonSize::Xs => "xs",
            InputGroupButtonSize::Sm => "sm",
            InputGroupButtonSize::IconXs => "icon-xs",
            InputGroupButtonSize::IconSm => "icon-sm",
        }
    }
}
