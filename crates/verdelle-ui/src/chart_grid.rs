/// Which grid lines a cartesian chart draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartGrid {
    /// Lines across the plot at each value tick (horizontal lines for upright bars and lines, vertical
    /// ones for horizontal bars). shadcn's `<CartesianGrid vertical={false} />`. The default.
    #[default]
    Value,
    /// Lines at each category.
    Category,
    /// Both.
    Both,
    /// No grid.
    None,
}

impl ChartGrid {
    pub(crate) fn has_value(self) -> bool {
        matches!(self, ChartGrid::Value | ChartGrid::Both)
    }

    pub(crate) fn has_category(self) -> bool {
        matches!(self, ChartGrid::Category | ChartGrid::Both)
    }
}
