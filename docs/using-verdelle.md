# Using Verdelle in an app

Verdelle is one crate, `verdelle-ui` (the headless layer is `verdelle_ui::primitives`), on crates.io and on GitHub
(`https://github.com/j-fulbright/verdelle-ui`). The CSS and feature steps below are the same whichever way the app gets it. See
also [new-app.md](new-app.md) for a complete working layout.

## Depending on it

**From crates.io (the normal way).** While Verdelle is a pre-release, name the version exactly; once `0.8.0` is released the
same line keeps working and picks it up on `cargo update`:

```toml
# app Cargo.toml
[dependencies]
verdelle-ui = "0.8.0-alpha.2"       # a plain "0.8" does not match pre-releases
```

Verdelle's minor version matches Leptos's: `0.8.x` is for Leptos 0.8. Pre-release versions (`0.8.0-alpha.N`) may change the API
between releases.

**Developing against a checkout: a `[patch]`.** For an app that uses the published crate but is being developed against a local
Verdelle, keep this out of the committed files (an uncommitted `.cargo/config.toml` is enough):

```toml
[patch.crates-io]
verdelle-ui = { path = "../verdelle/crates/verdelle-ui" }
```

**Unreleased changes: a git dependency, pinned.** To use a commit that is not released yet, pin it with `rev` (or a `tag`
once tags exist), never a branch:

```toml
verdelle-ui = { git = "https://github.com/j-fulbright/verdelle-ui", rev = "<commit>" }
```

The `crates/verdelle-ui` crate is found inside the workspace by name, so no `path` is needed. The public repository needs no
credentials. For a private fork or mirror, cargo's own git client may fail to authenticate where `git ls-remote` works; set
`CARGO_NET_GIT_FETCH_WITH_CLI=true` so cargo calls `git` (and give CI's `git` a token). `[patch]` for local development of a
git dependency patches that source instead (`[patch."https://github.com/j-fulbright/verdelle-ui"]`). Details and a CI sketch:
[new-app.md](new-app.md).

### Features

The crate has two features, both off by default, and no others:

| Feature | Forwards to |
|---|---|
| `ssr` | `leptos/ssr`, `leptos-use-storage/ssr` |
| `hydrate` | `leptos/hydrate` |

The app forwards its own features, as it does for Leptos:

```toml
[features]
ssr = ["leptos/ssr", "verdelle-ui/ssr"]
hydrate = ["leptos/hydrate", "verdelle-ui/hydrate"]
```

Without `ssr` on the server the theme storage panics; browser-only code in Verdelle is gated on `hydrate`. The crate
must be built on stable Rust (1.88 or newer) and needs `recursion_limit = "256"` in the app crate if the app builds
very large views of its own. Call `verdelle_ui::provide_ui_context()` once near the root of the app.

For a gallery-sized app, add `.cargo/config.toml` with `rustflags = ["--cfg", "erase_components"]` for dev builds
(`cargo leptos` does it in watch mode): without it Leptos compiles full view types and `target/` grows by tens of GB.
## Getting the CSS into the app

The stylesheets live in the crate's `css/` folder and are shipped in the package, and the crate also embeds them:

| File | What |
|---|---|
| `kit.css` | Entry point: `@import "./tokens.css"; @import "./components.css" layer(components);` |
| `tokens.css` | Generated shadcn tokens, one `[data-theme]` block per built-in theme |
| `components.css` | Component styles (nova), plain CSS |
| `tailwind.css` | Optional `@theme inline` mapping so Tailwind utilities can use the colours (`bg-background`, ...) |

For a path dependency the app can `@import` the files directly. For a git or registry dependency the files sit in
cargo's cache (`~/.cargo/registry/src/<index-hash>/verdelle-ui-0.8.0/css` or
`~/.cargo/git/checkouts/<name>-<hash>/<rev>/crates/verdelle-ui/css`), at a path that changes with the version, which an app
cannot write into a stylesheet. The tested answer, which works the same for path, git and registry sources, locally and
in CI, with or without Tailwind in the app:

**Recommended: serve the embedded stylesheet.** `verdelle_ui::kit_css()` returns the whole of `kit.css` (tokens, then the
components in the `components` layer) as a `&'static str`, starting with `@layer theme, base, components, utilities;` so
the layer order holds whichever stylesheet loads first. The app serves it and links it:

```rust
// main.rs, in the axum Router
.route("/verdelle.css", get(|| async {
    ([(header::CONTENT_TYPE, "text/css; charset=utf-8"),
      (header::CACHE_CONTROL, "public, max-age=31536000, immutable")],
     verdelle_ui::kit_css())
}))
// in the page head:  <link rel="stylesheet" href="/verdelle.css?v=<hash of kit_css()>" />
```

The app's Tailwind input then has no Verdelle import at all: `@import "tailwindcss";`, its own `@source "../src/**/*.rs"`,
and (optionally) a copy of `css/tailwind.css` for the colour mapping. Utilities given through a component's `class` prop
still override Verdelle's styles. Without Tailwind in the app, put the string in a `<style>` instead
(`verdelle_ui::kit_css()` inside `<style>{...}</style>`), or serve it the same way. Full steps:
[new-app.md](new-app.md).

**Why not a `build.rs` that copies `css/`?** Verdelle's own `build.rs` does report the folder (`links = "verdelle-ui"`, so an
app's build script reads `DEP_VERDELLE_UI_CSS_DIR`), but cargo-leptos starts Tailwind at the same time as the wasm
compile, before the app's build script has run. On a clean checkout the first `cargo leptos build` therefore fails with
`Can't resolve './verdelle/kit.css'` and the second passes (reproduced with cargo-leptos 0.3.11, Tailwind 4.1.13). That is
fine in a script that runs `cargo check` first, but it is a trap for CI and for new apps.

**Alternative: Tailwind imports Verdelle's file.** If you want Verdelle's CSS inside the app's single Tailwind output, run a
step before cargo-leptos that writes `style/verdelle.css` containing an absolute import of where cargo put the crate
(Tailwind's standalone binary resolves absolute `@import` paths, and `kit.css`'s relative imports then work):

```sh
manifest=$(cargo metadata --format-version 1 | python3 -c 'import json,sys; print(next(p["manifest_path"] for p in json.load(sys.stdin)["packages"] if p["name"]=="verdelle-ui"))')
css="$(dirname "$manifest")/css"
printf '@import "%s/kit.css";\n@import "%s/tailwind.css";\n' "$css" "$css" > style/verdelle.css   # git-ignored
```

and `@import "tailwindcss"; @import "./verdelle.css";` in `style/tailwind.css`. `cargo metadata` compiles nothing and works
for all three source kinds. The cost is one more step in every build (local and CI) and a generated file.

Other options and why they lost: committing a copy of `css/` (works, goes stale, needs a refresh script); a hard-coded
cargo cache path (breaks on every version bump); `cargo-leptos` settings (`tailwind-input-file`, `style-file`,
`assets-dir`) have no hook that runs before Tailwind.

Themes an app adds itself do not need any of this: `cargo run -p verdelle-ui --example themegen -- --themes <dir> --out <dir>`
(run from a checkout of Verdelle, or `cargo run --example themegen` inside it) writes the app's own `tokens.css` and
theme list; see `presets-and-themes.md`.


## Package contents

`cargo package -p verdelle-ui --list` shows what ships (`include` in `Cargo.toml`): `src/`, `css/`, `themes/`,
`examples/themegen`, `build.rs`, `NOTICE`, `LICENSE`, `LICENSES/`, `NAMING.md`, `README.md`, and the two JSON fixtures
the unit tests read (`tests/fixtures/preset_*.json`) plus `tests/css_structure.rs`. Not shipped: the Node oracle that
regenerated the fixtures (`tests/fixtures/shadcn_oracle.ts`) and `docs/` (it is in the repository, outside the
crate). Every `include_str!` points inside the package. Size at the time of writing (`cargo package -p verdelle-ui
--allow-dirty --no-verify`): 538 files, 2.3 MiB unpacked, about 504 KiB compressed (crates.io allows 10 MiB).

The licence expression is `MIT AND Apache-2.0 AND ISC`: Verdelle's code is MIT, `Indicator` derives from
gpui-component (Apache-2.0) and the icon set is Lucide (ISC); Leptix and shadcn/ui are MIT. The texts are in
`LICENSES/` and the credits in `NOTICE`.

## Version policy

- Verdelle's minor equals Leptos's: `0.8.x` targets Leptos 0.8, `0.9.x` will target Leptos 0.9. The patch is Verdelle's own.
  A component library cannot span two Leptos versions because Leptos types are in its public API.
- The crate stays at `0.8.0` until the first publish; after that, patch releases (`0.8.1`, ...) are Verdelle's own and
  the minor stays pinned to Leptos's. Say in the release notes when a patch changes an API.
- Releases are git tags (`v0.8.0`). When 0.9 work starts, `main` tracks 0.9 and a `leptos-0.8` branch takes fixes.
- Keep dependencies current; `wasm-bindgen-cli` must match the locked `wasm-bindgen`.
