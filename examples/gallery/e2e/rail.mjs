// Icon rail regression check on the App shells page: tooltips (at most one, follows hover and
// focus, closes on leave, blur, click and Escape, nothing left over from hovering while the sidebar
// was expanded), the group flyout menu (click, ArrowRight, Escape, select, outside press,
// on_navigate, active child), header and user block layout, and a clean console.
//
//   PLAYWRIGHT_DIR=/path/with/node_modules [SHOTS=/dir] node examples/gallery/e2e/rail.mjs [baseUrl]
//
// SHOTS: a directory to write screenshots into (they are for looking at, nothing compares them).
import { createRequire } from 'node:module';
import path from 'node:path';

const dir = process.env.PLAYWRIGHT_DIR;
if (!dir) { console.error('Set PLAYWRIGHT_DIR to a directory with playwright installed.'); process.exit(2); }
const { chromium } = createRequire(path.join(dir, 'x.js'))('playwright');
const base = process.argv[2] || 'http://127.0.0.1:3100';
const shots = process.env.SHOTS;

const browser = await chromium.launch();
const problems = [];
const check = (ok, what) => { if (!ok) problems.push(what); console.log(`${ok ? 'ok  ' : 'FAIL'} ${what}`); };

async function run(theme, width) {
  const ctx = await browser.newContext({ viewport: { width, height: 1000 } });
  const page = await ctx.newPage();
  page.on('console', (m) => { if (['error', 'warning'].includes(m.type())) problems.push(`console ${m.type()}: ${m.text().slice(0, 160)}`); });
  page.on('pageerror', (e) => problems.push(`page error: ${e.message.slice(0, 160)}`));
  await page.goto(base + '/app-shells', { waitUntil: 'networkidle' });
  if (theme === 'dark') await page.evaluate(() => document.documentElement.classList.add('dark'));
  const tag = `${theme}-${width}`;
  const shot = async (name, el) => { if (shots) await (el || page).screenshot({ path: `${shots}/${name}-${tag}.png` }); };
  const tips = () => page.locator('[role=tooltip]').count();
  const settle = () => page.waitForTimeout(350);
  const shell = page.locator('.vdl-shell').first();
  const btns = shell.locator('[data-sidebar="menu-button"]');
  const collapsed = () => shell.locator('.vdl-sidebar[data-collapsible="icon"]').count();
  const ensure = async (want) => { if (((await collapsed()) > 0) !== want) { await page.keyboard.press('Control+b'); await settle(); } };
  const pick = (label) => page.getByRole('button', { name: label, exact: true }).first().click();

  await pick('Icon rail');
  await pick('header_collapsed_slot');
  await shell.scrollIntoViewIfNeeded();
  await settle();

  if (width < 768) {
    // Phone: no rail, the sidebar is a sheet; its user block wraps actions below the text.
    await shell.locator('.vdl-sidebar__trigger').first().click();
    await settle();
    const user = page.locator('.vdl-sidebar__sheet-inner .vdl-shell-user').first();
    check(await user.count() === 1, `${tag}: sheet shows the user block`);
    const actions = await user.locator('.vdl-shell-user__actions').boundingBox();
    const text = await user.locator('.vdl-shell-user__text').boundingBox();
    check(actions && text && actions.y > text.y + text.height - 1, `${tag}: actions sit below the name and email`);
    await shot('sheet');
    await ctx.close();
    return;
  }

  // --- tooltips -----------------------------------------------------------------------------
  // 1. Hovering items while expanded must not leave tooltips behind for when it collapses.
  for (const i of [3, 4, 5]) { await btns.nth(i).hover(); await page.waitForTimeout(80); }
  await page.mouse.move(width - 20, 900);
  await page.keyboard.press('Control+b');
  await settle();
  check(await tips() === 0, `${tag}: no tooltip left over after hovering while expanded, then collapsing`);
  await shot('rail', shell);

  // 2. Hover, then move to the next icon: one tooltip at a time, following the pointer.
  await btns.nth(0).hover(); await settle();
  check(await tips() === 1, `${tag}: hover shows one tooltip`);
  await btns.nth(1).hover(); await settle();
  check(await tips() === 1 && (await page.locator('[role=tooltip]').first().innerText()).trim() === 'Activity', `${tag}: moving to the next icon shows only its tooltip`);
  await page.mouse.move(width - 20, 900); await settle();
  check(await tips() === 0, `${tag}: leaving closes the tooltip`);

  // 3. Keyboard focus, then Tab: focus shows it, Tab moves it (never two).
  await btns.nth(0).focus(); await settle();
  check(await tips() === 1, `${tag}: focus shows a tooltip`);
  await page.keyboard.press('Tab'); await settle();
  check(await tips() === 1, `${tag}: Tab moves the tooltip, never two at once`);
  await page.keyboard.press('Escape'); await settle();
  check(await tips() === 0, `${tag}: Escape closes the tooltip`);

  // 4. Focus on one, hover another: still at most one.
  await btns.nth(0).focus(); await settle();
  await btns.nth(2).hover(); await settle();
  check(await tips() === 1, `${tag}: focus on one and hover on another shows one tooltip`);
  await page.mouse.move(width - 20, 900);
  await btns.nth(0).blur(); await settle();
  check(await tips() === 0, `${tag}: blur and leave closes`);

  // 5. Hover then click a link: closed by the press.
  await btns.nth(0).hover(); await settle();
  await btns.nth(0).click(); await settle();
  check(await tips() === 0, `${tag}: clicking closes the tooltip`);
  await page.mouse.move(width - 20, 900);

  // --- group flyout -------------------------------------------------------------------------
  const flyout = page.locator('.vdl-shell__flyout');
  const group = btns.nth(3); // Alpha
  await group.hover(); await settle();
  check(await tips() === 1, `${tag}: group icon shows its tooltip`);
  await group.click(); await settle();
  check(await flyout.count() === 1 && await tips() === 0, `${tag}: click opens the flyout and the tooltip steps aside`);
  const items = flyout.locator('[role=menuitem]');
  check(await items.count() === 5, `${tag}: flyout lists the group's children`);
  const fb = await flyout.boundingBox();
  const gb = await group.boundingBox();
  check(fb && gb && fb.x >= gb.x + gb.width, `${tag}: flyout opens beside the rail, not over it`);
  await shot('flyout', page.locator('.h-\\[560px\\]').first());
  await page.keyboard.press('ArrowDown'); await page.keyboard.press('ArrowDown');
  await page.keyboard.press('Escape'); await settle();
  check(await flyout.count() === 0, `${tag}: Escape closes the flyout`);
  await page.waitForTimeout(300);
  check(await page.evaluate(() => document.activeElement?.getAttribute('data-sidebar')) === 'menu-button', `${tag}: focus returns to the group button`);
  await page.keyboard.press('Escape'); await settle();

  // ArrowRight opens it with the keyboard; Enter selects and navigates (on_navigate sets the path).
  await group.focus(); await page.keyboard.press('ArrowRight'); await settle();
  check(await flyout.count() === 1, `${tag}: ArrowRight opens the flyout`);
  await page.keyboard.press('ArrowDown'); await page.keyboard.press('Enter'); await settle();
  check(await flyout.count() === 0, `${tag}: choosing an item closes the flyout`);
  const heading = (await shell.locator('h3').first().innerText()).trim();
  check(heading.startsWith('/alpha/'), `${tag}: on_navigate ran (page is ${heading})`);

  // The current page's row is marked, and with child icons the rows have icons.
  await group.click(); await settle();
  check(await flyout.locator('.vdl-shell__menu-label[data-active]').count() === 1, `${tag}: exactly one child is marked as the current page`);
  await page.keyboard.press('Escape'); await settle();
  await pick('Icons'); await settle();
  await ensure(true);
  await group.click(); await settle();
  check(await flyout.locator('.vdl-menu__item svg').count() === 5, `${tag}: flyout rows show the child icons`);
  await shot('flyout-icons', page.locator('.h-\\[560px\\]').first());
  await page.keyboard.press('Escape'); await settle();

  // Outside press closes it, and only one flyout at a time.
  await group.click(); await settle();
  await page.mouse.click(width - 100, 600); await settle();
  check(await flyout.count() === 0, `${tag}: a press outside closes the flyout`);

  // --- header and user block in the rail ----------------------------------------------------
  const rail = await shell.locator('.vdl-sidebar__container').first().boundingBox();
  const mark = await shell.locator('.vdl-shell__header-rail').first().boundingBox();
  check(mark && rail && Math.abs((mark.x + mark.width / 2) - (rail.x + rail.width / 2)) <= 1.5, `${tag}: header_collapsed_slot is centred in the rail`);
  const av = await shell.locator('.vdl-shell-user .vdl-avatar').first().boundingBox();
  check(av && rail && Math.abs((av.x + av.width / 2) - (rail.x + rail.width / 2)) <= 1.5, `${tag}: user avatar is centred in the rail`);
  check(av && rail && av.x - rail.x >= 7, `${tag}: user avatar has at least 8px of room at the edge`);
  await shot('rail-footer', shell);

  // Header without the slot: clipped and centred, no overflow past the rail.
  await pick('Clipped (no help)'); await settle();
  await ensure(true);
  const full = await shell.locator('.vdl-shell__header-full').first().boundingBox();
  const rail2 = await shell.locator('.vdl-sidebar__container').first().boundingBox();
  check(full && rail2 && full.x >= rail2.x && full.x + full.width <= rail2.x + rail2.width, `${tag}: clipped header stays inside the rail`);
  await shot('rail-clipped', shell);
  await pick('hide-collapsed class'); await settle();
  await ensure(true);
  await shot('rail-hide-class', shell);

  // --- expanded: user block on several lines --------------------------------------------------
  await ensure(false);
  const user = shell.locator('.vdl-shell-user').first();
  const name = user.locator('.vdl-shell-user__name');
  check(await name.evaluate((e) => e.scrollWidth <= e.clientWidth + 1), `${tag}: the name is not cut at 256px`);
  const act = await user.locator('.vdl-shell-user__actions').boundingBox();
  const txt = await user.locator('.vdl-shell-user__text').boundingBox();
  check(act && txt && act.y >= txt.y + txt.height - 1, `${tag}: actions are on their own row`);
  await shot('expanded', shell);

  // --- page framing --------------------------------------------------------------------------
  const pageBox = () => page.locator('#shell-page').boundingBox();
  const inset = () => shell.locator('.vdl-shell__scroll').first().boundingBox();
  const before = await pageBox();
  await pick('Full'); await settle();
  await ensure(false);
  const after = await pageBox();
  const area = await inset();
  check(before && after && after.width > before.width && Math.abs(after.width - area.width) <= 2, `${tag}: Full fills the area beside the menu`);
  await shot('full', shell);
  await pick('Contained (default)');

  await ctx.close();
}

for (const theme of ['light', 'dark']) await run(theme, 1280);
await run('light', 390);
await run('dark', 390);
await browser.close();

const real = problems.filter((p) => !/^(ok|FAIL)/.test(p));
if (problems.length) { console.error(`\n${problems.length} problem(s):\n` + problems.join('\n')); process.exit(1); }
console.log('\nrail checks passed');
