import { expect, test } from "@playwright/test";

async function openWorkbench(page) {
  await page.goto("/static/");
  await expect(page.getByRole("heading", { name: "color_atlas" })).toBeVisible();
  await expect(page.getByText("#3B82F6", { exact: true }).first()).toBeVisible();
}

test("runs the main workflows through the real WASM boundary", async ({ page }) => {
  const problems = [];
  page.on("console", message => {
    if (["error", "warning"].includes(message.type())) problems.push(`${message.type()}: ${message.text()}`);
  });
  page.on("pageerror", error => problems.push(`pageerror: ${error.message}`));

  await openWorkbench(page);

  await page.getByRole("tab", { name: "Harmony" }).click();
  await expect(page.getByTitle(/Select to copy/)).toHaveCount(1);

  await page.getByRole("tab", { name: "Contrast" }).click();
  await expect(page.getByText("21.00:1", { exact: true })).toBeVisible();
  await expect(page.getByText("PASS - AAA normal", { exact: true })).toBeVisible();

  await page.getByRole("tab", { name: "Gradient" }).click();
  await expect(page.getByText("background: linear-gradient(90deg, #3b82f6, #8b5cf6);", { exact: true })).toBeVisible();

  await page.getByRole("tab", { name: "Vision preview" }).click();
  await expect(page.getByText("Protanopia approximation", { exact: true })).toBeVisible();

  const horizontalOverflow = await page.evaluate(
    () => document.documentElement.scrollWidth > document.documentElement.clientWidth
  );
  expect(horizontalOverflow).toBe(false);
  expect(problems).toEqual([]);
});

test("reports invalid colors without executing markup", async ({ page }) => {
  await openWorkbench(page);
  const input = page.getByPlaceholder("#RRGGBB");
  await input.fill('<img src=x onerror="window.injected=true">');
  await expect(page.getByText(/Unable to convert/)).toBeVisible();
  await expect(page.locator("#format-output img")).toHaveCount(0);
  expect(await page.evaluate(() => window.injected)).toBeUndefined();
});

test("exposes the primary color controls to assistive technology", async ({ page }) => {
  await openWorkbench(page);
  await expect(page.getByLabel("Convert color picker")).toBeVisible();
  await expect(page.getByLabel("Convert hex color")).toBeVisible();

  await page.getByRole("tab", { name: "Contrast" }).click();
  await expect(page.getByLabel("Foreground color picker")).toBeVisible();
  await expect(page.getByLabel("Foreground hex color")).toBeVisible();
});

test("keeps one harmony control active and exposes swatches as buttons", async ({ page }) => {
  await openWorkbench(page);
  await page.getByRole("tab", { name: "Harmony" }).click();

  const complementary = page.getByRole("button", { name: "Complementary" });
  const analogous = page.getByRole("button", { name: "Analogous" });
  await analogous.click();

  await expect(complementary).not.toHaveCSS("background-color", "rgb(59, 130, 246)");
  await expect(analogous).toHaveCSS("background-color", "rgb(59, 130, 246)");
  await expect(page.getByRole("button", { name: /Select to copy/ })).toHaveCount(5);
});
