use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Button, ButtonVariant, Checkbox, Label, RadioGroup, RadioGroupItem, Size, Switch, Table,
    TableBody, TableCell, TableHead, TableHeader, TableRow,
};

#[component]
pub fn SwitchesPage() -> impl IntoView {
    let airplane = RwSignal::new(false);
    let notify = RwSignal::new(true);
    let plan = RwSignal::new(String::from("pro"));
    let density = RwSignal::new(String::from("comfortable"));
    let terms = RwSignal::new(false);
    let marketing = RwSignal::new(true);
    let submitted = RwSignal::new(String::from("(nothing yet)"));

    view! {
        <Page
            title="Switch and radio group"
            summary="Switch and RadioGroup, built on the headless primitives. Space toggles a switch; arrow keys move and choose in a radio group."
        >
            <Section title="Switch" description="Default and small sizes, off and on, disabled. Click the label or press Space.">
                <div class="grid max-w-md gap-3">
                    <Row label="Default, off">
                        <Switch id="sw-a" aria_label="Default off" />
                        <Label html_for="sw-a">"Airplane mode"</Label>
                    </Row>
                    <Row label="Default, on">
                        <Switch id="sw-b" default_checked=true />
                        <Label html_for="sw-b">"Wi-Fi"</Label>
                    </Row>
                    <Row label="Small (24x14)">
                        <Switch id="sw-c" size=Size::Small />
                        <Switch id="sw-d" size=Size::Small default_checked=true />
                        <Label html_for="sw-d">"Compact"</Label>
                    </Row>
                    <Row label="Disabled">
                        <Switch id="sw-e" disabled=true aria_label="Disabled off" />
                        <Switch id="sw-f" disabled=true default_checked=true aria_label="Disabled on" />
                        <Label html_for="sw-f">"Locked by your admin"</Label>
                    </Row>
                </div>
            </Section>

            <Section title="Controlled switches" description="Signals own the state; the readout and the buttons share it.">
                <div class="grid max-w-md gap-3">
                    <Row>
                        <Switch
                            id="sw-ctl-1"
                            checked=airplane
                            on_checked_change=move |v| airplane.set(v)
                        />
                        <Label html_for="sw-ctl-1">"Airplane mode"</Label>
                    </Row>
                    <Row>
                        <Switch
                            id="sw-ctl-2"
                            size=Size::Small
                            checked=notify
                            on_checked_change=move |v| notify.set(v)
                        />
                        <Label html_for="sw-ctl-2">"Notifications"</Label>
                    </Row>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| airplane.update(|v| *v = !*v)
                        >
                            "Toggle airplane from outside"
                        </Button>
                    </Row>
                    <span id="sw-readout" class="text-sm text-muted-foreground">
                        "airplane: " {move || airplane.get().to_string()} ", notifications: "
                        {move || notify.get().to_string()}
                    </span>
                </div>
            </Section>

            <Section title="Radio group: vertical" description="orientation=vertical: Tab enters at the checked item; only Up/Down move and choose, wrapping at the ends and skipping the disabled item.">
                <div class="grid max-w-md gap-4">
                    <RadioGroup
                        aria_label="Plan"
                        orientation="vertical"
                        value=plan
                        on_value_change=move |v| plan.set(v)
                    >
                        <Row>
                            <RadioGroupItem id="plan-free" value="free" />
                            <Label html_for="plan-free">"Free"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="plan-pro" value="pro" />
                            <Label html_for="plan-pro">"Pro"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="plan-team" value="team" />
                            <Label html_for="plan-team">"Team"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="plan-ent" value="enterprise" disabled=true />
                            <Label html_for="plan-ent">"Enterprise (disabled item)"</Label>
                        </Row>
                    </RadioGroup>
                    <span id="radio-readout" class="text-sm text-muted-foreground">
                        "plan: " {move || plan.get()}
                    </span>
                </div>
            </Section>

            <Section title="Radio group: horizontal, uncontrolled, no default" description="Nothing is chosen at first; Tab lands on the first item. Left/Right (not Up/Down) move.">
                <div class="grid max-w-md gap-4">
                    <RadioGroup aria_label="Density" orientation="horizontal">
                        <Row>
                            <RadioGroupItem id="den-c" value="compact" />
                            <Label html_for="den-c">"Compact"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="den-m" value="comfortable" />
                            <Label html_for="den-m">"Comfortable"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="den-s" value="spacious" />
                            <Label html_for="den-s">"Spacious"</Label>
                        </Row>
                    </RadioGroup>
                    <RadioGroup
                        aria_label="Density (controlled)"
                        orientation="horizontal"
                        default_value="comfortable"
                        on_value_change=move |v| density.set(v)
                    >
                        <Row>
                            <RadioGroupItem id="den2-c" value="compact" />
                            <Label html_for="den2-c">"Compact"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="den2-m" value="comfortable" />
                            <Label html_for="den2-m">"Comfortable"</Label>
                        </Row>
                    </RadioGroup>
                    <span id="density-readout" class="text-sm text-muted-foreground">
                        "last change: " {move || density.get()}
                    </span>
                </div>
            </Section>

            <Section title="Disabled group, and the default orientation" description="No orientation set: stacked, and all four arrow keys move (try the form group below).">
                <RadioGroup aria_label="Disabled group" disabled=true default_value="b">
                    <Row>
                        <RadioGroupItem id="dis-a" value="a" />
                        <Label html_for="dis-a">"Option A"</Label>
                    </Row>
                    <Row>
                        <RadioGroupItem id="dis-b" value="b" />
                        <Label html_for="dis-b">"Option B (checked)"</Label>
                    </Row>
                </RadioGroup>
            </Section>

            <Section title="In a form" description="Checkbox, switch and radio group together; the hidden inputs make them submit with the form.">
                <form
                    class="grid max-w-md gap-4"
                    on:submit=move |ev| {
                        ev.prevent_default();
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(form) = ev
                            .target()
                            .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlFormElement>().ok())
                            && let Ok(data) = leptos::web_sys::FormData::new_with_form(&form)
                        {
                            let text = ["terms", "marketing", "delivery"]
                                .iter()
                                .map(|k| {
                                    format!(
                                        "{k}={}",
                                        data.get(k).as_string().unwrap_or_else(|| "(absent)".into())
                                    )
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            submitted.set(text);
                        }
                    }
                >
                    <Row>
                        <Checkbox id="form-terms" name="terms" checked=terms />
                        <Label html_for="form-terms">"Accept the terms"</Label>
                    </Row>
                    <Row>
                        <Switch id="form-mkt" name="marketing" default_checked=marketing.get_untracked() />
                        <Label html_for="form-mkt">"Marketing emails"</Label>
                    </Row>
                    <RadioGroup aria_label="Delivery" name="delivery" default_value="email">
                        <Row>
                            <RadioGroupItem id="del-email" value="email" />
                            <Label html_for="del-email">"Email"</Label>
                        </Row>
                        <Row>
                            <RadioGroupItem id="del-sms" value="sms" />
                            <Label html_for="del-sms">"SMS"</Label>
                        </Row>
                    </RadioGroup>
                    <div>
                        <button type="submit" id="form-submit" class="vdl-btn" data-variant="default" data-size="md">"Submit"</button>
                    </div>
                    <span id="form-readout" class="text-sm text-muted-foreground">
                        "submitted: " {move || submitted.get()}
                    </span>
                </form>
            </Section>
            <ControlledAndTableExamples />
        </Page>
    }
}

/// Checkbox controlled without a signal per row, and radios in table cells.
#[component]
fn ControlledAndTableExamples() -> impl IntoView {
    // One shared piece of state for every row.
    let selected = RwSignal::new(vec![2u32]);
    let ids = [1u32, 2, 3, 4];
    let all = Signal::derive(move || selected.with(|s| s.len() == ids.len()));
    let some = Signal::derive(move || selected.with(|s| !s.is_empty() && s.len() < ids.len()));
    let changes = RwSignal::new(0u32);
    let uncontrolled = RwSignal::new(String::from("(untouched)"));

    let picked = RwSignal::new(String::from("b"));
    let rows = [
        ("a", "Alpha plan", "$5"),
        ("b", "Beta plan", "$9"),
        ("c", "Gamma plan", "$20"),
    ];

    view! {
        <Section
            title="Checkbox: checked as a value, on_checked_change"
            description="checked takes an RwSignal (two-way, as before) or a read-only value (Signal, ReadSignal, Memo, bool) with on_checked_change, so rows share one piece of state instead of a signal each. Here every row reads one Vec and the select-all box is indeterminate while only some are chosen."
        >
            <div class="grid max-w-md gap-2">
                <Row>
                    <Checkbox
                        attr:id="cb-all"
                        aria_label="Select all"
                        checked=all
                        indeterminate=some
                        on_checked_change=move |on: bool| {
                            selected.set(if on { ids.to_vec() } else { vec![] });
                        }
                    />
                    <span class="text-sm">"Select all"</span>
                </Row>
                {ids
                    .into_iter()
                    .map(|id| {
                        view! {
                            <Row>
                                <Checkbox
                                    attr:id=format!("cb-row-{id}")
                                    aria_label=format!("Row {id}")
                                    checked=Signal::derive(move || selected.with(|s| s.contains(&id)))
                                    on_checked_change=move |on: bool| {
                                        selected
                                            .update(|s| {
                                                s.retain(|x| *x != id);
                                                if on {
                                                    s.push(id);
                                                }
                                            });
                                    }
                                />
                                <span class="text-sm">{format!("Row {id}")}</span>
                            </Row>
                        }
                    })
                    .collect_view()}
                <span id="cb-readout" class="text-sm text-muted-foreground">
                    "selected: " {move || selected.with(|s| format!("{s:?}"))}
                </span>
            </div>
            <div class="grid max-w-md gap-2">
                <Row label="Uncontrolled">
                    <Checkbox
                        attr:id="cb-uncontrolled"
                        aria_label="Uncontrolled, starts on"
                        default_checked=true
                        on_checked_change=move |on: bool| uncontrolled.set(format!("now {on}"))
                    />
                    <span id="cb-uncontrolled-readout" class="text-sm text-muted-foreground">
                        {move || uncontrolled.get()}
                    </span>
                </Row>
                <Row label="Fixed value">
                    <Checkbox
                        attr:id="cb-fixed"
                        aria_label="Fixed on"
                        checked=true
                        on_checked_change=move |_: bool| changes.update(|n| *n += 1)
                    />
                    <span id="cb-fixed-readout" class="text-sm text-muted-foreground">
                        "controlled to true: clicks are reported (" {move || changes.get()}
                        ") but the box snaps back"
                    </span>
                </Row>
            </div>
        </Section>

        <Section
            title="Radio group across table cells"
            description="Any RadioGroupItem under the group takes part, wherever it sits: the arrow keys and the tab stop follow document order, not the group element's children. With as_child the group's role and attributes go onto the element you give it (here a div around the table) instead of an extra div."
        >
            <RadioGroup
                as_child=true
                value=picked
                on_value_change=move |v: String| picked.set(v)
                aria_label="Plan"
            >
                <div id="radio-table" class="rounded-lg border">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead>""</TableHead>
                                <TableHead>"Plan"</TableHead>
                                <TableHead>"Price"</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {rows
                                .into_iter()
                                .map(|(value, name, price)| {
                                    view! {
                                        <TableRow>
                                            <TableCell>
                                                <RadioGroupItem
                                                    attr:id=format!("rt-{value}")
                                                    value=value
                                                    aria_label=name
                                                />
                                            </TableCell>
                                            <TableCell>{name}</TableCell>
                                            <TableCell>{price}</TableCell>
                                        </TableRow>
                                    }
                                })
                                .collect_view()}
                        </TableBody>
                    </Table>
                </div>
            </RadioGroup>
            <p id="radio-table-readout" class="text-sm text-muted-foreground">
                "chosen: " {move || picked.get()}
            </p>
        </Section>
    }
}
