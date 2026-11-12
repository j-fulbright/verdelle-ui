use crate::section::{Page, Row, Section};
use leptos::prelude::*;
use verdelle_ui::{Button, ButtonVariant, Progress, ProgressLabel, ProgressValue, Size};

#[component]
pub fn ProgressPage() -> impl IntoView {
    let value = RwSignal::new(40.0_f64);
    let files = RwSignal::new(3.0_f64);

    view! {
        <Page
            title="Progress"
            summary="A progress bar: role=progressbar with aria-valuenow and aria-valuetext, a data-state of loading, complete or indeterminate, and a sliding fill when there is no value."
        >
            <Section title="Determinate" description="With a label and a value, with a label only, and with just an aria_label.">
                <div class="grid max-w-md gap-6">
                    <Progress value=66.0>
                        <ProgressLabel>"Uploading"</ProgressLabel>
                        <ProgressValue />
                    </Progress>
                    <Progress value=25.0>
                        <ProgressLabel>"Storage used"</ProgressLabel>
                    </Progress>
                    <Progress value=100.0 aria_label="Complete">
                        <ProgressLabel>"Done"</ProgressLabel>
                        <ProgressValue />
                    </Progress>
                    <Progress value=0.0 aria_label="Not started" />
                </div>
            </Section>

            <Section title="Indeterminate" description="No value: the bar does not claim a percentage (no aria-valuenow).">
                <div class="grid max-w-md gap-6">
                    <Progress aria_label="Loading">
                        <ProgressLabel>"Working"</ProgressLabel>
                        <ProgressValue />
                    </Progress>
                </div>
            </Section>

            <Section title="Controlled" description="The value comes from a signal; the buttons change it. A max of 5 counts files instead of percent.">
                <div class="grid max-w-md gap-6">
                    <Progress value=value>
                        <ProgressLabel>"Export"</ProgressLabel>
                        <ProgressValue />
                    </Progress>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| value.update(|v| *v = (*v - 10.0).max(0.0))
                        >
                            "-10"
                        </Button>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| value.update(|v| *v = (*v + 10.0).min(100.0))
                        >
                            "+10"
                        </Button>
                        <span id="progress-readout" class="text-sm text-muted-foreground">
                            "value = " {move || value.get()}
                        </span>
                    </Row>
                    <Progress value=files max=5.0>
                        <ProgressLabel>"Files"</ProgressLabel>
                        <ProgressValue />
                    </Progress>
                    <Row>
                        <Button
                            variant=ButtonVariant::Outline
                            size=Size::Small
                            on_click=move |_| files.update(|v| *v = (*v + 1.0) % 6.0)
                        >
                            "Next file"
                        </Button>
                        <span class="text-sm text-muted-foreground">{move || files.get()} " of 5"</span>
                    </Row>
                </div>
            </Section>
        </Page>
    }
}
