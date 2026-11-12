//! Scales: maps from data values to pixel positions.

/// A straight-line map from a domain to a range (either may run backwards, as a y axis does).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearScale {
    pub d0: f64,
    pub d1: f64,
    pub r0: f64,
    pub r1: f64,
}

impl LinearScale {
    pub fn new(domain: (f64, f64), range: (f64, f64)) -> Self {
        Self {
            d0: domain.0,
            d1: domain.1,
            r0: range.0,
            r1: range.1,
        }
    }

    /// The range position of `v`. A zero-width domain maps everything to the middle of the range.
    pub fn apply(&self, v: f64) -> f64 {
        if self.d1 == self.d0 {
            return (self.r0 + self.r1) / 2.0;
        }
        self.r0 + (v - self.d0) / (self.d1 - self.d0) * (self.r1 - self.r0)
    }

    /// The domain value at range position `r`.
    pub fn invert(&self, r: f64) -> f64 {
        if self.r1 == self.r0 {
            return self.d0;
        }
        self.d0 + (r - self.r0) / (self.r1 - self.r0) * (self.d1 - self.d0)
    }
}

/// `n` equal bands across a range, with space between and around them (a bar chart's categories).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BandScale {
    n: usize,
    r0: f64,
    r1: f64,
    step: f64,
    start: f64,
    bandwidth: f64,
}

impl BandScale {
    /// `padding_inner` is the gap between bands (0 to 1 of a step) and `padding_outer` the gap at
    /// both ends (in steps), like d3's `scaleBand`. The range may run backwards.
    pub fn new(n: usize, range: (f64, f64), padding_inner: f64, padding_outer: f64) -> Self {
        let inner = padding_inner.clamp(0.0, 1.0);
        let outer = padding_outer.max(0.0);
        let span = range.1 - range.0;
        let count = n as f64;
        let denom = (count - inner + 2.0 * outer).max(1.0);
        let step = span / denom;
        let start = range.0 + (span - step * (count - inner)) / 2.0;
        Self {
            n,
            r0: range.0,
            r1: range.1,
            step,
            start,
            bandwidth: step * (1.0 - inner),
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// The width of a band (negative on a backwards range; use [`BandScale::extent`] to draw).
    pub fn bandwidth(&self) -> f64 {
        self.bandwidth
    }

    pub fn step(&self) -> f64 {
        self.step
    }

    /// Where band `i` starts.
    pub fn start(&self, i: usize) -> f64 {
        self.start + self.step * i as f64
    }

    /// The middle of band `i`.
    pub fn center(&self, i: usize) -> f64 {
        self.start(i) + self.bandwidth / 2.0
    }

    /// Band `i` as `(low, width)` with a positive width whichever way the range runs.
    pub fn extent(&self, i: usize) -> (f64, f64) {
        let a = self.start(i);
        let b = a + self.bandwidth;
        (a.min(b), (b - a).abs())
    }

    /// The band whose slot (the range cut in `n` equal parts) holds position `r`, clamped to the ends;
    /// `None` for no bands. Hover uses this, so the gaps between bars belong to the nearest bar.
    pub fn index_at(&self, r: f64) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        let (lo, hi) = (self.r0.min(self.r1), self.r0.max(self.r1));
        let slot = (hi - lo) / self.n as f64;
        if slot <= 0.0 {
            return Some(0);
        }
        let mut i = ((r - lo) / slot).floor();
        if self.r1 < self.r0 {
            i = self.n as f64 - 1.0 - i;
        }
        Some(i.clamp(0.0, (self.n - 1) as f64) as usize)
    }
}

/// `n` points spread evenly over a range (a line chart's categories). `padding` is the space before
/// the first and after the last point as a fraction of the step: 0 puts them on the edges, 0.5 makes
/// them sit in band centres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointScale {
    n: usize,
    step: f64,
    start: f64,
}

impl PointScale {
    pub fn new(n: usize, range: (f64, f64), padding: f64) -> Self {
        let pad = padding.max(0.0);
        let span = range.1 - range.0;
        let gaps = (n as f64 - 1.0).max(0.0) + 2.0 * pad;
        let step = if gaps > 0.0 { span / gaps } else { 0.0 };
        // A single point sits in the middle.
        let start = if n <= 1 {
            range.0 + span / 2.0
        } else {
            range.0 + pad * step
        };
        Self { n, step, start }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// The distance between neighbouring points.
    pub fn step(&self) -> f64 {
        self.step
    }

    pub fn position(&self, i: usize) -> f64 {
        self.start + self.step * i as f64
    }

    /// The point nearest to `r`.
    pub fn index_at(&self, r: f64) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        if self.step == 0.0 {
            return Some(0);
        }
        let i = ((r - self.start) / self.step).round();
        Some(i.clamp(0.0, (self.n - 1) as f64) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn linear_maps_and_inverts_including_a_flipped_range() {
        let s = LinearScale::new((0.0, 100.0), (200.0, 0.0));
        assert!(close(s.apply(0.0), 200.0));
        assert!(close(s.apply(100.0), 0.0));
        assert!(close(s.apply(25.0), 150.0));
        assert!(close(s.invert(150.0), 25.0));
        // Values outside the domain extrapolate rather than clamp.
        assert!(close(s.apply(150.0), -100.0));
    }

    #[test]
    fn linear_with_an_empty_domain_is_the_middle() {
        let s = LinearScale::new((5.0, 5.0), (0.0, 100.0));
        assert!(close(s.apply(5.0), 50.0));
    }

    #[test]
    fn bands_fill_the_range_with_padding() {
        let s = BandScale::new(4, (0.0, 400.0), 0.0, 0.0);
        assert!(close(s.bandwidth(), 100.0));
        assert!(close(s.start(2), 200.0));
        assert!(close(s.center(0), 50.0));

        let p = BandScale::new(3, (0.0, 300.0), 0.2, 0.1);
        // step = 300 / (3 - 0.2 + 0.2) = 100; bandwidth 80; the bands are centred in the range.
        assert!(close(p.step(), 100.0));
        assert!(close(p.bandwidth(), 80.0));
        let first = p.start(0);
        let last_end = p.start(2) + p.bandwidth();
        assert!(close(first, 300.0 - last_end), "{first} {last_end}");
    }

    #[test]
    fn band_extent_is_positive_on_a_flipped_range() {
        let s = BandScale::new(2, (200.0, 0.0), 0.0, 0.0);
        let (low, width) = s.extent(0);
        assert!(close(low, 100.0) && close(width, 100.0));
        let (low, _) = s.extent(1);
        assert!(close(low, 0.0));
    }

    #[test]
    fn band_hover_picks_the_slot() {
        let s = BandScale::new(4, (0.0, 400.0), 0.3, 0.15);
        assert_eq!(s.index_at(-50.0), Some(0));
        assert_eq!(s.index_at(99.0), Some(0));
        assert_eq!(s.index_at(101.0), Some(1));
        assert_eq!(s.index_at(999.0), Some(3));
        assert_eq!(BandScale::new(0, (0.0, 10.0), 0.0, 0.0).index_at(1.0), None);
    }

    #[test]
    fn band_hover_works_on_a_flipped_range() {
        // A horizontal bar chart's category axis runs from the top down in data order, but a flipped
        // range puts the first band at the bottom.
        let s = BandScale::new(4, (400.0, 0.0), 0.0, 0.0);
        assert_eq!(s.index_at(10.0), Some(3));
        assert_eq!(s.index_at(390.0), Some(0));
    }

    #[test]
    fn points_sit_on_the_edges_or_in_band_centres() {
        let edge = PointScale::new(5, (0.0, 400.0), 0.0);
        assert!(close(edge.position(0), 0.0));
        assert!(close(edge.position(4), 400.0));
        let centred = PointScale::new(4, (0.0, 400.0), 0.5);
        assert!(close(centred.position(0), 50.0));
        assert!(close(centred.position(3), 350.0));
        let one = PointScale::new(1, (0.0, 200.0), 0.0);
        assert!(close(one.position(0), 100.0));
    }

    #[test]
    fn points_find_the_nearest() {
        let s = PointScale::new(5, (0.0, 400.0), 0.0);
        assert_eq!(s.index_at(10.0), Some(0));
        assert_eq!(s.index_at(160.0), Some(2));
        assert_eq!(s.index_at(5000.0), Some(4));
        assert_eq!(PointScale::new(0, (0.0, 1.0), 0.0).index_at(0.5), None);
    }
}
