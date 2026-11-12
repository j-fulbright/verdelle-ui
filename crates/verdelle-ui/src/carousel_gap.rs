/// The space between slides of a `Carousel`.
///
/// shadcn leaves it to classes on every slide (`pl-4` with `-ml-4` on the track); here one prop
/// on `Carousel` sets it and the CSS applies it to the track and to every slide.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CarouselGap {
    /// No space between slides.
    None,
    /// 8px.
    Small,
    /// 16px, shadcn's default.
    #[default]
    Medium,
    /// 24px.
    Large,
}

impl CarouselGap {
    pub fn as_str(self) -> &'static str {
        match self {
            CarouselGap::None => "none",
            CarouselGap::Small => "sm",
            CarouselGap::Medium => "md",
            CarouselGap::Large => "lg",
        }
    }
}
