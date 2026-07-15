// The one test that runs the built bundle in a real browser:
// bundling alone does not instantiate the WASM, this does
import { expect, test } from "@playwright/test";

test("the browser bundle initializes the WASM and renders the banner", async ({ page }) => {
	await page.goto("/");

	await expect(page.locator("#banner div")).toContainText("█");
});
