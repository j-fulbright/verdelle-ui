/// Which shell [`crate::AppShell`] draws the menu in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShellLayout {
    /// A column beside the page ([`crate::SidebarShell`]); a sheet on small screens.
    #[default]
    Sidebar,
    /// A bar above the page ([`crate::TopBarShell`]); one menu button on small screens.
    TopBar,
}
