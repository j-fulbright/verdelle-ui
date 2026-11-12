/// The shape of a radar chart's grid rings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadarGrid {
    /// Straight edges between the axes. The default.
    #[default]
    Polygon,
    /// Circles.
    Circle,
}
