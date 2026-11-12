/// Which way a bar chart's bars run.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ChartOrientation {
    /// Bars stand on a horizontal category axis. The default.
    #[default]
    Vertical,
    /// Bars lie on a vertical category axis and grow to the right (shadcn's `layout="vertical"`).
    Horizontal,
}
