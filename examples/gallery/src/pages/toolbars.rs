use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    ButtonVariant, Orientation, Toolbar, ToolbarButton, ToolbarInput, ToolbarLink,
    ToolbarSeparator, ToolbarToggleGroup, ToolbarToggleItem,
};

#[component]
pub fn ToolbarsPage() -> impl IntoView {
    let formats = RwSignal::new(vec!["bold".to_string()]);
    let align = RwSignal::new(vec!["left".to_string()]);
    let clicks = RwSignal::new(0u32);
    let search = RwSignal::new(String::new());

    view! {
        <Page
            title="Toolbar"
            summary="A row of controls with one tab stop. Tab enters it once and leaves it again; the arrow keys move between the items, Home and End jump to the ends. Shown on its own because shadcn has no Toolbar; this is Base UI's."
        >
            <Section
                title="Buttons, toggles and a link"
                description="Press Tab to enter, then the arrow keys. Bold, italic and underline are a multiple toggle group; the alignment group allows one. Focus wraps around."
            >
                <Toolbar aria_label="Text formatting">
                    <ToolbarToggleGroup
                        multiple=true
                        value=formats
                        on_value_change=move |v| formats.set(v)
                        aria_label="Text style"
                    >
                        <ToolbarToggleItem value="bold" aria_label="Bold">
                            <b>"B"</b>
                        </ToolbarToggleItem>
                        <ToolbarToggleItem value="italic" aria_label="Italic">
                            <i>"I"</i>
                        </ToolbarToggleItem>
                        <ToolbarToggleItem value="underline" aria_label="Underline">
                            <u>"U"</u>
                        </ToolbarToggleItem>
                    </ToolbarToggleGroup>
                    <ToolbarSeparator />
                    <ToolbarToggleGroup
                        value=align
                        on_value_change=move |v| align.set(v)
                        aria_label="Alignment"
                    >
                        <ToolbarToggleItem value="left">"Left"</ToolbarToggleItem>
                        <ToolbarToggleItem value="center">"Center"</ToolbarToggleItem>
                        <ToolbarToggleItem value="right">"Right"</ToolbarToggleItem>
                    </ToolbarToggleGroup>
                    <ToolbarSeparator />
                    <ToolbarButton on_click=move |_| clicks.update(|n| *n += 1)>"Share"</ToolbarButton>
                    <ToolbarLink href="#toolbar-link">"Help"</ToolbarLink>
                </Toolbar>
                <Row label="State">
                    <span class="text-sm text-muted-foreground" data-testid="toolbar-state">
                        {move || {
                            format!(
                                "style: {} / align: {} / shared: {}",
                                formats.get().join(","),
                                align.get().join(","),
                                clicks.get(),
                            )
                        }}
                    </span>
                </Row>
            </Section>

            <Section
                title="Disabled items"
                description="A disabled button is skipped by the arrow keys and cannot hold the tab stop; Cut is the second item, so Tab lands on Copy."
            >
                <Toolbar aria_label="Clipboard">
                    <ToolbarButton disabled=true>"Cut"</ToolbarButton>
                    <ToolbarButton>"Copy"</ToolbarButton>
                    <ToolbarButton disabled=true>"Paste"</ToolbarButton>
                    <ToolbarButton>"Select all"</ToolbarButton>
                </Toolbar>
            </Section>

            <Section
                title="Button variants"
                description="Toolbar buttons take the Button variants; ghost is the default."
            >
                <Toolbar aria_label="Variants">
                    <ToolbarButton variant=ButtonVariant::Ghost>"Ghost"</ToolbarButton>
                    <ToolbarButton variant=ButtonVariant::Outline>"Outline"</ToolbarButton>
                    <ToolbarButton variant=ButtonVariant::Secondary>"Secondary"</ToolbarButton>
                    <ToolbarButton variant=ButtonVariant::Default>"Default"</ToolbarButton>
                </Toolbar>
            </Section>

            <Section
                title="Text field"
                description="Left and Right move the caret inside the field and only hand focus on at the edge of the text. Home and End stay in the field."
            >
                <Toolbar aria_label="Search">
                    <ToolbarButton>"Filters"</ToolbarButton>
                    <ToolbarInput value=search placeholder="Search" aria_label="Search text" />
                    <ToolbarButton variant=ButtonVariant::Default>"Go"</ToolbarButton>
                </Toolbar>
            </Section>

            <Section
                title="Vertical"
                description="orientation=Vertical: Up and Down move, the separator runs across."
            >
                <Toolbar orientation=Orientation::Vertical aria_label="Tools">
                    <ToolbarButton>"Select"</ToolbarButton>
                    <ToolbarButton>"Draw"</ToolbarButton>
                    <ToolbarSeparator />
                    <ToolbarButton>"Erase"</ToolbarButton>
                </Toolbar>
            </Section>

            <Section
                title="No looping"
                description="loop_focus=false: the arrow keys stop at the first and last item."
            >
                <Toolbar loop_focus=false aria_label="Pager">
                    <ToolbarButton>"First"</ToolbarButton>
                    <ToolbarButton>"Middle"</ToolbarButton>
                    <ToolbarButton>"Last"</ToolbarButton>
                </Toolbar>
            </Section>
        </Page>
    }
}
