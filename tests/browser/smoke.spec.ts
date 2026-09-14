import { type ConsoleMessage, expect, type Locator, type Page, test } from "@playwright/test";
import { Font } from "cfonts";

/** The names the page fills its font pickers with, read from the same enum */
const FONT_NAMES = Object.keys(Font).filter((key) => Number.isNaN(Number(key)));

function captureMessages(page: Page, type: string): ConsoleMessage[] {
	const messages: ConsoleMessage[] = [];

	page.on("console", (message) => {
		if (message.type() === type) {
			messages.push(message);
		}
	});

	return messages;
}

function capturePageErrors(page: Page): Error[] {
	const errors: Error[] = [];

	page.on("pageerror", (error) => {
		errors.push(error);
	});

	return errors;
}

/** The format string and the style arguments of one console call, as the page passed them */
async function consoleArguments(message: ConsoleMessage): Promise<[string, string[]]> {
	const values = await Promise.all(message.args().map((argument) => argument.jsonValue()));
	const [format, ...styles] = values;

	if (typeof format !== "string" || !styles.every((style) => typeof style === "string")) {
		throw new TypeError("Expected a console format string followed by style strings");
	}

	return [format, styles as string[]];
}

async function expectStyledConsoleMessage(message: ConsoleMessage): Promise<[string, string[]]> {
	const [format, styles] = await consoleArguments(message);

	expect(styles.length).toBeGreaterThan(0);
	expect(format.match(/%c/g) ?? []).toHaveLength(styles.length); // one style per marker
	expect(styles[0]).toMatch(/^color:/);

	return [format, styles];
}

/** The background of every banded row in a rendered canvas */
function bands(canvas: Locator): Promise<string[]> {
	return canvas
		.locator("div[style*='background:']")
		.evaluateAll((rows) => rows.map((row) => (row as HTMLElement).style.background));
}

test("the page renders a styled banner and fills the font pickers from the enum", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const banner = page.locator("#canvas > div");

	await expect(banner).toBeVisible();
	await expect(banner).toHaveCSS("white-space", "pre");
	await expect(page.locator("#canvas span[style*='color:']").first()).toBeAttached();
	await expect(page.locator("#canvas")).not.toContainText("%c");

	for (const picker of ["#browser_font", "#console_font"]) {
		await expect(page.locator(`${picker} option`)).toHaveText(FONT_NAMES);
		await expect(page.locator(picker)).toHaveValue("Huge");
	}

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("BrowserConsoleEnv produces a reusable console artifact", async ({ page }) => {
	const messages = captureMessages(page, "info");
	const errors = capturePageErrors(page);
	const messagePromise = page.waitForEvent("console", (message) => message.type() === "info");

	await page.goto("/");

	const message = await messagePromise;

	expect(messages).toHaveLength(1);
	await expectStyledConsoleMessage(message);
	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the browser form re-renders the canvas with the picked font, colors and background", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const canvas = page.locator("#canvas");
	const form = page.locator("#browser_form");
	expect(await bands(canvas)).toEqual([]); // the first render carries no background

	/** Submits the form with one choice per picker and waits for the banded re-render */
	async function submit(colors: string, background: string, font: string): Promise<string[]> {
		await form.getByLabel("Text").fill("Playwright");
		await form.getByLabel("Colors").selectOption(colors);
		await form.getByLabel("Background").selectOption(background);
		await form.locator("#browser_font").selectOption(font);
		await form.getByRole("button", { name: "Show" }).click();
		await expect(canvas.locator("div[style*='background:']").first()).toBeAttached();

		return bands(canvas);
	}

	// one static background paints every row the same
	const tiny = await submit("blue", "red", "Tiny");
	expect(tiny.length).toBeGreaterThan(1);
	expect(new Set(tiny).size).toBe(1);

	// a taller font adds rows, so the picked font reaches the render
	const huge = await submit("blue", "red", "Huge");
	expect(huge.length).toBeGreaterThan(tiny.length);

	// the agender preset paints a pale green no other block on the page uses, so the picked colors reach it too
	await expect(canvas.locator("span[style*='#b8f483']")).toHaveCount(0);
	await submit("agender", "red", "Tiny");
	await expect(canvas.locator("span[style*='#b8f483']").first()).toBeAttached();

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the console form logs one styled call per submit with the picked background", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);

	await page.goto("/");

	const form = page.locator("#console_form");
	await form.getByLabel("Text").fill("Playwright");
	await form.getByLabel("Background").selectOption("red");
	await form.locator("#console_font").selectOption("Tiny");

	const messagePromise = page.waitForEvent("console", (message) => message.type() === "log");
	await form.getByRole("button", { name: "Show in devtool console" }).click();
	const [, styles] = await expectStyledConsoleMessage(await messagePromise);

	expect(messages).toHaveLength(1);
	expect(styles.some((style) => style.includes("background:"))).toBe(true);

	// the default background is the terminal's own, the next submit paints none
	await form.getByLabel("Background").selectOption("system");
	const plainPromise = page.waitForEvent("console", (message) => message.type() === "log");
	await form.getByRole("button", { name: "Show in devtool console" }).click();
	const [, plainStyles] = await expectStyledConsoleMessage(await plainPromise);

	expect(messages).toHaveLength(2);
	expect(plainStyles.some((style) => style.includes("background:"))).toBe(false);

	expect(errors.map((error) => error.message)).toEqual([]);
});
