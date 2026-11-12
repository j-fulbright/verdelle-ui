# CLAUDE.md

Guidance for Claude Code (claude.ai/code) when working in this repository.

## What this is
Verdelle: shadcn/ui components for Leptos, on Base UI-style headless primitives. Early and in active development; the repository is public
on GitHub, and the crate is on crates.io (0.8.0). The first consumer is a separate private app that depends on it from crates.io. See `README.md` for the
overview. Also `docs/reporting-issues.md` (how an app's agents file Verdelle issues without leaking private details; issue templates are in `.github/ISSUE_TEMPLATE/`), `docs/api-coverage.md` (each shadcn component's API against Verdelle's; update the row when a component's props change), `docs/new-app.md`, `docs/presets-and-themes.md` and `docs/using-verdelle.md` (depending on Verdelle, CSS import, features, version policy). CI (`.github/workflows/ci.yml`) mirrors `scripts/check.sh`; keep them in step when you add a check. `build.rs` in verdelle-ui only reports the `css/` folder to dependents (`links`); public items should have doc comments (`cargo doc -p verdelle-ui --no-deps` must stay warning-free).

Planning notes (status, decisions, what is next) are kept locally in `docs/internal/`, which is gitignored and not part of the repository. If you have them, keep them current as things land; if not, `docs/api-coverage.md` and the open GitHub issues are the status.

## Layout
- `crates/verdelle-ui`: the one crate (published to crates.io; releases go through `.github/workflows/publish.yml` with Trusted Publishing: only an admin can create a `v*` tag, and the job waits for approval in the `crates-io` environment). Styled components (`src/*.rs`) and the CSS (`css/components.css`, generated
  `css/tokens.css`, `css/kit.css` entry point, optional `css/tailwind.css`). It contains the headless behaviour as the public
  module `verdelle_ui::primitives` (`src/primitives/`: roles, keyboard, focus, ids, positioning; `core/` holds shared helpers, each
  primitive is one module next to it). Ported from Leptix (MIT); see `NOTICE` and `LICENSES/LEPTIX-MIT.txt`.
- `crates/verdelle-ui/examples/themegen`: turns shadcn-format theme CSS (`crates/verdelle-ui/themes/` by default, or `--themes <dir>`)
  into `tokens.css` and `themes.rs`. Ships with the crate so apps can make their own themes.
- Theming (`preset.rs`, `preset_tokens.rs`, `theme_css.rs`, `custom_theme.rs`, `theme_provider.rs`, `theme_style.rs`): shadcn preset codes, token composition, the shadcn-CSS parser, `ThemeProvider`/`ThemeStyle` and browser-kept custom themes. See `docs/presets-and-themes.md`. Preset logic is checked against shadcn's own functions (fixtures in `crates/verdelle-ui/tests/fixtures/`, regenerate by running `shadcn_oracle.ts` under node in the scratchpad with shadcn's `preset.ts`/`themes.ts`); do not hand-edit the fixtures. Theme state is set only through `ThemeProvider`; the menu CSS reads `data-vdl-menu-*`, never remap tokens for it.
- `examples/gallery`: a cargo-leptos storybook app (SSR + hydration), no external services needed.

## Commands
`.cargo/config.toml` sets `--cfg erase_components` (what `cargo leptos` does in dev). Keep it: without it plain `cargo test`/`clippy` compile the gallery
with full Leptos view types and `target/` grows by 30+ GB; with it a full `./scripts/check.sh` from clean leaves about 4.5 GB.
```bash
./scripts/check.sh                      # fmt, clippy (native, wasm, gallery ssr and hydrate) with -D warnings, tests, themegen: run before every commit
cargo test --workspace
cargo check --workspace --target wasm32-unknown-unknown
cargo run -p verdelle-ui --example themegen            # regenerate tokens.css and themes.rs after changing themes/
cargo run -p verdelle-ui --example themegen -- --check # fails if the generated files are stale
cargo leptos watch                     # run from the repo root or examples/gallery; http://127.0.0.1:3100
```
`wasm-bindgen-cli` must match the `wasm-bindgen` version in `Cargo.lock` (`wasm-bindgen --version`); reinstall it with
`cargo install wasm-bindgen-cli --version <version> --locked` after dependency updates.

Leptos 0.8 is the target (Verdelle 0.8.x, tagged). Leptos 0.9 will be Verdelle 0.9.x (Verdelle's minor matches Leptos's); the library cannot span both.
Stable Rust is enough; do not use nightly-only syntax (use `.get()` / `.set()`, not call syntax on signals).

## Conventions
- One component per file; shared types (variant and size enums) in their own files.
- Components take typed props (`variant`, `size`, ...) and emit `data-*` attributes; the CSS styles those. Do not build class
  strings from props. Every component takes a `class` prop for layout classes.
- CSS prefix is `vdl-` for classes, `data-vdl-*` attributes and our own `--vdl-*` variables. The shadcn token names
  (`--background`, `--primary`, `--ring`, ...) are NOT prefixed; they are the shared standard that makes pasted themes work.
- Component CSS is imported into Tailwind's `components` layer (`kit.css`), so utility classes can override it.
- Follow shadcn/ui's source (default "nova" style (Verdelle supports only nova), `apps/v4/registry/bases/base/ui` in shadcn-ui/ui) for sizes, states and
  tokens: read the real source, do not work from memory. Component names follow shadcn's; extras are named by what they do
  and recorded in `crates/verdelle-ui/NAMING.md`.
- Port behaviour faithfully first, rename toward Base UI second. Keep licence notices (NOTICE files, `LICENSES/`, credit headers).
- Theme and mode are independent: `data-theme` on `<html>`, light/dark is the `.dark` class. Colour stays on shadcn's tokens.
- leptos-use 0.20 is split into micro crates and the umbrella `leptos-use` is deprecated: depend on the one you need
  (`leptos-use-storage`, `leptos-use-window`, ...) and forward its `ssr` feature. Read `cargo leptos build` warnings; deprecations show there.
- Browser-only code (`use_local_storage`, DOM access) must be gated with `#[cfg(feature = "hydrate")]`; leptos-use-storage panics
  during SSR. State set before hydration is not reflected in server-rendered text: set it in an effect after hydration.
- Popper-positioned content (select, popover, menu, tooltip, hover card): `attr:class` / `attr:style` on a primitive content component land on the
  outer positioning wrapper, whose `transform` places it. Style the content through the `class` / `style` props (they reach the inner element); an
  animation or transform on the wrapper replaces the position and the popup flies in from the page corner. Check new overlays by sampling the
  content's rect on each animation frame after opening (see the Select fix).
- Exit animations: a portal must stay mounted until its content has finished animating out (`PortalKeep` / `keep_portal_mounted` in `core/presence`), and `attr:data-state` on a popper content lands on the *wrapper*, so style closing panels through it (`[data-vdl-popper-content-wrapper][data-state="closed"] > .vdl-x`). Native `<dialog>`s use `native_dialog.rs`.
- Menu color/accent (`data-vdl-menu-color`, `data-vdl-menu-accent` on `<html>`, set by the theme provider): a new menu-like popup (shadcn's `cn-menu-target`: menus, select, combobox) must be added to the surface lists in the "Menu colour and accent" block at the end of `components.css` and to `MENU_SURFACES` in `examples/themegen`, then regenerate tokens. Never put a `backdrop-filter` on the popup itself (it re-parents its fixed sub menus); use the `::before` layer.
- `attr:class` (and any `attr:`) on a component whose root is a `<Show>` is lost while it renders nothing; give such parts a `class` prop (`ComboboxItem`) or render the element always with `hidden`.
- Provide context to children with `<Provider value=...>`, not `provide_context` in the body: children built later (hydration) otherwise all see the last provider. Do not rely on `use_id` matching between the server and the browser for ids that code looks up in the DOM; make those ids after hydration (see `core/listbox.rs`).
- Icons come from `verdelle_ui::Icon` / `IconName` (Lucide, generated into `icon_name.rs` by `scripts/gen-icons.py`; add a name to the list in the script and re-run). Do not paste new inline `<svg>` icons into components or gallery pages.
- A popover's content is portalled to `<body>`: any DOM query for something inside it (to focus it, say) must be scoped to the popover's own class, not `document`-wide, or it finds a copy elsewhere in the page, focus leaves, and the dismissable layer closes the popover.
- For a value a parent may set, clear or leave alone, use `use_synced` (`calendar_state.rs`), not `use_controllable_state`, which cannot tell "controlled and empty" from "not controlled".
- A prop that may change after the first render must be a `Signal` (or `MaybeProp`), read at use time. Wrapping a component in `move || view!{ <X flag=sig.get() /> }` does not rebuild it: component props are untracked and the view diff reuses the same DOM element with a fresh component instance (a `Carousel` kept its scroll position that way).
- Drag gestures: decisions (snap, dismiss, who owns the gesture) are pure functions in `verdelle_ui::primitives` (see `drawer/model.rs`), DOM wiring only feeds them. Touch needs touch events (`preventDefault` on `touchmove` is the only way to stop the browser's scroll); mouse/pen use pointer events with window listeners, never pointer capture before a drag is decided (it retargets the click). A `<dialog>` pinned with `bottom`/`right` is moved by any margin a parent's `space-y-*` gives it: `margin: 0 !important`.
- Anything that positions itself with margins is fragile (Tailwind `space-y-*` overrides them); use top/left + transform.
- Code must be `cargo fmt` clean and clippy clean (`-D warnings`) everywhere, the gallery included; `./scripts/check.sh` runs it all.
  The one crate-wide allow is `module_inception` in `primitives` (Leptix's `toast/toast.rs` layout).
- The gallery's sidebar and Overview page both come from one list, `examples/gallery/src/registry.rs`. A new page needs an entry
  there (group, title, what it shows), a route in `app.rs` and a `pub use` in `pages/mod.rs`; tests fail if any is missing. When a
  component is added to an existing page, update that entry's `shows` text.
- Everything built, component or primitive, and every option it gets, must have a gallery example (a page in
  `examples/gallery/src/pages/`, listed in `registry.rs` and routed in `app.rs`). The gallery is how options are validated; primitives without a
  styled component get a page on the raw primitive.
- Tests must exercise real code. Do not write tests that assert on values the test itself just set.
- Never put an application's private details (its code, security findings, internal URLs) in this repository or its history.

## What can and cannot be checked from here
Claude can compile, run `./scripts/check.sh`, and build and run the gallery's server to request pages with `curl` (a good SSR smoke test:
build `verdelle-gallery` with `--no-default-features --features ssr`, run it with `LEPTOS_OUTPUT_NAME=verdelle-gallery
LEPTOS_SITE_ROOT=target/site LEPTOS_SITE_PKG_DIR=pkg LEPTOS_SITE_ADDR=127.0.0.1:3100 LEPTOS_RELOAD_PORT=3101 LEPTOS_ENV=DEV`).
Mobile regression check (every registry page at 390px: horizontal overflow and console errors; keep it passing): with the gallery running, `PLAYWRIGHT_DIR=<dir with playwright installed> node examples/gallery/e2e/mobile.mjs [baseUrl] [width]`. `cargo leptos build` works in the sandbox (Tailwind included), and so does a real browser: Playwright (chromium) is installed
under the session scratchpad (`npm install playwright && npx playwright install chromium`, never inside this repo). Run the
built server (command above) and drive it with a small script: click through the page, read the DOM, take screenshots (toggle
dark with `document.documentElement.classList.add('dark')`) and collect console warnings and errors. Use it to verify visual and
interaction changes instead of asking the user; still ask them for taste calls (spacing, feel) and real-device behaviour.
