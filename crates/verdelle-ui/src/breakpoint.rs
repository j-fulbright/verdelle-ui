/// A viewport width breakpoint, with Tailwind's sizes, for hiding or restacking table columns on
/// narrow screens. "Below `Md`" means narrower than 768px.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Breakpoint {
    /// 640px.
    Sm,
    /// 768px.
    Md,
    /// 1024px.
    Lg,
    /// 1280px.
    Xl,
}

impl Breakpoint {
    /// The value of the `data-hide-below` / `data-stack-below` attribute the CSS matches.
    pub fn attr(self) -> &'static str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
        }
    }

    /// The width in CSS pixels at which the breakpoint starts.
    pub fn min_width(self) -> u32 {
        match self {
            Breakpoint::Sm => 640,
            Breakpoint::Md => 768,
            Breakpoint::Lg => 1024,
            Breakpoint::Xl => 1280,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_increase_and_attrs_are_distinct() {
        let all = [
            Breakpoint::Sm,
            Breakpoint::Md,
            Breakpoint::Lg,
            Breakpoint::Xl,
        ];
        for pair in all.windows(2) {
            assert!(pair[0].min_width() < pair[1].min_width());
            assert_ne!(pair[0].attr(), pair[1].attr());
        }
    }
}
