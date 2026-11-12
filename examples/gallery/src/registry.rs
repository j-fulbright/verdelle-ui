//! The list of gallery pages: one place that feeds the sidebar and the Overview page, so a page cannot be
//! in one and missing from the other. A new page gets an entry here, a route in `app.rs` and a line in
//! `pages/mod.rs`; the test below fails if a page is added to `pages/mod.rs` without an entry.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Group {
    Basics,
    Forms,
    Overlays,
    Disclosure,
}

impl Group {
    pub const ALL: [Group; 4] = [
        Group::Basics,
        Group::Forms,
        Group::Overlays,
        Group::Disclosure,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Group::Basics => "Basics",
            Group::Forms => "Forms and inputs",
            Group::Overlays => "Overlays and feedback",
            Group::Disclosure => "Disclosure and scrolling",
        }
    }
}

pub struct PageInfo {
    pub path: &'static str,
    pub title: &'static str,
    pub group: Group,
    /// What is on the page: the components and primitives it shows.
    pub shows: &'static str,
}

pub const PAGES: &[PageInfo] = &[
    // Basics
    PageInfo {
        path: "/buttons",
        title: "Buttons",
        group: Group::Basics,
        shows: "Button: variants, sizes, states, icon-only, link buttons (href, target, rel, disabled), type, title, loading in a form, conditional props, Leptos Fn-children note",
    },
    PageInfo {
        path: "/badges",
        title: "Badges and indicators",
        group: Group::Basics,
        shows: "Badge (also clickable, removable chips), Indicator",
    },
    PageInfo {
        path: "/layout",
        title: "Cards, tabs, alerts",
        group: Group::Basics,
        shows: "Card (header action stacks when narrow), Tabs, Alert (icon, success and warning tones, dismiss, AlertAction), Separator, Spinner",
    },
    PageInfo {
        path: "/page-headers",
        title: "Page header and stat card",
        group: Group::Basics,
        shows: "PageHeader (title, description, actions, back link, breadcrumb, signals), StatCard (delta trend and tone, icon, footer, small, selectable filter tiles)",
    },
    PageInfo {
        path: "/app-shells",
        title: "App shells",
        group: Group::Basics,
        shows: "AppShell (AppNav data drawn by SidebarShell or TopBarShell, switchable; icon rail with header_collapsed_slot, the hide-collapsed class, group flyout menus and one-at-a-time tooltips; ShellContent Contained or Full) and five hand-built navigation layouts for one app menu: persistent sidebar, phone dropdown, top bar, product menu with tabs, icon rail",
    },
    PageInfo {
        path: "/tables",
        title: "Tables",
        group: Group::Basics,
        shows: "Table: striped, selected, footer, sticky header",
    },
    PageInfo {
        path: "/data-tables",
        title: "Data Table",
        group: Group::Basics,
        shows: "DataTable: sort (multi), search, column filters, paging, selection, column menu, footer totals, empty, loading, row actions and click, server-side mode, sticky header, 1000 rows",
    },
    PageInfo {
        path: "/data-tables-layout",
        title: "Data Table: layout",
        group: Group::Basics,
        shows: "DataTable layout and rows: wrap, min_width and max_width, hide_below, sticky and sticky_end columns, stack_below cards (select all, sort bar), row_state and row_class, expandable detail rows (controlled through DataTableState::expanded), show_status, footer_text, footer_end, toolbar_end, bordered=false inside a Card",
    },
    PageInfo {
        path: "/avatars",
        title: "Avatar",
        group: Group::Basics,
        shows: "Avatar: image, fallback, badge, group",
    },
    PageInfo {
        path: "/progress",
        title: "Progress",
        group: Group::Basics,
        shows: "Progress: determinate, indeterminate, label and value",
    },
    PageInfo {
        path: "/skeletons-misc",
        title: "Skeleton, Kbd, Aspect Ratio",
        group: Group::Basics,
        shows: "Skeleton, Kbd, KbdGroup, AspectRatio",
    },
    PageInfo {
        path: "/empty-items",
        title: "Empty and Item",
        group: Group::Basics,
        shows: "Empty, Item (media, content, actions, groups, wrapping title)",
    },
    PageInfo {
        path: "/navigation",
        title: "Breadcrumb and Pagination",
        group: Group::Basics,
        shows: "Breadcrumb, Pagination",
    },
    // Forms and inputs
    PageInfo {
        path: "/forms",
        title: "Forms",
        group: Group::Forms,
        shows: "Input, Textarea, Label, Checkbox (including the mixed state)",
    },
    PageInfo {
        path: "/selects",
        title: "Selects",
        group: Group::Forms,
        shows: "Select: sizes, groups, disabled, controlled, long lists, trigger label with and without items, empty-value All item, placeholder, clearable",
    },
    PageInfo {
        path: "/comboboxes",
        title: "Comboboxes",
        group: Group::Forms,
        shows: "Combobox: single and multiple (chips), groups, clearable, controlled, custom filters, async results, a server picker, forms",
    },
    PageInfo {
        path: "/calendars",
        title: "Calendar and Date Picker",
        group: Group::Forms,
        shows: "Calendar: single, multiple and range, caption menus, week start and numbers, fixed weeks, min/max and disabled days, several months, controlled value and month, names, form fields. DatePicker and DateRangePicker: formats, sizes, clearable, typed entry, required, presets",
    },
    PageInfo {
        path: "/switches",
        title: "Switch and radio group",
        group: Group::Forms,
        shows: "Switch, RadioGroup (also as_child and radios in table cells), Checkbox (signal, read-only value with on_checked_change, uncontrolled)",
    },
    PageInfo {
        path: "/toggles",
        title: "Toggle and toggle group",
        group: Group::Forms,
        shows: "Toggle, ToggleGroup: single and multiple, deselectable",
    },
    PageInfo {
        path: "/sliders",
        title: "Slider",
        group: Group::Forms,
        shows: "Slider: range, step, vertical, multiple thumbs",
    },
    PageInfo {
        path: "/otp",
        title: "Input OTP",
        group: Group::Forms,
        shows: "InputOtp: length, patterns, paste",
    },
    PageInfo {
        path: "/time-inputs",
        title: "Time input",
        group: Group::Forms,
        shows: "TimeInput: 24-hour and 12-hour segments, typing and stepping, controlled and on_value_change, switching hour_cycle, min and max (and across midnight), minute_step, sizes, invalid, disabled, read-only, in a form",
    },
    PageInfo {
        path: "/number-fields",
        title: "Number field",
        group: Group::Forms,
        shows: "NumberField: buttons with hold-to-repeat, min and max, step sizes, snap on step, decimals, prefix and suffix, empty, controlled, sizes, invalid, disabled, read-only, scrub area, in a form",
    },
    PageInfo {
        path: "/password-inputs",
        title: "Password input",
        group: Group::Forms,
        shows: "PasswordInput: show/hide toggle, shown at first, invalid, disabled, read-only, in a form",
    },
    PageInfo {
        path: "/fields",
        title: "Field",
        group: Group::Forms,
        shows: "Field, FieldGroup, FieldSet, FieldLabel, FieldError, validation",
    },
    PageInfo {
        path: "/icons",
        title: "Icons",
        group: Group::Basics,
        shows: "Icon and IconName: 190+ Lucide icons, searchable, sizes, stroke width, colour, labels, signal name, inside Button, Badge and Input Group",
    },
    PageInfo {
        path: "/themes",
        title: "Themes",
        group: Group::Forms,
        shows: "ThemeCustomizer (the Appearance panel: theme and mode, preset code box with summary and fallbacks, fine tuning, pasted CSS with live preview, saved themes with rename, delete, export and import, preview card; every show_* prop alone and in a playground, compact, inside a Card, in Tabs, at 390px, max_saved, on_apply, on_save, load_fonts; compact next to the full panel), SyncedStorage (write-through log, load from a pretend server, debounce_ms, flush, snapshot, seeding the first paint with theme_init_script_seeded), ThemeProvider (theme and mode, system dark, no flash, storage adapter), shadcn preset codes (reference codes, every field with a live sample, what Verdelle renders and what falls back), ThemeStyle with pasted shadcn CSS, ThemeSwitcher (with and without the provider, swatches of the preview and saved themes, own list, sizes, Field, disabled), ModeToggle, ThemePicker cards (with the provider, so registered and saved themes appear; controlled; own list); all 24 shadcn themes",
    },
    PageInfo {
        path: "/groups",
        title: "Button, Input, Select groups",
        group: Group::Forms,
        shows: "ButtonGroup, InputGroup, NativeSelect",
    },
    // Overlays and feedback
    PageInfo {
        path: "/dialogs",
        title: "Dialogs",
        group: Group::Overlays,
        shows: "Dialog: native dialog, dismissal options, on_open_change and on_close, DialogHost (mount while needed, still animates out)",
    },
    PageInfo {
        path: "/sheets",
        title: "Sheet",
        group: Group::Overlays,
        shows: "Sheet: four sides, form, long content, dismissal options, sizing, exit animation, on_open_change and on_close, DialogHost",
    },
    PageInfo {
        path: "/drawers",
        title: "Drawer",
        group: Group::Overlays,
        shows: "Drawer: four directions, swipe to dismiss, snap points (default, controlled), handle, scrolling content, form, not dismissible, non-modal, controlled open, scaled background, responsive Dialog/Drawer, on_open_change and on_close, DialogHost",
    },
    PageInfo {
        path: "/commands",
        title: "Command",
        group: Group::Overlays,
        shows: "Command and CommandDialog: groups, shortcuts, Ctrl/Cmd K palette, controlled search, fuzzy filter, in a popover, CommandDialog callbacks",
    },
    PageInfo {
        path: "/animations",
        title: "Animations",
        group: Group::Overlays,
        shows: "Exit animations of every overlay (popover, menus, select, combobox, tooltip, hover card, dialogs, sheet, toast) and the Disable animation switch",
    },
    PageInfo {
        path: "/alert-dialogs",
        title: "Alert dialog",
        group: Group::Overlays,
        shows: "AlertDialog: confirm and destructive actions, async action that stays open with loading and an error, disabled action and cancel, DialogHost",
    },
    PageInfo {
        path: "/popovers",
        title: "Popovers",
        group: Group::Overlays,
        shows: "Popover: placement, collisions, forms inside",
    },
    PageInfo {
        path: "/menus",
        title: "Dropdown menus",
        group: Group::Overlays,
        shows: "DropdownMenu: items, checkbox and radio items, submenus",
    },
    PageInfo {
        path: "/context-menus",
        title: "Context menus",
        group: Group::Overlays,
        shows: "ContextMenu: right-click and long-press, items, checkbox and radio items, submenus, collisions",
    },
    PageInfo {
        path: "/menu-styles",
        title: "Menu styles",
        group: Group::Overlays,
        shows: "Menu color (default, inverted, default-translucent, inverted-translucent) and menu accent (subtle, bold) on dropdown, context and menubar menus, select and combobox popups, with a control that sets the two <html> attributes",
    },
    PageInfo {
        path: "/menubars",
        title: "Menubar",
        group: Group::Overlays,
        shows: "Menubar: menus, submenus, check and radio items, keyboard and hover switching, controlled value",
    },
    PageInfo {
        path: "/meters",
        title: "Meter",
        group: Group::Basics,
        shows: "Meter: label and reading, low/high/optimum colouring (low good, high good, middle good), custom range and text, aria_label",
    },
    PageInfo {
        path: "/resizables",
        title: "Resizable",
        group: Group::Basics,
        shows: "Resizable: horizontal, vertical, nested groups, grip handle, min/max size, collapsible panels (collapsed signal, strip size), keyboard step, controlled sizes with on_layout_end saving, on_layout_change, disabled handle, reactive orientation",
    },
    PageInfo {
        path: "/charts",
        title: "Charts: bar, line, area",
        group: Group::Basics,
        shows: "ChartContainer, ChartConfig, BarChart (grouped, stacked, 100%, horizontal, negative values, labels), LineChart (all six curves, dots, domain), AreaChart (stacked, 100%), ChartTooltip (dot, line, dashed, formatters, icons), ChartLegend (toggle, top, static, icons), axes and grid options, reactive data",
    },
    PageInfo {
        path: "/charts-polar",
        title: "Charts: pie, radial, radar",
        group: Group::Basics,
        shows: "PieChart (labels, pad angle, half circle), donut with centre label, RadialBarChart (rings, partial sweep, stacked half ring), RadarChart (polygon and circle grid, dots, lines only), custom colours",
    },
    PageInfo {
        path: "/typography",
        title: "Typography",
        group: Group::Basics,
        shows: "TypographyH1 to H4, P, Lead, Blockquote, List, InlineCode, Large, Small, Muted, and Prose for bare tags",
    },
    PageInfo {
        path: "/toolbars",
        title: "Toolbar",
        group: Group::Basics,
        shows: "Toolbar: one tab stop with arrow-key focus, toggle groups, buttons, link, text field, separators, disabled items, vertical, no looping",
    },
    PageInfo {
        path: "/tooltips",
        title: "Tooltips",
        group: Group::Overlays,
        shows: "Tooltip: delays, placement, keyboard, hoverable content, TooltipText shorthand",
    },
    PageInfo {
        path: "/navigation-menus",
        title: "Navigation menu",
        group: Group::Basics,
        shows: "NavigationMenu: viewport, indicator, delays, keyboard, vertical",
    },
    PageInfo {
        path: "/sidebars",
        title: "Sidebar",
        group: Group::Basics,
        shows: "Sidebar: variants, sides, collapse modes, groups, menus, badges, actions, skeletons, controlled state, mobile sheet",
    },
    PageInfo {
        path: "/hover-cards",
        title: "Hover cards",
        group: Group::Overlays,
        shows: "HoverCard: delays, placement, keyboard, controlled",
    },
    PageInfo {
        path: "/toasts",
        title: "Toasts",
        group: Group::Overlays,
        shows: "Toast, Toaster, use_toaster: variants, positions, swipe",
    },
    // Disclosure and scrolling
    PageInfo {
        path: "/accordions",
        title: "Accordion",
        group: Group::Disclosure,
        shows: "Accordion: single, multiple, animated",
    },
    PageInfo {
        path: "/collapsibles",
        title: "Collapsible",
        group: Group::Disclosure,
        shows: "Collapsible, shared state across table rows (use_collapsible, as_child), rotating icon hook",
    },
    PageInfo {
        path: "/scroll-areas",
        title: "Scroll area",
        group: Group::Disclosure,
        shows: "ScrollArea: vertical, horizontal, both",
    },
    PageInfo {
        path: "/carousels",
        title: "Carousel",
        group: Group::Disclosure,
        shows: "Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext, CarouselDots: default, slides per view and basis classes, gap, vertical, looping, step, autoplay, dots, controlled index with on_select and on_api, default_index, draggable off, cards",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn paths_are_unique_and_rooted() {
        let mut seen = HashSet::new();
        for page in PAGES {
            assert!(
                page.path.starts_with('/') && page.path.len() > 1,
                "{}",
                page.path
            );
            assert!(seen.insert(page.path), "duplicate path {}", page.path);
        }
    }

    #[test]
    fn every_page_module_is_registered() {
        // pages/mod.rs has one `pub use ...Page;` per page; the Overview is the home page, not listed.
        let exported = include_str!("pages/mod.rs")
            .lines()
            .filter(|l| l.starts_with("pub use ") && l.contains("Page;"))
            .count();
        assert_eq!(
            PAGES.len() + 1,
            exported,
            "pages/mod.rs exports {exported} pages but registry.rs lists {} (plus the Overview)",
            PAGES.len()
        );
    }

    #[test]
    fn every_registered_path_has_a_route() {
        let app = include_str!("app.rs");
        for page in PAGES {
            let segment = page.path.trim_start_matches('/');
            assert!(
                app.contains(&format!("StaticSegment(\"{segment}\")")),
                "{} has no route in app.rs",
                page.path
            );
        }
    }
}
