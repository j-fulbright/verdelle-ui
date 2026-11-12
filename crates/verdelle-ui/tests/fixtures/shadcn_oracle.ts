// Generates preset_codes.json and preset_tokens.json from shadcn's own functions.
// Setup (outside the repo): copy packages/registry/src/preset/preset.ts and apps/v4/registry/themes.ts from
// shadcn-ui/ui next to this file (delete themes.ts's first import line and its `RegistryItem` types), then
//   node --experimental-strip-types shadcn_oracle.ts <outdir>
// Runs shadcn's own preset functions to produce expected values for Verdelle's tests.
// node --experimental-strip-types oracle.ts <outdir>
import { writeFileSync } from "node:fs"
import {
  decodePreset,
  encodePreset,
  PRESET_BASE_COLORS,
  PRESET_FONT_HEADINGS,
  PRESET_FONTS,
  PRESET_ICON_LIBRARIES,
  PRESET_MENU_ACCENTS,
  PRESET_MENU_COLORS,
  PRESET_RADII,
  PRESET_STYLES,
  PRESET_THEMES,
  V1_CHART_COLOR_MAP,
} from "./preset.ts"
import { THEMES } from "./themes.ts"

const out = process.argv[2] ?? "."

const BASE_COLORS = THEMES.filter((t: any) =>
  ["neutral", "stone", "zinc", "mauve", "olive", "mist", "taupe"].includes(t.name)
)
const RADII = [
  { name: "default", label: "Default", value: "" },
  { name: "none", label: "None", value: "0" },
  { name: "small", label: "Small", value: "0.45rem" },
  { name: "medium", label: "Medium", value: "0.625rem" },
  { name: "large", label: "Large", value: "0.875rem" },
]
const getTheme = (n: string) => (THEMES as any[]).find((t) => t.name === n)
const getBaseColor = (n: string) => (BASE_COLORS as any[]).find((t) => t.name === n)

// Copied from apps/v4/registry/config.ts (buildRegistryTheme + buildThemeForPreset).
function buildRegistryTheme(config: any) {
  const baseColor = getBaseColor(config.baseColor)
  const theme = getTheme(config.theme)
  if (!baseColor || !theme) return null
  const lightVars: Record<string, string> = { ...baseColor.cssVars?.light, ...theme.cssVars?.light }
  const darkVars: Record<string, string> = { ...baseColor.cssVars?.dark, ...theme.cssVars?.dark }
  const chartTheme = getTheme(config.chartColor)
  if (chartTheme) {
    for (let i = 1; i <= 5; i++) {
      const key = `chart-${i}`
      if (chartTheme.cssVars?.light?.[key]) lightVars[key] = chartTheme.cssVars.light[key]
      if (chartTheme.cssVars?.dark?.[key]) darkVars[key] = chartTheme.cssVars.dark[key]
    }
  }
  if (config.menuAccent === "bold") {
    lightVars.accent = lightVars.primary
    lightVars["accent-foreground"] = lightVars["primary-foreground"]
    darkVars.accent = darkVars.primary
    darkVars["accent-foreground"] = darkVars["primary-foreground"]
  }
  if (config.radius && config.radius !== "default") {
    const radius = RADII.find((r) => r.name === config.radius)
    if (radius && radius.value) lightVars.radius = radius.value
  }
  return { light: lightVars, dark: darkVars }
}

// Deterministic PRNG so the fixtures are reproducible.
let seed = 12345
const rnd = () => {
  seed = (seed * 1664525 + 1013904223) % 4294967296
  return seed / 4294967296
}
const pick = <T,>(a: readonly T[]) => a[Math.floor(rnd() * a.length)]

const fields = {
  style: PRESET_STYLES,
  baseColor: PRESET_BASE_COLORS,
  theme: PRESET_THEMES,
  chartColor: PRESET_THEMES,
  iconLibrary: PRESET_ICON_LIBRARIES,
  font: PRESET_FONTS,
  fontHeading: PRESET_FONT_HEADINGS,
  radius: PRESET_RADII,
  menuAccent: PRESET_MENU_ACCENTS,
  menuColor: PRESET_MENU_COLORS,
}

const codes: any[] = []
const add = (cfg: any) => {
  const code = encodePreset(cfg)
  const decoded = decodePreset(code)
  codes.push({ code, config: decoded })
}
add({})
// Each field at each of its values, the rest default.
for (const [key, values] of Object.entries(fields)) {
  for (const v of values) add({ [key]: v })
}
// 300 random configs.
for (let i = 0; i < 80; i++) {
  add(Object.fromEntries(Object.entries(fields).map(([k, v]) => [k, pick(v as any)])))
}
// Out-of-range indexes decode to the default (index 0); also v1 ("a") codes.
const raw: any[] = []
for (const code of ["b2oE5cW2a", "a0", "b0", "a2oE5c", "aZZZZZZZZ", "bZZZZZZZZ", "a1", "b1", "bz"]) {
  raw.push({ code, config: decodePreset(code) })
}

// Composition: pick configs where theme is valid for the base colour (as shadcn requires).
const baseNames = BASE_COLORS.map((b: any) => b.name)
const composed: any[] = []
const configs: any[] = []
for (const base of baseNames) {
  for (const theme of PRESET_THEMES) {
    if (baseNames.includes(theme) && theme !== base) continue
    configs.push({ baseColor: base, theme, chartColor: theme })
  }
}
for (let i = 0; i < 12; i++) {
  const baseColor = pick(baseNames)
  const themes = PRESET_THEMES.filter((t: string) => !baseNames.includes(t) || t === baseColor)
  configs.push({
    baseColor,
    theme: pick(themes),
    chartColor: pick(themes),
    radius: pick(PRESET_RADII),
    menuAccent: pick(PRESET_MENU_ACCENTS),
    style: "vega",
  })
}
let n = 0
for (const c of configs) {
  if (n++ % 6 !== 0 && n < configs.length - 12) continue
  const full = { style: "vega", radius: "default", menuAccent: "subtle", ...c }
  const tokens = buildRegistryTheme(full)
  if (!tokens) continue
  composed.push({ code: encodePreset(full), tokens })
}

writeFileSync(`${out}/preset_codes.json`, JSON.stringify({ codes, raw, v1ChartColorMap: V1_CHART_COLOR_MAP }))
writeFileSync(`${out}/preset_tokens.json`, JSON.stringify(composed))
console.log("codes", codes.length, "composed", composed.length)
console.log(JSON.stringify(decodePreset("b2oE5cW2a")))
