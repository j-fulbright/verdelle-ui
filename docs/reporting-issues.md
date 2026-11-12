# Reporting Verdelle problems from an app

Verdelle's repository is public: <https://github.com/j-fulbright/verdelle-ui>. An app that uses Verdelle (a person or an AI agent
working in that app) should report what it finds there instead of working around it silently, so the fix benefits every app
and the workaround can be removed later.

## What goes in a public issue, and what does not
Issues are public and permanent. Write them so they reveal nothing about the app.

- **Never include**: the app's name or company, internal URLs and hostnames, repository or ticket names, customer data, tokens
  or keys, screenshots of real data, or more of the app's code than a minimal generic snippet.
- **Do use**: example names (Alpha, Beta, QA, Admin; `example.com`), a snippet that reproduces the problem in isolation, or a
  gallery page (`cargo leptos watch` in Verdelle, `examples/gallery`) with the steps.
- **Always include**: the Verdelle version (git commit or tag from `Cargo.lock`), the browser and OS, what you expected (link
  shadcn's behaviour when it differs), and what happened.
- **Security problems** are not filed as issues. Use a private advisory:
  <https://github.com/j-fulbright/verdelle-ui/security/advisories/new>.

## Steps
1. **Search first**: `gh issue list -R j-fulbright/verdelle-ui --state all --search "<keywords>"`. If it exists, add a comment
   (a new case or a version) instead of a duplicate.
2. **File it** with the templates (bug or missing feature), and the `from-app` label:
   `gh issue create -R j-fulbright/verdelle-ui --label from-app --label bug --title "<generic title>" --body-file <file>`.
   Titles name the component and the problem ("Select: an item with an empty value shows the placeholder"), never the app.
3. **Work around it in the app if needed**, with a comment that names the issue so the workaround can be found and deleted:
   `// verdelle-ui#123: remove when fixed`.
4. **Do not patch Verdelle from inside the app** (no vendored edits, no forks of components copied into the app to change them
   quietly). Verdelle changes are made in the Verdelle repository.
5. When Verdelle releases the fix, bump the pinned `rev`, delete the workaround, and close the loop on the issue.

## Block to paste into an app's `CLAUDE.md` / `AGENTS.md`
```markdown
## Verdelle (UI library)
This app uses Verdelle (https://github.com/j-fulbright/verdelle-ui, public), pinned by commit in Cargo.toml. Read its docs/api-coverage.md
before building a component yourself: Verdelle may already have it (docs/api-coverage.md lists shadcn's API against Verdelle's).
When you find a Verdelle bug, a missing prop or component, or a layout problem:
1. Search existing issues: `gh issue list -R j-fulbright/verdelle-ui --state all --search "<keywords>"`.
2. If none, file one: `gh issue create -R j-fulbright/verdelle-ui --label from-app --label bug|enhancement ...`.
   The repository is PUBLIC. Never put this app's name, company names, internal URLs, hostnames, repository or ticket names,
   customer data, tokens, real screenshots, or app code in the issue. Use a minimal generic snippet and example names (Alpha,
   Beta, QA, Admin; example.com). Include the Verdelle commit from Cargo.lock, the browser, what you expected and what happened.
   Security problems go through a private advisory, never an issue.
3. Work around it here if you must, with `// verdelle-ui#<number>: remove when fixed`, and list it in the app's own migration notes.
4. Do not edit Verdelle's code inside this repository or copy and quietly modify its components.
```
