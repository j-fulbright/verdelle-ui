/// The edge a [`crate::Drawer`] comes from, and so the way it is swiped to dismiss it (a bottom
/// drawer is pulled down, a right one to the right).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawerDirection {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}

impl DrawerDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            DrawerDirection::Top => "top",
            DrawerDirection::Right => "right",
            DrawerDirection::Bottom => "bottom",
            DrawerDirection::Left => "left",
        }
    }

    /// Whether the drawer moves along the vertical axis (top and bottom).
    pub fn is_vertical(self) -> bool {
        matches!(self, DrawerDirection::Top | DrawerDirection::Bottom)
    }

    /// `1.0` when closing means the finger moves toward larger coordinates (down, right),
    /// `-1.0` when toward smaller ones (up, left).
    pub(crate) fn close_sign(self) -> f64 {
        match self {
            DrawerDirection::Bottom | DrawerDirection::Right => 1.0,
            DrawerDirection::Top | DrawerDirection::Left => -1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axes_and_signs() {
        assert!(DrawerDirection::Bottom.is_vertical());
        assert!(!DrawerDirection::Left.is_vertical());
        assert_eq!(DrawerDirection::Bottom.close_sign(), 1.0);
        assert_eq!(DrawerDirection::Top.close_sign(), -1.0);
        assert_eq!(DrawerDirection::Right.close_sign(), 1.0);
        assert_eq!(DrawerDirection::Left.close_sign(), -1.0);
        assert_eq!(DrawerDirection::default(), DrawerDirection::Bottom);
    }
}
