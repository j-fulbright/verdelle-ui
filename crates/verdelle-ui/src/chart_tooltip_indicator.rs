/// How a tooltip row marks its series colour (shadcn's `indicator`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartTooltipIndicator {
    /// A small rounded square. The default.
    #[default]
    Dot,
    /// A tall narrow bar.
    Line,
    /// A tall dashed bar.
    Dashed,
}

impl ChartTooltipIndicator {
    pub fn as_str(self) -> &'static str {
        match self {
            ChartTooltipIndicator::Dot => "dot",
            ChartTooltipIndicator::Line => "line",
            ChartTooltipIndicator::Dashed => "dashed",
        }
    }
}
