//! The menu of an application as plain data, with no layout in it.
//!
//! An [`AppNav`] is a list of [`NavItem`]s; an item with children is a group. The same value
//! drives every shell ([`crate::SidebarShell`], [`crate::TopBarShell`]), so changing the
//! layout of an app means changing one component, not the menu. The current page comes in as a
//! path string, which keeps this free of any router: pass `use_location().pathname`.

use leptos::prelude::*;

/// How an item's `href` is compared with the current path.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavMatch {
    /// Only the same path is active (a trailing slash, a query and a hash are ignored).
    #[default]
    Exact,
    /// The path itself and everything below it is active: `/servers` matches `/servers/12`
    /// but not `/servers-old`. A root `/` only ever matches exactly.
    Prefix,
}

/// A path without its query, hash or trailing slash (`/a/b/?x=1#y` becomes `/a/b`).
pub fn normalize_path(path: &str) -> &str {
    let end = path.find(['?', '#']).unwrap_or(path.len());
    let path = &path[..end];
    if path.len() > 1 {
        path.trim_end_matches('/')
    } else {
        path
    }
}

/// Whether `href` is the current `path`, by `mode`.
pub fn path_matches(href: &str, path: &str, mode: NavMatch) -> bool {
    let (href, path) = (normalize_path(href), normalize_path(path));
    if href == path {
        return true;
    }
    match mode {
        NavMatch::Exact => false,
        NavMatch::Prefix => {
            href != "/"
                && path
                    .strip_prefix(href)
                    .is_some_and(|rest| rest.starts_with('/'))
        }
    }
}

/// A click on a menu link. With an `on_navigate` callback the browser's own navigation is
/// stopped and the callback gets the `href` (hook it to the router's `use_navigate`);
/// without one the anchor behaves normally, which a client-side router intercepts by itself.
/// Clicks with a modifier key are left to the browser (open in a new tab).
pub(crate) fn follow(
    on_navigate: Option<Callback<String>>,
    href: &str,
    ev: &leptos::ev::MouseEvent,
) {
    if let Some(cb) = on_navigate
        && ev.button() == 0
        && !(ev.ctrl_key() || ev.meta_key() || ev.shift_key() || ev.alt_key())
    {
        ev.prevent_default();
        cb.run(href.to_string());
    }
}

/// Go to `href` from something that is not an anchor (a dropdown item): the callback if there
/// is one, otherwise a full page load.
pub(crate) fn go(on_navigate: Option<Callback<String>>, href: &str) {
    match on_navigate {
        Some(cb) => cb.run(href.to_string()),
        None => {
            let _ = window().location().set_href(href);
        }
    }
}

/// One entry of the menu: a link, or (with children) a group of links.
///
/// Build it with the chained methods: `NavItem::new("Servers", "/servers").badge("24")`,
/// `NavItem::group("Admin").child(...)`. Menus are two levels deep (a group and its links);
/// deeper children are ignored by the shells.
#[derive(Clone)]
pub struct NavItem {
    pub label: String,
    /// Where it goes. A group without a link has none and only opens and closes.
    pub href: Option<String>,
    /// A 16px icon, drawn by the shells where there is room. Children take one too (the second
    /// menu level): the sidebar draws it in the sub-menu row, the top bar in the dropdown
    /// and phone-menu rows. The icon-collapsed sidebar shows only top-level icons (a group's
    /// own icon stands for it and opens its children, with their icons, in a flyout menu), so a
    /// group should have an icon of its own. No icon by default.
    pub icon: Option<ViewFn>,
    pub children: Vec<NavItem>,
    /// A short count or tag shown at the end of the row.
    pub badge: Option<String>,
    pub disabled: bool,
    /// Hide the item while this is false, for permission checks (`Signal::derive(...)`).
    pub visible: Option<Signal<bool>>,
    pub matching: NavMatch,
}

impl NavItem {
    /// A link.
    pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: Some(href.into()),
            icon: None,
            children: Vec::new(),
            badge: None,
            disabled: false,
            visible: None,
            matching: NavMatch::Exact,
        }
    }

    /// A group with no page of its own; add links with [`Self::child`].
    pub fn group(label: impl Into<String>) -> Self {
        Self {
            href: None,
            ..Self::new(label, "")
        }
    }

    /// Sets the icon (see the field). Works on children as on top-level items.
    pub fn icon<V: IntoView + 'static>(
        mut self,
        icon: impl Fn() -> V + Send + Sync + 'static,
    ) -> Self {
        self.icon = Some(ViewFn::from(icon));
        self
    }

    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn visible(mut self, visible: impl Into<Signal<bool>>) -> Self {
        self.visible = Some(visible.into());
        self
    }

    pub fn matching(mut self, matching: NavMatch) -> Self {
        self.matching = matching;
        self
    }

    pub fn child(mut self, child: NavItem) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = NavItem>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn is_group(&self) -> bool {
        !self.children.is_empty()
    }

    /// Not hidden by its `visible` signal. Reading it inside a reactive context tracks the signal.
    pub fn is_visible(&self) -> bool {
        self.visible.is_none_or(|v| v.get())
    }

    /// The children that are visible.
    pub fn visible_children(&self) -> Vec<NavItem> {
        self.children
            .iter()
            .filter(|c| c.is_visible())
            .cloned()
            .collect()
    }

    /// This item's own link is the current page.
    pub fn is_current(&self, path: &str) -> bool {
        self.href
            .as_deref()
            .is_some_and(|href| path_matches(href, path, self.matching))
    }

    /// One of the children (or theirs) is the current page. Hidden children still count, so a
    /// page you can reach by URL keeps its group open.
    pub fn has_active_descendant(&self, path: &str) -> bool {
        self.children.iter().any(|c| c.is_active(path))
    }

    /// The current page is this item or inside it.
    pub fn is_active(&self, path: &str) -> bool {
        self.is_current(path) || self.has_active_descendant(path)
    }
}

/// The whole menu.
#[derive(Clone, Default)]
pub struct AppNav {
    pub items: Vec<NavItem>,
}

impl AppNav {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, item: NavItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = NavItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// The top-level items that are visible.
    pub fn visible_items(&self) -> Vec<NavItem> {
        self.items
            .iter()
            .filter(|i| i.is_visible())
            .cloned()
            .collect()
    }

    /// The labels from the top down to the current page (`["Alpha", "GitHub"]`), for a
    /// breadcrumb or a title. Empty when no item matches. The deepest exact match wins; a
    /// group's own link only counts when none of its children match.
    pub fn trail(&self, path: &str) -> Vec<String> {
        fn walk(items: &[NavItem], path: &str, out: &mut Vec<String>) -> bool {
            for item in items {
                out.push(item.label.clone());
                if walk(&item.children, path, out) || item.is_current(path) {
                    return true;
                }
                out.pop();
            }
            false
        }
        let mut out = Vec::new();
        walk(&self.items, path, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menu() -> AppNav {
        AppNav::new()
            .item(NavItem::new("Dashboard", "/app"))
            .item(
                NavItem::group("Alpha")
                    .child(NavItem::new("GitHub", "/app/alpha/github"))
                    .child(
                        NavItem::new("Servers", "/app/alpha/servers").matching(NavMatch::Prefix),
                    ),
            )
            .item(NavItem::new("Reports", "/app/reports").matching(NavMatch::Prefix))
    }

    #[test]
    fn normalizes_query_hash_and_trailing_slash() {
        assert_eq!(normalize_path("/a/b/?x=1#y"), "/a/b");
        assert_eq!(normalize_path("/a#top"), "/a");
        assert_eq!(normalize_path("/"), "/");
        assert_eq!(normalize_path(""), "");
    }

    #[test]
    fn exact_matching_ignores_decoration_but_not_depth() {
        assert!(path_matches(
            "/app/reports",
            "/app/reports/",
            NavMatch::Exact
        ));
        assert!(path_matches(
            "/app/reports",
            "/app/reports?tab=1",
            NavMatch::Exact
        ));
        assert!(!path_matches(
            "/app/reports",
            "/app/reports/12",
            NavMatch::Exact
        ));
    }

    #[test]
    fn prefix_matching_is_segment_aware() {
        assert!(path_matches("/servers", "/servers/12", NavMatch::Prefix));
        assert!(!path_matches("/servers", "/servers-old", NavMatch::Prefix));
        assert!(!path_matches("/servers", "/", NavMatch::Prefix));
        // A root link would match everything if prefix matching applied to it.
        assert!(path_matches("/", "/", NavMatch::Prefix));
        assert!(!path_matches("/", "/anything", NavMatch::Prefix));
    }

    #[test]
    fn a_group_is_active_when_a_child_is() {
        let nav = menu();
        let alpha = &nav.items[1];
        assert!(
            !alpha.is_current("/app/alpha/github"),
            "a group has no page of its own"
        );
        assert!(alpha.has_active_descendant("/app/alpha/github"));
        assert!(
            alpha.is_active("/app/alpha/servers/7"),
            "prefix child keeps the group active"
        );
        assert!(!alpha.is_active("/app"));
        assert!(!nav.items[0].has_active_descendant("/app"));
    }

    #[test]
    fn trail_lists_labels_down_to_the_page() {
        let nav = menu();
        assert_eq!(nav.trail("/app/alpha/github"), ["Alpha", "GitHub"]);
        assert_eq!(nav.trail("/app"), ["Dashboard"]);
        assert_eq!(nav.trail("/app/reports/2024"), ["Reports"]);
        assert!(nav.trail("/nowhere").is_empty());
    }

    #[test]
    fn a_groups_own_link_counts_only_without_a_matching_child() {
        let nav = AppNav::new().item(
            NavItem::new("Admin", "/admin").child(NavItem::new("Sessions", "/admin/sessions")),
        );
        assert_eq!(nav.trail("/admin"), ["Admin"]);
        assert_eq!(nav.trail("/admin/sessions"), ["Admin", "Sessions"]);
    }

    #[test]
    fn children_keep_their_icons_when_filtered() {
        let nav = AppNav::new().item(
            NavItem::group("Admin")
                .child(NavItem::new("Sessions", "/admin/sessions").icon(|| "S"))
                .child(NavItem::new("Plain", "/admin/plain")),
        );
        let kids = nav.items[0].visible_children();
        assert!(kids[0].icon.is_some());
        assert!(kids[1].icon.is_none(), "no icon by default");
    }

    #[test]
    fn hidden_items_are_filtered_but_still_keep_their_group_open() {
        let show = RwSignal::new(false);
        let nav = AppNav::new().item(
            NavItem::group("Admin")
                .visible(Signal::derive(move || show.get()))
                .child(NavItem::new("Sessions", "/admin/sessions")),
        );
        assert!(nav.visible_items().is_empty());
        assert!(nav.items[0].has_active_descendant("/admin/sessions"));
        show.set(true);
        assert_eq!(nav.visible_items().len(), 1);
    }
}
