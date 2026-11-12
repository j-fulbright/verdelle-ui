/// How the shell frames the page content (see [`crate::AppShell`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShellContent {
    /// Centred, at most 80rem wide, with 24px padding: the page is a column in the middle of the shell.
    #[default]
    Contained,
    /// The page fills the whole area beside the menu with no padding or width limit, so it can
    /// draw its own full-bleed background and layout. Use it while pages that were written
    /// without a shell are moved over one at a time.
    Full,
}

impl ShellContent {
    /// The `data-width` value the CSS reads; `None` for the default.
    pub(crate) fn attribute(self) -> Option<&'static str> {
        match self {
            ShellContent::Contained => None,
            ShellContent::Full => Some("full"),
        }
    }
}
