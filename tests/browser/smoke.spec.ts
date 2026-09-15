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
		expect(FONT_NAMES).toContain(await page.locator(picker).inputValue()); // the pickers open on a real font
	}

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the page prints the devtools banner once on load", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);
	const messagePromise = page.waitForEvent("console", (message) => message.type() === "log");

	await page.goto("/");

	const message = await messagePromise;

	expect(messages).toHaveLength(1);
	await expectStyledConsoleMessage(message);
	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the browser form re-renders the canvas on every pick with the font, colors and background", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const canvas = page.locator("#canvas");
	const form = page.locator("#browser_form");
	expect(await bands(canvas)).toEqual([]); // the first render carries no background

	/** Fills one choice per picker, every pick re-renders, and waits for the banded render */
	async function pick(colors: string, background: string, font: string): Promise<string[]> {
		await form.getByLabel("Text").fill("Playwright");
		await form.getByLabel("Colors").selectOption(colors);
		await form.getByLabel("Background").selectOption(background);
		await form.locator("#browser_font").selectOption(font);
		await expect(canvas.locator("div[style*='background:']").first()).toBeAttached();

		return bands(canvas);
	}

	// one static background paints every row the same
	const tiny = await pick("cyan", "red", "Tiny");
	expect(tiny.length).toBeGreaterThan(1);
	expect(new Set(tiny).size).toBe(1);

	// a taller font adds rows, so the picked font reaches the render
	const huge = await pick("cyan", "red", "Huge");
	expect(huge.length).toBeGreaterThan(tiny.length);

	// the agender preset paints a pale green no other block on the page uses, so the picked colors reach it too
	await expect(canvas.locator("span[style*='#b8f483']")).toHaveCount(0);
	await pick("agender", "red", "Tiny");
	await expect(canvas.locator("span[style*='#b8f483']").first()).toBeAttached();

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the console form prints one styled banner per pick and one more on enter", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);
	const loadPromise = page.waitForEvent("console", (message) => message.type() === "log");

	await page.goto("/");
	await loadPromise; // the banner the page prints on load, every pick and every enter adds one

	const form = page.locator("#console_form");

	/** Runs one action, waits for the banner it prints and returns the styles of that banner */
	async function printed(action: () => Promise<void>): Promise<string[]> {
		const messagePromise = page.waitForEvent("console", (message) => message.type() === "log");
		await action();
		const [, styles] = await expectStyledConsoleMessage(await messagePromise);

		return styles;
	}

	// a typed text waits for enter, so the fill alone prints nothing
	await form.getByLabel("Text").fill("Playwright");
	expect(messages).toHaveLength(1);

	await printed(() => form.locator("#console_font").selectOption("Tiny"));
	const styles = await printed(() => form.getByLabel("Background").selectOption("red"));
	expect(styles.some((style) => style.includes("background:"))).toBe(true);

	// enter prints the same picks again
	const again = await printed(() => form.getByLabel("Text").press("Enter"));
	expect(again).toEqual(styles);

	// the default background is the terminal's own, so the next pick paints none
	const plainStyles = await printed(() => form.getByLabel("Background").selectOption("system"));
	expect(plainStyles.some((style) => style.includes("background:"))).toBe(false);

	expect(messages).toHaveLength(5);
	expect(errors.map((error) => error.message)).toEqual([]);
});
