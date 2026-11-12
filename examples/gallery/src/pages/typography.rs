use crate::section::{Page, Section};
use leptos::prelude::*;
use verdelle_ui::{
    Prose, TypographyBlockquote, TypographyH1, TypographyH2, TypographyH3, TypographyH4,
    TypographyInlineCode, TypographyLarge, TypographyLead, TypographyList, TypographyMuted,
    TypographyP, TypographySmall,
};

#[component]
pub fn TypographyPage() -> impl IntoView {
    view! {
        <Page
            title="Typography"
            summary="shadcn's Typography page as components: headings, paragraph, blockquote, list, inline code, lead, large, small and muted. Prose styles the bare tags of HTML you do not write yourself."
        >
            <Section
                title="Headings"
                description="TypographyH1 to TypographyH4. The H1 is centred and extra-bold, the H2 has a bottom border."
            >
                <div class="space-y-2">
                    <TypographyH1>"The Joke Tax Chronicles"</TypographyH1>
                    <TypographyH2>"The People of the Kingdom"</TypographyH2>
                    <TypographyH3>"The Joke Tax"</TypographyH3>
                    <TypographyH4>"People stopped telling jokes"</TypographyH4>
                </div>
            </Section>

            <Section
                title="Paragraph and lead"
                description="TypographyP has a 28px line height and space above every paragraph but the first; TypographyLead is larger and muted."
            >
                <div>
                    <TypographyLead>
                        "A modal dialog that interrupts the user with important content and expects a response."
                    </TypographyLead>
                    <TypographyP>
                        "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax."
                    </TypographyP>
                    <TypographyP>
                        "Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place."
                    </TypographyP>
                </div>
            </Section>

            <Section
                title="Blockquote and list"
                description="The blockquote is italic with a rule on its start side; the list is bulleted with 8px between items."
            >
                <div>
                    <TypographyBlockquote>
                        "\"After all,\" he said, \"everyone enjoys a good joke, so it's only fair that they should pay for the privilege.\""
                    </TypographyBlockquote>
                    <TypographyList>
                        <li>"1st level of puns: 5 gold coins"</li>
                        <li>"2nd level of jokes: 10 gold coins"</li>
                        <li>"3rd level of one-liners: 20 gold coins"</li>
                    </TypographyList>
                </div>
            </Section>

            <Section
                title="Inline code, large, small and muted"
                description="TypographyInlineCode sits inside a sentence; Large, Small and Muted are for callouts, fine print and secondary text."
            >
                <div class="space-y-3">
                    <TypographyP>
                        "Install it with " <TypographyInlineCode>"cargo add verdelle-ui"</TypographyInlineCode>
                        " and import the styles."
                    </TypographyP>
                    <TypographyLarge>"Are you absolutely sure?"</TypographyLarge>
                    <div>
                        <TypographySmall>"Email address"</TypographySmall>
                    </div>
                    <TypographyMuted>"Enter your email address."</TypographyMuted>
                </div>
            </Section>

            <Section
                title="Prose"
                description="Prose styles bare tags: here a heading, paragraphs with a link and code, a list, a quote, a table, a code block and a rule, as rendered Markdown would arrive."
            >
                <Prose class="max-w-xl">
                    <h2>"Release notes"</h2>
                    <p>
                        "Version 0.8 adds a " <a href="#prose">"Toolbar"</a> " and " <code>"NumberField"</code>
                        "."
                    </p>
                    <ul>
                        <li>"Roving focus"</li>
                        <li>"Hold to repeat"</li>
                    </ul>
                    <ol>
                        <li>"Install"</li>
                        <li>"Import"</li>
                    </ol>
                    <blockquote>"Tab once, arrow around."</blockquote>
                    <table>
                        <thead>
                            <tr>
                                <th>"Key"</th>
                                <th>"Action"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"Up"</td>
                                <td>"Step up"</td>
                            </tr>
                            <tr>
                                <td>"PageDown"</td>
                                <td>"Large step down"</td>
                            </tr>
                        </tbody>
                    </table>
                    <pre>"let n = 1 + 2;"</pre>
                    <hr />
                    <p>"The end."</p>
                </Prose>
            </Section>
        </Page>
    }
}
