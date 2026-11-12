//! shadcn/ui components for Leptos 0.8, on headless, accessible primitives.
//!
//! Verdelle supports one shadcn style, **nova** (shadcn's default). Other styles in a preset code
//! fall back to it; [`preset::Preset::unsupported`] says which parts were not rendered. The
//! components are plain CSS (`css/components.css`, in Tailwind's `components` layer so utility
//! classes override it); Tailwind is optional for an app.
//!
//! # Using it
//!
//! Import the stylesheet (`css/kit.css`, tokens plus component styles; `css/tailwind.css` is
//! an optional colour mapping for Tailwind utilities), call [`provide_ui_context`] once near the
//! root of the app, and use the components. Forward the `ssr` and `hydrate` features from the
//! app. See `docs/using-verdelle.md` for how an app depends on the crate and imports the CSS.
//!
//! # What is in the crate
//!
//! - **Components**, at the crate root: one per file, typed props (`variant`, `size`, ...) that
//!   become `data-*` attributes the CSS selects on, and a `class` prop for layout classes. They
//!   follow shadcn's names (`Button`, `Dialog`, `DropdownMenu`, `Select`, `Sidebar`, `Calendar`,
//!   `Chart`, ...) plus a few of our own (`Indicator`, `DataTable`, `AppShell`, `Carousel`,
//!   `Drawer`, `Meter`, `NumberField`, ...). `NAMING.md` lists where names differ from shadcn's.
//!   [`Icon`] and [`IconName`] are the Lucide icon set.
//! - [`primitives`]: the headless layer (roles, keyboard, focus, ids, positioning) the components
//!   are built on, usable without the styling. Thirty-two modules are ported from Leptix; the
//!   others (calendar, carousel, chart, combobox, command, data table, drawer, number field,
//!   resizable) are Verdelle's own, with tested pure models where there is logic to test.
//! - **Theming**: a theme is a shadcn-format CSS file; [`themes`] holds the built-in ones (24,
//!   generated into `css/tokens.css` by `cargo run -p verdelle-ui --example themegen`). Select one
//!   with `data-theme` on `<html>`; light or dark is the independent `.dark` class.
//!   [`ThemeProvider`] owns both, plus shadcn's menu colour and accent, fonts and a no-flash
//!   script, and [`ThemeSwitcher`], [`ThemePicker`] and [`ModeToggle`] are the controls.
//! - **Presets**: [`preset::Preset`] decodes and encodes shadcn preset codes (the string
//!   `shadcn init --preset` and ui.shadcn.com/create produce) and composes their tokens.
//!   [`ThemeStyle`] applies pasted shadcn CSS, and [`custom_theme`] keeps themes people save in
//!   the browser (`localStorage` by default, replaceable through [`custom_theme::ThemeStorage`]).
//!   [`theme_css`] is the CSS parser both use. [`ThemeCustomizer`] is a ready-made Appearance
//!   panel for all of it, and [`SyncedStorage`] keeps a person's theme on a server.
//!
//! # Guides in `docs/`
//!
//! - `docs/presets-and-themes.md`: presets, custom themes, `ThemeProvider`, fonts and storage.
//! - `docs/using-verdelle.md`: depending on Verdelle, importing its CSS, features, version policy.
//!
//! # Conventions worth knowing
//!
//! Content of popper-positioned components (popover, menus, select, tooltip, hover card) is
//! styled through its `class` / `style` props, not `attr:class`, which lands on the positioning
//! wrapper. `attr:` on a component whose root is a `<Show>` is lost while it renders nothing, so
//! such parts take a `class` prop instead. Browser-only code is gated on the `hydrate` feature.

// Leptos view types nest deeply; with `ssr` the async `resolve()` of the ThemeSwitcher's view reaches
// depth 130, just over the default 128, depending on what the compiler has cached. A consumer's own
// crate may need a raised limit too if it builds large views.
#![recursion_limit = "256"]

/// Headless, accessible behaviour (roles, keyboard, focus, ids, positioning) the components are built on.
/// No styling; usable on its own.
pub mod primitives;

pub mod custom_theme;
pub mod preset;
mod preset_tokens;
pub use preset_tokens::remap_bold_accent;
pub mod theme_css;

mod accordion;
mod accordion_content;
mod accordion_item;
mod accordion_trigger;
mod alert;
mod alert_description;
mod alert_dialog;
mod alert_dialog_action;
mod alert_dialog_cancel;
mod alert_dialog_description;
mod alert_dialog_footer;
mod alert_dialog_header;
mod alert_dialog_media;
mod alert_dialog_title;
mod alert_title;
mod alert_variant;
mod align;
mod app_shell;
mod avatar;
mod avatar_badge;
mod avatar_fallback;
mod avatar_group;
mod avatar_group_count;
mod avatar_image;
mod badge;
mod badge_variant;
mod breakpoint;
mod button;
mod button_variant;
mod calendar;
mod calendar_mode;
mod calendar_state;
mod caption_layout;
mod card;
mod card_action;
mod card_content;
mod card_description;
mod card_footer;
mod card_header;
mod card_title;
mod carousel;
mod carousel_content;
mod carousel_dots;
mod carousel_gap;
mod carousel_item;
mod carousel_next;
mod carousel_previous;
mod checkbox;
mod collapsible;
mod collapsible_content;
mod collapsible_trigger;
mod combobox;
mod combobox_chip;
mod combobox_chip_list;
mod combobox_chips;
mod combobox_chips_input;
mod combobox_content;
mod combobox_empty;
mod combobox_group;
mod combobox_input;
mod combobox_item;
mod combobox_label;
mod combobox_list;
mod combobox_separator;
mod command;
mod command_dialog;
mod command_empty;
mod command_group;
mod command_input;
mod command_item;
mod command_list;
mod command_separator;
mod command_shortcut;
mod data_table;
mod data_table_column;
mod data_table_column_filter;
mod data_table_state;
mod date_picker;
mod date_range_picker;
mod dialog;
mod dialog_content;
mod dialog_description;
mod dialog_footer;
mod dialog_header;
mod dialog_host;
mod dialog_title;
mod drawer;
mod drawer_close;
mod drawer_description;
mod drawer_direction;
mod drawer_footer;
mod drawer_header;
mod drawer_title;
mod drawer_trigger;
mod drawer_wrapper;
mod dropdown_menu;
mod dropdown_menu_checkbox_item;
mod dropdown_menu_content;
mod dropdown_menu_group;
mod dropdown_menu_item;
mod dropdown_menu_label;
mod dropdown_menu_radio_group;
mod dropdown_menu_radio_item;
mod dropdown_menu_separator;
mod dropdown_menu_shortcut;
mod dropdown_menu_sub;
mod dropdown_menu_sub_content;
mod dropdown_menu_sub_trigger;
mod dropdown_menu_trigger;
mod field;
mod field_content;
mod field_description;
mod field_error;
mod field_group;
mod field_label;
mod field_legend;
mod field_legend_variant;
mod field_orientation;
mod field_separator;
mod field_set;
mod field_title;
mod hover_card;
mod hover_card_content;
mod hover_card_trigger;
mod indicator;
mod input;
mod input_otp;
mod input_otp_group;
mod input_otp_separator;
mod input_otp_slot;
mod label;
mod menu_item_variant;
mod meter;
mod meter_label;
mod meter_value;
mod mode_toggle;
mod native_dialog;
mod nav;
mod number_field;
mod number_field_decrement;
mod number_field_group;
mod number_field_increment;
mod number_field_input;
mod number_field_scrub_area;
mod password_input;
mod popover;
mod popover_close;
mod popover_content;
mod popover_description;
mod popover_header;
mod popover_title;
mod popover_trigger;
mod progress;
mod progress_label;
mod progress_value;
mod radio_group;
mod radio_group_item;
mod resizable_handle;
mod resizable_panel;
mod resizable_panel_group;
mod scroll_area;
mod scroll_area_orientation;
mod select;
mod select_content;
mod select_group;
mod select_item;
mod select_label;
mod select_separator;
mod select_trigger;
mod select_value;
mod separator;
mod sheet;
mod sheet_close;
mod sheet_description;
mod sheet_footer;
mod sheet_header;
mod sheet_title;
mod shell_content;
mod shell_layout;
mod shell_user;
mod side;
mod sidebar;
mod sidebar_collapsible;
mod sidebar_content;
mod sidebar_context;
mod sidebar_footer;
mod sidebar_group;
mod sidebar_group_action;
mod sidebar_group_content;
mod sidebar_group_label;
mod sidebar_header;
mod sidebar_input;
mod sidebar_inset;
mod sidebar_menu;
mod sidebar_menu_action;
mod sidebar_menu_badge;
mod sidebar_menu_button;
mod sidebar_menu_button_variant;
mod sidebar_menu_item;
mod sidebar_menu_skeleton;
mod sidebar_menu_sub;
mod sidebar_menu_sub_button;
mod sidebar_menu_sub_item;
mod sidebar_provider;
mod sidebar_rail;
mod sidebar_separator;
mod sidebar_shell;
mod sidebar_side;
mod sidebar_trigger;
mod sidebar_variant;
mod size;
mod slider;
mod spinner;
mod switch;
mod synced_storage;
mod table;
mod table_body;
mod table_caption;
mod table_cell;
mod table_footer;
mod table_head;
mod table_header;
mod table_row;
mod tabs;
mod tabs_content;
mod tabs_list;
mod tabs_trigger;
mod text_align;
mod textarea;
mod theme_customizer;
mod theme_customizer_model;
mod theme_picker;
mod theme_provider;
mod theme_style;
mod theme_switcher;
/// The built-in themes (generated by the `themegen` example) and the token stylesheet.
pub mod themes;
mod time_input;
mod toast;
mod toast_action;
mod toast_close;
mod toast_description;
mod toast_position;
mod toast_provider;
mod toast_title;
mod toast_variant;
mod toast_viewport;
mod toaster;
mod toggle;
mod toggle_group;
mod toggle_group_item;
mod toggle_variant;
mod toolbar;
mod toolbar_button;
mod toolbar_input;
mod toolbar_link;
mod toolbar_separator;
mod toolbar_toggle_group;
mod toolbar_toggle_item;
mod tooltip;
mod tooltip_content;
mod tooltip_provider;
mod tooltip_text;
mod tooltip_trigger;
mod top_bar_shell;
mod typography;

// Pure-CSS components (no primitive).
mod aspect_ratio;
mod breadcrumb;
mod breadcrumb_ellipsis;
mod breadcrumb_item;
mod breadcrumb_link;
mod breadcrumb_list;
mod breadcrumb_page;
mod breadcrumb_separator;
mod button_group;
mod button_group_separator;
mod button_group_text;
mod empty;
mod empty_content;
mod empty_description;
mod empty_header;
mod empty_media;
mod empty_media_variant;
mod empty_title;
mod icon;
mod icon_name;
mod icons;
mod input_group;
mod input_group_addon;
mod input_group_align;
mod input_group_button;
mod input_group_button_size;
mod input_group_input;
mod input_group_text;
mod input_group_textarea;
mod item;
mod item_actions;
mod item_content;
mod item_description;
mod item_footer;
mod item_group;
mod item_header;
mod item_media;
mod item_media_variant;
mod item_separator;
mod item_title;
mod item_variant;
mod kbd;
mod kbd_group;
mod native_select;
mod native_select_opt_group;
mod native_select_option;
mod navigation_menu;
mod navigation_menu_content;
mod navigation_menu_indicator;
mod navigation_menu_item;
mod navigation_menu_link;
mod navigation_menu_list;
mod navigation_menu_trigger;
mod pagination;
mod pagination_content;
mod pagination_ellipsis;
mod pagination_item;
mod pagination_link;
mod pagination_next;
mod pagination_previous;
mod skeleton;
// Context Menu and Menubar
mod context_menu;
mod context_menu_checkbox_item;
mod context_menu_content;
mod context_menu_group;
mod context_menu_item;
mod context_menu_label;
mod context_menu_radio_group;
mod context_menu_radio_item;
mod context_menu_separator;
mod context_menu_shortcut;
mod context_menu_sub;
mod context_menu_sub_content;
mod context_menu_sub_trigger;
mod context_menu_trigger;
mod menubar;
mod menubar_checkbox_item;
mod menubar_content;
mod menubar_group;
mod menubar_item;
mod menubar_label;
mod menubar_menu;
mod menubar_radio_group;
mod menubar_radio_item;
mod menubar_separator;
mod menubar_shortcut;
mod menubar_sub;
mod menubar_sub_content;
mod menubar_sub_trigger;
mod menubar_trigger;

mod area_chart;
mod bar_chart;
mod chart_cartesian;
mod chart_common;
mod chart_config;
mod chart_container;
mod chart_context;
mod chart_grid;
mod chart_legend;
mod chart_legend_content;
mod chart_orientation;
mod chart_overlay;
mod chart_polar;
mod chart_tooltip;
mod chart_tooltip_content;
mod chart_tooltip_indicator;
mod line_chart;
mod pie_chart;
mod radar_chart;
mod radar_grid;
mod radial_bar_chart;

pub use crate::primitives::calendar::{
    CalendarDate, CalendarNames, DateFormat, DateRange, Weekday,
};
pub use crate::primitives::collapsible::{CollapsibleState, use_collapsible};
pub use crate::primitives::combobox::use_combobox_values;
pub use crate::primitives::core::listbox::{
    FilterQuery, contains_filter, fuzzy_filter, starts_with_filter,
};
pub use crate::primitives::data_table::{ColumnFilter, DataTableQuery, SortDirection, SortKey};
pub use accordion::Accordion;
pub use accordion::AccordionType;
pub use accordion_content::AccordionContent;
pub use accordion_item::AccordionItem;
pub use accordion_trigger::AccordionTrigger;
pub use alert::Alert;
pub use alert_description::AlertDescription;
pub use alert_dialog::AlertDialog;
pub use alert_dialog_action::AlertDialogAction;
pub use alert_dialog_cancel::AlertDialogCancel;
pub use alert_dialog_description::AlertDialogDescription;
pub use alert_dialog_footer::AlertDialogFooter;
pub use alert_dialog_header::AlertDialogHeader;
pub use alert_dialog_media::AlertDialogMedia;
pub use alert_dialog_title::AlertDialogTitle;
pub use alert_title::AlertTitle;
pub use alert_variant::AlertVariant;
pub use align::Align;
pub use app_shell::AppShell;
pub use avatar::Avatar;
pub use avatar_badge::AvatarBadge;
pub use avatar_fallback::AvatarFallback;
pub use avatar_group::AvatarGroup;
pub use avatar_group_count::AvatarGroupCount;
pub use avatar_image::{AvatarImage, ImageLoadingStatus};
pub use badge::Badge;
pub use badge_variant::BadgeVariant;
pub use breakpoint::Breakpoint;
pub use button::Button;
pub use button_variant::ButtonVariant;
pub use calendar::Calendar;
pub use calendar_mode::CalendarMode;
pub use caption_layout::CaptionLayout;
pub use card::Card;
pub use card_action::CardAction;
pub use card_content::CardContent;
pub use card_description::CardDescription;
pub use card_footer::CardFooter;
pub use card_header::CardHeader;
pub use card_title::CardTitle;
pub use carousel::{Carousel, CarouselApi, use_carousel};
pub use carousel_content::CarouselContent;
pub use carousel_dots::CarouselDots;
pub use carousel_gap::CarouselGap;
pub use carousel_item::CarouselItem;
pub use carousel_next::CarouselNext;
pub use carousel_previous::CarouselPrevious;
pub use checkbox::{Checkbox, Checked};
pub use collapsible::Collapsible;
pub use collapsible_content::CollapsibleContent;
pub use collapsible_trigger::CollapsibleTrigger;
pub use combobox::Combobox;
pub use combobox_chip::ComboboxChip;
pub use combobox_chip_list::ComboboxChipList;
pub use combobox_chips::ComboboxChips;
pub use combobox_chips_input::ComboboxChipsInput;
pub use combobox_content::ComboboxContent;
pub use combobox_empty::ComboboxEmpty;
pub use combobox_group::ComboboxGroup;
pub use combobox_input::ComboboxInput;
pub use combobox_item::ComboboxItem;
pub use combobox_label::ComboboxLabel;
pub use combobox_list::ComboboxList;
pub use combobox_separator::ComboboxSeparator;
pub use command::Command;
pub use command_dialog::CommandDialog;
pub use command_empty::CommandEmpty;
pub use command_group::CommandGroup;
pub use command_input::CommandInput;
pub use command_item::CommandItem;
pub use command_list::CommandList;
pub use command_separator::CommandSeparator;
pub use command_shortcut::CommandShortcut;
pub use data_table::{DataTable, DataTableFooterInfo};
pub use data_table_column::DataTableColumn;
pub use data_table_column_filter::DataTableColumnFilter;
pub use data_table_state::DataTableState;
pub use date_picker::DatePicker;
pub use date_range_picker::DateRangePicker;
pub use dialog::Dialog;
pub use dialog_content::DialogContent;
pub use dialog_description::DialogDescription;
pub use dialog_footer::DialogFooter;
pub use dialog_header::DialogHeader;
pub use dialog_host::DialogHost;
pub use dialog_title::DialogTitle;
pub use drawer::Drawer;
pub use drawer_close::DrawerClose;
pub use drawer_description::DrawerDescription;
pub use drawer_direction::DrawerDirection;
pub use drawer_footer::DrawerFooter;
pub use drawer_header::DrawerHeader;
pub use drawer_title::DrawerTitle;
pub use drawer_trigger::DrawerTrigger;
pub use drawer_wrapper::DrawerWrapper;
pub use dropdown_menu::DropdownMenu;
pub use dropdown_menu_checkbox_item::DropdownMenuCheckboxItem;
pub use dropdown_menu_content::DropdownMenuContent;
pub use dropdown_menu_group::DropdownMenuGroup;
pub use dropdown_menu_item::DropdownMenuItem;
pub use dropdown_menu_label::DropdownMenuLabel;
pub use dropdown_menu_radio_group::DropdownMenuRadioGroup;
pub use dropdown_menu_radio_item::DropdownMenuRadioItem;
pub use dropdown_menu_separator::DropdownMenuSeparator;
pub use dropdown_menu_shortcut::DropdownMenuShortcut;
pub use dropdown_menu_sub::DropdownMenuSub;
pub use dropdown_menu_sub_content::DropdownMenuSubContent;
pub use dropdown_menu_sub_trigger::DropdownMenuSubTrigger;
pub use dropdown_menu_trigger::DropdownMenuTrigger;
pub use field::Field;
pub use field_content::FieldContent;
pub use field_description::FieldDescription;
pub use field_error::FieldError;
pub use field_group::FieldGroup;
pub use field_label::FieldLabel;
pub use field_legend::FieldLegend;
pub use field_legend_variant::FieldLegendVariant;
pub use field_orientation::FieldOrientation;
pub use field_separator::FieldSeparator;
pub use field_set::FieldSet;
pub use field_title::FieldTitle;
pub use hover_card::HoverCard;
pub use hover_card_content::HoverCardContent;
pub use hover_card_trigger::HoverCardTrigger;
pub use icon::Icon;
pub use icon_name::IconName;
pub use indicator::Indicator;
pub use input::Input;
pub use input_otp::InputOtp;
pub use input_otp::InputValidationType;
pub use input_otp_group::InputOtpGroup;
pub use input_otp_separator::InputOtpSeparator;
pub use input_otp_slot::InputOtpSlot;
pub use label::Label;
pub use menu_item_variant::MenuItemVariant;
pub use meter::{Meter, MeterState, meter_state};
pub use meter_label::MeterLabel;
pub use meter_value::MeterValue;
pub use mode_toggle::ModeToggle;
pub use nav::{AppNav, NavItem, NavMatch, normalize_path, path_matches};
pub use number_field::NumberField;
pub use number_field_decrement::NumberFieldDecrement;
pub use number_field_group::NumberFieldGroup;
pub use number_field_increment::NumberFieldIncrement;
pub use number_field_input::NumberFieldInput;
pub use number_field_scrub_area::NumberFieldScrubArea;
pub use password_input::PasswordInput;
pub use popover::Popover;
pub use popover_close::PopoverClose;
pub use popover_content::PopoverContent;
pub use popover_description::PopoverDescription;
pub use popover_header::PopoverHeader;
pub use popover_title::PopoverTitle;
pub use popover_trigger::PopoverTrigger;
pub use progress::Progress;
pub use progress_label::ProgressLabel;
pub use progress_value::ProgressValue;
pub use radio_group::RadioGroup;
pub use radio_group_item::RadioGroupItem;
pub use resizable_handle::ResizableHandle;
pub use resizable_panel::ResizablePanel;
pub use resizable_panel_group::ResizablePanelGroup;
pub use scroll_area::{ScrollArea, ScrollAreaType};
pub use scroll_area_orientation::ScrollAreaOrientation;
pub use select::Select;
pub use select_content::SelectContent;
pub use select_group::SelectGroup;
pub use select_item::SelectItem;
pub use select_label::SelectLabel;
pub use select_separator::SelectSeparator;
pub use select_trigger::SelectTrigger;
pub use select_value::SelectValue;
pub use separator::Orientation;
pub use separator::Separator;
pub use sheet::Sheet;
pub use sheet_close::SheetClose;
pub use sheet_description::SheetDescription;
pub use sheet_footer::SheetFooter;
pub use sheet_header::SheetHeader;
pub use sheet_title::SheetTitle;
pub use shell_content::ShellContent;
pub use shell_layout::ShellLayout;
pub use shell_user::ShellUser;
pub use side::Side;
pub use sidebar::Sidebar;
pub use sidebar_collapsible::SidebarCollapsible;
pub use sidebar_content::SidebarContent;
pub use sidebar_context::SidebarContext;
pub use sidebar_context::use_sidebar;
pub use sidebar_footer::SidebarFooter;
pub use sidebar_group::SidebarGroup;
pub use sidebar_group_action::SidebarGroupAction;
pub use sidebar_group_content::SidebarGroupContent;
pub use sidebar_group_label::SidebarGroupLabel;
pub use sidebar_header::SidebarHeader;
pub use sidebar_input::SidebarInput;
pub use sidebar_inset::SidebarInset;
pub use sidebar_menu::SidebarMenu;
pub use sidebar_menu_action::SidebarMenuAction;
pub use sidebar_menu_badge::SidebarMenuBadge;
pub use sidebar_menu_button::SidebarMenuButton;
pub use sidebar_menu_button_variant::SidebarMenuButtonVariant;
pub use sidebar_menu_item::SidebarMenuItem;
pub use sidebar_menu_skeleton::SidebarMenuSkeleton;
pub use sidebar_menu_sub::SidebarMenuSub;
pub use sidebar_menu_sub_button::SidebarMenuSubButton;
pub use sidebar_menu_sub_item::SidebarMenuSubItem;
pub use sidebar_provider::SidebarProvider;
pub use sidebar_rail::SidebarRail;
pub use sidebar_separator::SidebarSeparator;
pub use sidebar_shell::{SidebarShell, SidebarShellProps};
pub use sidebar_side::SidebarSide;
pub use sidebar_trigger::SidebarTrigger;
pub use sidebar_variant::SidebarVariant;
pub use size::Size;
pub use slider::Slider;
pub use spinner::Spinner;
pub use switch::Switch;
pub use synced_storage::{PendingWrites, SyncedStorage, Write};
pub use table::Table;
pub use table_body::TableBody;
pub use table_caption::TableCaption;
pub use table_cell::TableCell;
pub use table_footer::TableFooter;
pub use table_head::TableHead;
pub use table_header::TableHeader;
pub use table_row::{RowState, TableRow};
pub use tabs::Tabs;
pub use tabs_content::TabsContent;
pub use tabs_list::TabsList;
pub use tabs_trigger::TabsTrigger;
pub use text_align::TextAlign;
pub use textarea::Textarea;
pub use theme_customizer::{ThemeApplied, ThemeCustomizer};
pub use theme_picker::ThemePicker;
pub use theme_provider::{
    ThemeChoice, ThemeContext, ThemeMode, ThemeProvider, ThemeSource, theme_init_script,
    theme_init_script_seeded, use_theme,
};
pub use theme_style::ThemeStyle;
pub use theme_switcher::ThemeSwitcher;
pub use toast::Toast;
pub use toast_action::ToastAction;
pub use toast_close::ToastClose;
pub use toast_description::ToastDescription;
pub use toast_position::ToastPosition;
pub use toast_provider::ToastProvider;
pub use toast_title::ToastTitle;
pub use toast_variant::ToastVariant;
pub use toast_viewport::ToastViewport;
pub use toaster::{ToastActionData, ToastData, ToastHandle, Toaster, use_toaster};
pub use toggle::Toggle;
pub use toggle_group::ToggleGroup;
pub use toggle_group_item::ToggleGroupItem;
pub use toggle_variant::ToggleVariant;
pub use toolbar::Toolbar;
pub use toolbar_button::ToolbarButton;
pub use toolbar_input::ToolbarInput;
pub use toolbar_link::ToolbarLink;
pub use toolbar_separator::ToolbarSeparator;
pub use toolbar_toggle_group::ToolbarToggleGroup;
pub use toolbar_toggle_item::ToolbarToggleItem;
pub use top_bar_shell::{TopBarShell, TopBarShellProps};
pub use typography::{
    Prose, TypographyBlockquote, TypographyH1, TypographyH2, TypographyH3, TypographyH4,
    TypographyInlineCode, TypographyLarge, TypographyLead, TypographyList, TypographyMuted,
    TypographyP, TypographySmall,
};

pub use crate::primitives::time_field::HourCycle;
pub use aspect_ratio::AspectRatio;
pub use breadcrumb::Breadcrumb;
pub use breadcrumb_ellipsis::BreadcrumbEllipsis;
pub use breadcrumb_item::BreadcrumbItem;
pub use breadcrumb_link::BreadcrumbLink;
pub use breadcrumb_list::BreadcrumbList;
pub use breadcrumb_page::BreadcrumbPage;
pub use breadcrumb_separator::BreadcrumbSeparator;
pub use button_group::ButtonGroup;
pub use button_group_separator::ButtonGroupSeparator;
pub use button_group_text::ButtonGroupText;
pub use context_menu::ContextMenu;
pub use context_menu_checkbox_item::ContextMenuCheckboxItem;
pub use context_menu_content::ContextMenuContent;
pub use context_menu_group::ContextMenuGroup;
pub use context_menu_item::ContextMenuItem;
pub use context_menu_label::ContextMenuLabel;
pub use context_menu_radio_group::ContextMenuRadioGroup;
pub use context_menu_radio_item::ContextMenuRadioItem;
pub use context_menu_separator::ContextMenuSeparator;
pub use context_menu_shortcut::ContextMenuShortcut;
pub use context_menu_sub::ContextMenuSub;
pub use context_menu_sub_content::ContextMenuSubContent;
pub use context_menu_sub_trigger::ContextMenuSubTrigger;
pub use context_menu_trigger::ContextMenuTrigger;
pub use empty::Empty;
pub use empty_content::EmptyContent;
pub use empty_description::EmptyDescription;
pub use empty_header::EmptyHeader;
pub use empty_media::EmptyMedia;
pub use empty_media_variant::EmptyMediaVariant;
pub use empty_title::EmptyTitle;
pub use input_group::InputGroup;
pub use input_group_addon::InputGroupAddon;
pub use input_group_align::InputGroupAlign;
pub use input_group_button::InputGroupButton;
pub use input_group_button_size::InputGroupButtonSize;
pub use input_group_input::InputGroupInput;
pub use input_group_text::InputGroupText;
pub use input_group_textarea::InputGroupTextarea;
pub use item::Item;
pub use item_actions::ItemActions;
pub use item_content::ItemContent;
pub use item_description::ItemDescription;
pub use item_footer::ItemFooter;
pub use item_group::ItemGroup;
pub use item_header::ItemHeader;
pub use item_media::ItemMedia;
pub use item_media_variant::ItemMediaVariant;
pub use item_separator::ItemSeparator;
pub use item_title::ItemTitle;
pub use item_variant::ItemVariant;
pub use kbd::Kbd;
pub use kbd_group::KbdGroup;
pub use menubar::Menubar;
pub use menubar_checkbox_item::MenubarCheckboxItem;
pub use menubar_content::MenubarContent;
pub use menubar_group::MenubarGroup;
pub use menubar_item::MenubarItem;
pub use menubar_label::MenubarLabel;
pub use menubar_menu::MenubarMenu;
pub use menubar_radio_group::MenubarRadioGroup;
pub use menubar_radio_item::MenubarRadioItem;
pub use menubar_separator::MenubarSeparator;
pub use menubar_shortcut::MenubarShortcut;
pub use menubar_sub::MenubarSub;
pub use menubar_sub_content::MenubarSubContent;
pub use menubar_sub_trigger::MenubarSubTrigger;
pub use menubar_trigger::MenubarTrigger;
pub use native_select::NativeSelect;
pub use native_select_opt_group::NativeSelectOptGroup;
pub use native_select_option::NativeSelectOption;
pub use navigation_menu::NavigationMenu;
pub use navigation_menu_content::NavigationMenuContent;
pub use navigation_menu_indicator::NavigationMenuIndicator;
pub use navigation_menu_item::NavigationMenuItem;
pub use navigation_menu_link::NavigationMenuLink;
pub use navigation_menu_list::NavigationMenuList;
pub use navigation_menu_trigger::NavigationMenuTrigger;
pub use pagination::Pagination;
pub use pagination_content::PaginationContent;
pub use pagination_ellipsis::PaginationEllipsis;
pub use pagination_item::PaginationItem;
pub use pagination_link::PaginationLink;
pub use pagination_next::PaginationNext;
pub use pagination_previous::PaginationPrevious;
pub use skeleton::Skeleton;
pub use time_input::TimeInput;
pub use tooltip::Tooltip;
pub use tooltip_content::TooltipContent;
pub use tooltip_provider::TooltipProvider;
pub use tooltip_text::TooltipText;
pub use tooltip_trigger::TooltipTrigger;

/// Component styles only. Pair with [`themes::TOKENS_CSS`] when injecting CSS
/// from Rust instead of importing `css/kit.css`.
pub const COMPONENTS_CSS: &str = include_str!("../css/components.css");

/// The whole stylesheet as one string, ready to serve from the app (a route returning
/// `text/css`) or to put in a `<style>`: the same as `css/kit.css` (tokens, then the
/// component styles in the `components` layer) but with no file to find, so it works the
/// same whether the crate came from a path, git or the registry.
///
/// It starts with `@layer theme, base, components, utilities;`, which fixes the layer order
/// whichever stylesheet loads first: the app's Tailwind output (preflight in `base`,
/// utilities in `utilities`) then still overrides the component styles, as with `kit.css`.
/// Built once and cached. See `docs/new-app.md` in the repository.
pub fn kit_css() -> &'static str {
    static CSS: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CSS.get_or_init(|| {
        format!(
            "@layer theme, base, components, utilities;\n{}\n@layer components {{\n{}\n}}\n",
            themes::TOKENS_CSS,
            COMPONENTS_CSS
        )
    })
}

/// Set up what the components share. Call it once near the root of the app (inside
/// the top-level component, so each server request gets its own): it provides the
/// id counter that keeps ids such as `aria-controls` the same on the server and in the
/// browser.
pub fn provide_ui_context() {
    crate::primitives::core::id::provide_id_generator();
}
pub use crate::primitives::chart::Curve as ChartCurve;
pub use area_chart::AreaChart;
pub use bar_chart::BarChart;
pub use chart_config::{ChartConfig, ChartDatum, ChartSeries};
pub use chart_container::ChartContainer;
pub use chart_context::{ChartTooltipItem, ChartTooltipState};
pub use chart_grid::ChartGrid;
pub use chart_legend::ChartLegend;
pub use chart_legend_content::ChartLegendContent;
pub use chart_orientation::ChartOrientation;
pub use chart_tooltip::ChartTooltip;
pub use chart_tooltip_content::ChartTooltipContent;
pub use chart_tooltip_indicator::ChartTooltipIndicator;
pub use line_chart::LineChart;
pub use pie_chart::PieChart;
pub use radar_chart::RadarChart;
pub use radar_grid::RadarGrid;
pub use radial_bar_chart::RadialBarChart;

// Link buttons, alert extras, text field callbacks, page header, stat card, initials.
mod alert_action;
mod button_type;
mod initials;
mod page_header;
mod stat_card;
mod stat_tone;
mod stat_trend;
mod text_field;
pub use alert_action::AlertAction;
pub use button_type::ButtonType;
pub use initials::initials;
pub use page_header::PageHeader;
pub use stat_card::StatCard;
pub use stat_tone::StatTone;
pub use stat_trend::StatTrend;
