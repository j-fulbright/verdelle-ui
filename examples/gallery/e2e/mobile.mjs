// Mobile regression check: loads every gallery page (the paths in src/registry.rs) at 390px wide with touch
// emulation and fails on horizontal overflow or console errors.
//
//   PLAYWRIGHT_DIR=/path/with/node_modules node examples/gallery/e2e/mobile.mjs [baseUrl] [width]
//
// PLAYWRIGHT_DIR is a directory that has `playwright` installed (npm install playwright && npx playwright
// install chromium), kept outside the repo. The gallery must be running (default http://127.0.0.1:3100).
import { createRequire } from 'node:module';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';

const dir = process.env.PLAYWRIGHT_DIR;
if (!dir) { console.error('Set PLAYWRIGHT_DIR to a directory with playwright installed.'); process.exit(2); }
const { chromium } = createRequire(path.join(dir, 'x.js'))('playwright');

const base = process.argv[2] || 'http://127.0.0.1:3100';
const width = Number(process.argv[3] || 390);
const registry = path.join(path.dirname(fileURLToPath(import.meta.url)), '../src/registry.rs');
const paths = [...readFileSync(registry, 'utf8').matchAll(/path:\s*"([^"]+)"/g)].map((m) => m[1]);

const browser = await chromium.launch();
// isMobile stays off: it widens the layout viewport to fit overflowing content and hides the overflow.
const page = await (await browser.newContext({ viewport: { width, height: 844 }, hasTouch: true })).newPage();
let current = '';
const problems = [];
page.on('console', (m) => { if (m.type() === 'error' || m.type() === 'warning') problems.push(`${current}: console ${m.type()}: ${m.text().slice(0, 140)}`); });
page.on('pageerror', (e) => problems.push(`${current}: page error: ${e.message.slice(0, 140)}`));

for (const p of paths) {
  current = p;
  await page.goto(base + p, { waitUntil: 'networkidle' });
  const over = await page.evaluate((w) => {
    const d = document.documentElement;
    if (d.scrollWidth <= w) return null;
    const wide = [...document.querySelectorAll('main *')].find((e) => e.getBoundingClientRect().right > w + 1 && getComputedStyle(e).position !== 'fixed');
    return `${d.scrollWidth - w}px${wide ? ` (first: <${wide.tagName.toLowerCase()} class="${(wide.getAttribute('class') || '').slice(0, 50)}">)` : ''}`;
  }, width);
  if (over) problems.push(`${p}: horizontal overflow ${over}`);
}
await browser.close();
console.log(`${paths.length} pages at ${width}px`);
if (problems.length) { console.error(problems.join('\n')); process.exit(1); }
console.log('ok');
