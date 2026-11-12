/// The look of a [`crate::Sidebar`].
///
/// `Sidebar` is a plain panel against the page edge with a border; `Floating` is inset from
/// the edges with a ring, rounded corners and a shadow; `Inset` is also inset, and its
/// [`crate::SidebarInset`] sibling is drawn as a rounded card next to it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}
