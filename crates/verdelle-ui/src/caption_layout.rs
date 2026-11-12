/// How a [`crate::Calendar`] shows the month and year above each month.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CaptionLayout {
    /// The text `November 2026`; move with the arrow buttons.
    #[default]
    Label,
    /// A month menu and a year menu.
    Dropdown,
    /// A month menu and the year as text.
    DropdownMonths,
    /// The month as text and a year menu.
    DropdownYears,
}

impl CaptionLayout {
    pub(crate) fn has_month_menu(self) -> bool {
        matches!(
            self,
            CaptionLayout::Dropdown | CaptionLayout::DropdownMonths
        )
    }

    pub(crate) fn has_year_menu(self) -> bool {
        matches!(self, CaptionLayout::Dropdown | CaptionLayout::DropdownYears)
    }
}
