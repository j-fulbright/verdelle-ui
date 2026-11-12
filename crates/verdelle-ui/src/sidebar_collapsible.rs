/// What a [`crate::Sidebar`] does when it is collapsed.
///
/// `Offcanvas` slides out of view; `Icon` shrinks to a rail of icons (3rem wide, or a little
/// more for the floating and inset variants); `None` never collapses and is always shown,
/// also on small screens, where the other two turn into a [`crate::Sheet`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarCollapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl SidebarCollapsible {
    pub fn as_str(self) -> &'static str {
        match self {
            SidebarCollapsible::Offcanvas => "offcanvas",
            SidebarCollapsible::Icon => "icon",
            SidebarCollapsible::None => "none",
        }
    }
}
