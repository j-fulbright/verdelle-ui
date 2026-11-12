# Starting a new app on Verdelle

A step-by-step recipe, built and run from clean states: a fresh Leptos 0.8 SSR + hydrate app that depends on `verdelle-ui`
as a path, a git dependency and a registry package, in each case checked in Chromium (hydration, styles, dark mode,
dialog, select, data table, no console messages). The app it was checked with (a sidebar shell with three routes, a Card, a form with Input
and Select, a Dialog, a Data Table, a `ThemeSwitcher` and a `ModeToggle`) is not part of this repository; the files below
are what it consisted of, and the gallery (`examples/gallery`) is the in-repo example of a working app.

## The recipe in one paragraph

Verdelle's CSS is not a file the app has to find. The crate embeds it, and `verdelle_ui::kit_css()` returns it as one string
(theme tokens, then the component styles in Tailwind's `components` layer, with the layer order stated first). The
server serves that string at `/verdelle.css`, the page links it, and the app's own Tailwind step builds only the app's own
utilities. There is no `build.rs`, no copied folder and no path into cargo's cache, so it behaves the same for a path,
git or registry dependency, locally and in CI, and there is no ordering to get wrong between cargo-leptos's Tailwind
step and the Rust build.

## Layout of the app

```
my-app/
  Cargo.toml              verdelle-ui dependency, forwarded features, [package.metadata.leptos]
  rust-toolchain.toml     stable + wasm32-unknown-unknown
  .cargo/config.toml      rustflags = ["--cfg", "erase_components"]   (small, fast debug builds)
  .gitignore              target/
  style/tailwind.css      the app's Tailwind input (no Verdelle import) + the colour mapping
  src/
    main.rs               axum server: Leptos routes, and GET /verdelle.css -> verdelle_ui::kit_css()
    lib.rs                hydrate() entry point
    app.rs                shell() (head: theme init script, /verdelle.css link), App, ThemeProvider, Router, AppShell
    pages/                one file per route (dashboard.rs, servers.rs, settings.rs)
```

## Steps

**1. Tools** (once per machine):

```sh
rustup toolchain install stable --target wasm32-unknown-unknown   # Rust 1.88 or newer
cargo install cargo-leptos --locked                                # 0.3.x
cargo install wasm-bindgen-cli --version <the wasm-bindgen version in Cargo.lock> --locked
```

Tailwind needs no install: cargo-leptos downloads its standalone Tailwind v4 binary on first use.
**Known local issue (macOS):** the binary cargo-leptos picks by default (v4.1.17 at the time of writing) was killed
with SIGKILL on one Mac; `LEPTOS_TAILWIND_VERSION=v4.1.13 cargo leptos build` works. Set it in your shell profile or in
the build step if you see `Tailwind` die without a message.

**2. Create the app** from the [start-axum](https://github.com/leptos-rs/start-axum) template and apply steps 3 to 6
(the package name appears in `Cargo.toml`'s `output-name`, in the `/pkg/<name>.css` link in `app.rs` and in `use
<crate_name>::app::*` in `main.rs`).

**3. Add Verdelle and forward its features** in `Cargo.toml`. Keep exactly one dependency line:

```toml
[dependencies]
verdelle-ui = { path = "../verdelle/crates/verdelle-ui" }                                   # a checkout next to the app
# verdelle-ui = { git = "https://github.com/j-fulbright/verdelle-ui", tag = "v0.8.0" }  # CI, and anyone without a checkout
# verdelle-ui = "0.8"                                                                # after publication

[features]
hydrate = ["leptos/hydrate", "verdelle-ui/hydrate", "dep:console_error_panic_hook", "dep:wasm-bindgen"]
ssr = ["dep:axum", "dep:tokio", "dep:leptos_axum", "leptos/ssr", "verdelle-ui/ssr",
       "leptos_meta/ssr", "leptos_router/ssr"]

[package.metadata.leptos]
tailwind-input-file = "style/tailwind.css"      # not `style-file`
bin-features = ["ssr"]
bin-default-features = false
lib-features = ["hydrate"]
lib-default-features = false
```

`verdelle-ui/ssr` matters: without it the theme storage panics on the server. Add `#![recursion_limit = "256"]` to `lib.rs`
for larger pages, and `.cargo/config.toml` with `rustflags = ["--cfg", "erase_components"]` (without it a debug build
of a Verdelle app gets very large).

**4. Serve Verdelle's stylesheet** in `main.rs` (the route goes before `leptos_routes`):

```rust
use axum::{Router, http::header, routing::get};

let app = Router::new()
    .route("/verdelle.css", get(|| async {
        (
            [
                (header::CONTENT_TYPE, "text/css; charset=utf-8"),
                (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
            ],
            verdelle_ui::kit_css(),
        )
    }))
    .leptos_routes(&leptos_options, routes, { /* as in the template */ })
    /* ... */;
```

The page links it with a hash of the content in the URL, so it is cached for a year and still changes whenever Verdelle
does (see `shell` in your `app.rs`):

```rust
#[cfg(feature = "ssr")]
fn verdelle_css_version() -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let mut h = DefaultHasher::new();
    verdelle_ui::kit_css().hash(&mut h);
    format!("{:x}", h.finish())
}
// in <head>:  <link rel="stylesheet" href=format!("/verdelle.css?v={}", verdelle_css_version()) />
```

(Without a hash, use `Cache-Control: no-cache`.) The stylesheet is about 250 KB of CSS, 42 KB gzipped; add
tower-http's `CompressionLayer` if you want it compressed.

**5. The app's Tailwind input**, `style/tailwind.css`: no Verdelle import. The `@theme inline` block is a copy of
`crates/verdelle-ui/css/tailwind.css` (it maps shadcn's tokens to utilities such as `bg-background` and
`text-muted-foreground`; the app owns its copy, as with shadcn's `globals.css`):

```css
@import "tailwindcss";
@source "../src/**/*.rs";
@variant dark (&:where(.dark, .dark *));

@theme inline { --color-background: var(--background); /* ... the rest of crates/verdelle-ui/css/tailwind.css */ }

@layer base { *, ::before, ::after { border-color: var(--border); } }   /* Tailwind v4's `border` is currentColor */
body { background-color: var(--background); color: var(--foreground); margin: 0; }
```

Utility classes (`flex`, `gap-2`, `max-w-md`, ...) given to a component's `class` prop still override Verdelle's styles:
`/verdelle.css` begins with `@layer theme, base, components, utilities;`, so the order is right whichever sheet loads first.
Verdelle's own styles are plain CSS keyed on `vdl-` classes and `data-*` attributes, so Tailwind needs no `@source` for
them; it only scans the app's `src/`.

**6. The shell and the app** (`app.rs`): the head prints `theme_init_script(...)` (no flash of the wrong
theme) and links `/verdelle.css`; `App` calls `verdelle_ui::provide_ui_context()`, links the Tailwind output
(`<Stylesheet href="/pkg/<name>.css"/>`), and wraps a `Router` in `<ThemeProvider script=false>`; inside the router an
`AppShell` takes an `AppNav`, `current_path` from `use_location()` and `on_navigate` from `use_navigate()`.

**7. Build and run:**

```sh
LEPTOS_TAILWIND_VERSION=v4.1.13 cargo leptos build     # or `cargo leptos watch`
LEPTOS_SITE_ADDR=127.0.0.1:3000 LEPTOS_OUTPUT_NAME=<name> LEPTOS_SITE_ROOT=target/site \
  LEPTOS_SITE_PKG_DIR=pkg LEPTOS_ENV=DEV target/debug/<name>      # run the built server by hand
```

A clean build of the test app took about 90 seconds; its `target/` is 5.5 GB after a debug `cargo leptos build` (the
server and the wasm build are separate target trees). Release builds were not measured.

## Why not import Verdelle's CSS file from Tailwind?

It is possible, but the file's location is the problem, and so is the order of the build steps. Facts measured with
cargo-leptos 0.3.11 and Tailwind 4.1.13:

- cargo-leptos starts the Tailwind step **at the same time as** the wasm (front) compile (`build_frontend` in its
  `command/build.rs` joins front, assets and style), and the server compile is concurrent with them or just after.
  The app's `build.rs` runs inside one of those compiles, long after Tailwind has read its input, because the
  dependencies compile first (about a minute here; Tailwind takes about 100 ms).
- So a `build.rs` that copies Verdelle's `css/` (reading `DEP_VERDELLE_UI_CSS_DIR`, which Verdelle's own `build.rs` sets) cannot
  work on a clean checkout: the first `cargo leptos build` fails with `Can't resolve './verdelle/kit.css'` from Tailwind
  and the folder exists only afterwards. The second run passes. That is why the recipe above does not use it.
- Tailwind (the standalone v4 binary) does resolve an **absolute** `@import "/abs/path/kit.css"`, and the relative
  imports inside `kit.css` then work. `file://` URLs do not.

If you want Verdelle's CSS to go through Tailwind anyway (minified with the rest, one stylesheet), write the import into a
file *before* cargo-leptos runs, with a step that does not compile anything:

```sh
#!/bin/sh
# scripts/verdelle-css.sh: writes style/verdelle.css (git-ignored): where cargo put verdelle-ui, as an absolute import
set -e
manifest=$(cargo metadata --format-version 1 | python3 -c 'import json,sys; print(next(p["manifest_path"] for p in json.load(sys.stdin)["packages"] if p["name"]=="verdelle-ui"))')
css="$(dirname "$manifest")/css"
printf '@import "%s/kit.css";\n@import "%s/tailwind.css";\n' "$css" "$css" > style/verdelle.css
```

```css
/* style/tailwind.css */
@import "tailwindcss";
@import "./verdelle.css";      /* tokens + components layer + the colour mapping */
```

Run `./scripts/verdelle-css.sh` before `cargo leptos` (locally after a Verdelle update, and as a CI step). `cargo metadata`
fetches the crate but compiles nothing, takes well under a second once fetched, and works for path, git and registry
sources (tested against a registry source: it printed `~/.cargo/registry/src/<index>/verdelle-ui-0.8.0/css`).
A `cargo check` before cargo-leptos also works with the `build.rs` copy, because it runs the build script first, but that
compiles the whole app twice. Do not commit `style/verdelle.css` (it holds a machine-specific path).

## Developing against a Verdelle checkout while depending on git or the registry

Keep the app's `Cargo.toml` on the git tag, and point cargo at the checkout from an **uncommitted**
`.cargo/config.toml` (or your user-level one):

```toml
# git dependency: name the same URL as in Cargo.toml
[patch."https://github.com/j-fulbright/verdelle-ui"]
verdelle-ui = { path = "../verdelle/crates/verdelle-ui" }
# registry dependency instead:
# [patch.crates-io]
# verdelle-ui = { path = "../verdelle/crates/verdelle-ui" }
```

(Tested for the git form: cargo still contacts the git URL once, to see what it patches, so the credentials in the next
section are needed even then.) The served stylesheet comes from whatever `verdelle-ui` is built, so a patched checkout's CSS
edits show up after the server rebuilds. Do not commit the patch: CI must build against the tag.

## CI

Anything that builds the app needs: Rust stable with the wasm target, cargo-leptos, wasm-bindgen-cli at the
`Cargo.lock` version, and access to Verdelle's source.

**Verdelle as a git dependency (public repository).** `https://github.com/j-fulbright/verdelle-ui` is public, so cargo needs no
credentials and none of the setup below. It is for a private fork or mirror: cargo's built-in git client does not use your
git credentials; with a token, make cargo call the `git` binary and give git the token:

```sh
export CARGO_NET_GIT_FETCH_WITH_CLI=true
git config --global url."https://x-access-token:${VERDELLE_READ_TOKEN}@github.com/".insteadOf "https://github.com/"
```

`VERDELLE_READ_TOKEN` is a token with read access to the repository (a fine-grained personal access token, a GitHub
App installation token, or a deploy key with `ssh://` URLs and `ssh-agent`). Cargo's built-in git client does not use a
credential helper, so for a private fork set `CARGO_NET_GIT_FETCH_WITH_CLI=true`. Pin by `tag` (or `rev`);
`Cargo.lock` holds the exact commit either way, and CI should build with `--locked`.

**Verdelle from crates.io:** nothing special; `Cargo.lock` pins the version (`verdelle-ui = "0.8.0-alpha.1"` while it is a pre-release).

**Caching.** Cache `~/.cargo/registry`, `~/.cargo/git`, `target/` and cargo-leptos's own cache (`~/.cache/cargo-leptos` on
Linux: the Tailwind binary, wasm-opt) keyed on `Cargo.lock`; for example `Swatinem/rust-cache` plus a cache step for the
cargo-leptos directory. Install cargo-leptos and wasm-bindgen-cli from binaries (`cargo binstall`, or a prebuilt-binary
action) rather than compiling them every run. Because the recipe has no CSS pre-step, nothing else has to run before
`cargo leptos build --release`.

A GitHub Actions sketch (not run from here; adjust the token secret name and tool versions):

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    env:
      CARGO_NET_GIT_FETCH_WITH_CLI: "true"
    steps:
      - uses: actions/checkout@v4
      - run: git config --global url."https://x-access-token:${{ secrets.VERDELLE_READ_TOKEN }}@github.com/".insteadOf "https://github.com/"
      - uses: dtolnay/rust-toolchain@stable
        with: { targets: wasm32-unknown-unknown }
      - uses: Swatinem/rust-cache@v2
      - uses: taiki-e/install-action@v2
        with: { tool: "cargo-leptos,wasm-bindgen-cli@<version in Cargo.lock>" }
      - run: cargo leptos build --release
```

## What was tested

| Verdelle came from | How | Result |
|---|---|---|
| a path | `path = "../verdelle/crates/verdelle-ui"` | clean `cargo leptos build` (Tailwind v4.1.13), hydration, styles, dark mode, dialog, select, data table, router navigation, console clean |
| a git dependency | `git = "file:///.../verdelle", branch = ...` (a local clone; the GitHub URL, then still private, was also reachable with `CARGO_NET_GIT_FETCH_WITH_CLI=true`), checked out to `~/.cargo/git/checkouts/verdelle-<hash>/<rev>/` | same |
| the registry layout | `cargo package -p verdelle-ui` (works while `publish = false`), then a small local registry (git index with a `file://` download URL), extracted to `~/.cargo/registry/src/<registry>/verdelle-ui-0.8.0/` | same |

Not run: a release build, a Linux or Windows machine, a CI run. (These checks were made before the crate was on crates.io; the crates.io source works the same way as the local registry layout above.)
