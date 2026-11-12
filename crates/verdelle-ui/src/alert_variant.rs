/// Visual variant of an [`crate::Alert`]. `Default` and `Destructive` are shadcn/ui's;
/// `Success` and `Warning` are Verdelle extras (see `NAMING.md`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    /// The standard look.
    #[default]
    Default,
    /// For errors: destructive-coloured text.
    Destructive,
    /// For a completed or healthy state: green text and a faint green tint. Verdelle extra.
    Success,
    /// For a caution: amber text and a faint amber tint. Verdelle extra.
    Warning,
}

impl AlertVariant {
    /// The value of the `data-variant` attribute the CSS selects on.
    pub fn as_str(self) -> &'static str {
        match self {
            AlertVariant::Default => "default",
            AlertVariant::Destructive => "destructive",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
        }
    }

    /// The ARIA role: `alert` (announced at once) for destructive and warning alerts, `status`
    /// (announced politely) for the others.
    pub fn role(self) -> &'static str {
        match self {
            AlertVariant::Destructive | AlertVariant::Warning => "alert",
            AlertVariant::Default | AlertVariant::Success => "status",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urgent_variants_are_announced_at_once() {
        assert_eq!(AlertVariant::Destructive.role(), "alert");
        assert_eq!(AlertVariant::Warning.role(), "alert");
        assert_eq!(AlertVariant::Default.role(), "status");
        assert_eq!(AlertVariant::Success.role(), "status");
        assert_eq!(AlertVariant::Success.as_str(), "success");
    }
}
