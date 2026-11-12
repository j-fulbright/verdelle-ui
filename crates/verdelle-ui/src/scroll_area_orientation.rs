/// Which directions a [`crate::ScrollArea`] scrolls and draws a bar for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollAreaOrientation {
    /// Up and down; wide content is clipped. shadcn's default.
    #[default]
    Vertical,
    /// Left and right; tall content is clipped.
    Horizontal,
    /// Both, with a corner where the bars meet.
    Both,
}

impl ScrollAreaOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            ScrollAreaOrientation::Vertical => "vertical",
            ScrollAreaOrientation::Horizontal => "horizontal",
            ScrollAreaOrientation::Both => "both",
        }
    }

    pub(crate) fn has_vertical(self) -> bool {
        !matches!(self, ScrollAreaOrientation::Horizontal)
    }

    pub(crate) fn has_horizontal(self) -> bool {
        !matches!(self, ScrollAreaOrientation::Vertical)
    }
}

#[cfg(test)]
mod tests {
    use super::ScrollAreaOrientation::*;

    #[test]
    fn each_orientation_draws_the_right_bars() {
        assert!(Vertical.has_vertical() && !Vertical.has_horizontal());
        assert!(!Horizontal.has_vertical() && Horizontal.has_horizontal());
        assert!(Both.has_vertical() && Both.has_horizontal());
    }
}
