//! A bad merge once spliced one component's rules into the middle of another's, leaving
//! `.a { ... .b { ... } }`: the braces still balanced, so nothing complained, but browsers
//! read the inner rules as nested and the components lost their styling. Only at-rules
//! (`@media`, `@keyframes`, ...) may contain blocks in these files.

fn check(name: &str, css: &str) {
    let mut stripped = String::new();
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        stripped.push_str(&rest[..start]);
        let end = rest[start..].find("*/").expect("unterminated comment") + start + 2;
        rest = &rest[end..];
    }
    stripped.push_str(rest);

    // Each open block remembers whether it is an at-rule.
    let mut stack: Vec<bool> = Vec::new();
    let mut prelude = String::new();
    for ch in stripped.chars() {
        match ch {
            '{' => {
                let is_at = prelude.trim_start().starts_with('@');
                assert!(
                    stack.last() != Some(&false),
                    "{name}: rule `{}` opens inside another style rule",
                    prelude.trim()
                );
                stack.push(is_at);
                prelude.clear();
            }
            '}' => {
                assert!(stack.pop().is_some(), "{name}: unmatched `}}`");
                prelude.clear();
            }
            ';' => prelude.clear(),
            _ => prelude.push(ch),
        }
    }
    assert!(stack.is_empty(), "{name}: unclosed block");
}

#[test]
fn components_css_has_no_nested_style_rules() {
    check("components.css", include_str!("../css/components.css"));
}

#[test]
fn kit_css_matches_the_kit_entry_point() {
    let css = verdelle_ui::kit_css();
    check("kit_css()", css);
    // Layer order first, so it holds whichever stylesheet loads first.
    assert!(css.starts_with("@layer theme, base, components, utilities;"));
    // Tokens sit outside any layer (as in kit.css); components are inside `components`.
    let tokens = css
        .find(verdelle_ui::themes::TOKENS_CSS)
        .expect("tokens included");
    let layer = css.find("@layer components {").expect("components layer");
    assert!(tokens < layer);
    assert!(css[layer..].contains(verdelle_ui::COMPONENTS_CSS));
    // kit.css imports exactly these two files; if it changes, kit_css() must follow.
    let kit = include_str!("../css/kit.css");
    assert!(kit.contains("@import \"./tokens.css\";"));
    assert!(kit.contains("@import \"./components.css\" layer(components);"));
}

#[test]
fn checker_catches_a_spliced_rule() {
    let spliced = std::panic::catch_unwind(|| {
        check("x", ".a { color: red;\n.b { color: blue; }\n}");
    });
    assert!(spliced.is_err());
    check(
        "x",
        "@media (x) { .a { color: red; } }\n.b { color: blue; }",
    );
}
