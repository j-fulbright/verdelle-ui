/// Where a [`crate::ToastViewport`] sits on the screen. The same six places as Sonner's
/// `position`. Toasts swipe away towards the nearest side edge (`*-center` positions swipe
/// towards their top or bottom edge).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastPosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl ToastPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            ToastPosition::TopLeft => "top-left",
            ToastPosition::TopCenter => "top-center",
            ToastPosition::TopRight => "top-right",
            ToastPosition::BottomLeft => "bottom-left",
            ToastPosition::BottomCenter => "bottom-center",
            ToastPosition::BottomRight => "bottom-right",
        }
    }

    /// The swipe direction that dismisses a toast shown at this position.
    pub(crate) fn swipe_direction(self) -> &'static str {
        match self {
            ToastPosition::TopLeft | ToastPosition::BottomLeft => "left",
            ToastPosition::TopRight | ToastPosition::BottomRight => "right",
            ToastPosition::TopCenter => "up",
            ToastPosition::BottomCenter => "down",
        }
    }
}
