/// The size of a [`crate::FieldLegend`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FieldLegendVariant {
    /// 16px, a heading for the set.
    #[default]
    Legend,
    /// 14px, the size of a label.
    Label,
}

impl FieldLegendVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            FieldLegendVariant::Legend => "legend",
            FieldLegendVariant::Label => "label",
        }
    }
}
