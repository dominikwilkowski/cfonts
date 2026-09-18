import { type ConsoleMessage, expect, type Locator, type Page, test } from "@playwright/test";
import { Font } from "cfonts";

/** The names the page fills its font pickers with, the command line spelling of every font of the enum */
const FONT_NAMES = Object.keys(Font)
	.filter((key) => Number.isNaN(Number(key)))
	.map((name) => (name === "Font3D" ? "3d" : name.toLowerCase()));

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

test("the page runs the command line in the terminal and fills the font pickers from the enum", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);

	await page.goto("/");

	const banner = page.locator("#canvas > div");

	await expect(banner).toBeVisible();
	await expect(banner).toHaveCSS("white-space", "pre");
	await expect(page.locator("#canvas span[style*='color:']").first()).toBeAttached();
	await expect(page.locator("#canvas")).not.toContainText("%c");
	await expect(page.locator("#command")).toContainText('cfonts "How are you?"');

	for (const picker of ["select[name='font']", "select[name='next-font']"]) {
		await expect(page.locator(`${picker} option`)).toHaveText(FONT_NAMES);
		expect(FONT_NAMES).toContain(await page.locator(picker).inputValue()); // the pickers open on a real font
	}

	expect(messages).toEqual([]); // the page prints nothing until the console is picked
	expect(errors.map((error) => error.message)).toEqual([]);
});

test("every pick re-renders the terminal and spells the command line", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const canvas = page.locator("#canvas");
	const form = page.locator("#configurator");
	const command = page.locator("#command");
	expect(await bands(canvas)).toEqual([]); // the first render carries no background

	/** Fills one choice per field, every pick re-renders, and waits for the banded render */
	async function pick(colors: string, background: string, font: string): Promise<string[]> {
		await form.getByLabel("text", { exact: true }).fill("Playwright");
		await form.getByLabel("colors").fill(colors);
		await form.getByLabel("background").fill(background);
		await form.locator("select[name='font']").selectOption(font);
		await expect(canvas.locator("div[style*='background:']").first()).toBeAttached();

		return bands(canvas);
	}

	// one static background paints every row the same
	const tiny = await pick("cyan", "red", "tiny");
	expect(tiny.length).toBeGreaterThan(1);
	expect(new Set(tiny).size).toBe(1);
	await expect(command).toHaveText('cfonts "Playwright" --font tiny --word-wrap --colors cyan --background red');

	// a taller font adds rows, so the picked font reaches the render
	const huge = await pick("cyan", "red", "huge");
	expect(huge.length).toBeGreaterThan(tiny.length);

	// the agender preset paints a pale green no other block on the page uses, so the picked colors reach it too
	await expect(canvas.locator("span[style*='#b8f483']")).toHaveCount(0);
	await pick("agender", "red", "tiny");
	await expect(canvas.locator("span[style*='#b8f483']").first()).toBeAttached();

	// a next block takes its own font, the flag follows the block on the command line
	await form.getByLabel("next", { exact: true }).fill(" World");
	await form.locator("select[name='next-font']").selectOption("huge");
	await expect(command).toContainText('--next " World" --font huge');
	expect((await bands(canvas)).length).toBeGreaterThan(tiny.length); // the taller block sets the height

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("a value the command line refuses shows its error in the terminal", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const canvas = page.locator("#canvas");
	const colors = page.locator("#configurator").getByLabel("colors");

	await colors.fill("foo");
	await expect(canvas).toContainText("ERROR");
	await expect(canvas).toContainText("Unsupported color `foo`");
	await expect(colors).toHaveJSProperty("validity.valid", false);
	await expect(page.locator("#command")).not.toContainText("--colors"); // the command stops at the refused option

	await colors.fill("cyan");
	await expect(canvas).not.toContainText("ERROR");
	await expect(colors).toHaveJSProperty("validity.valid", true);

	expect(errors.map((error) => error.message)).toEqual([]);
});

test("the browser console prints one styled banner per pick and one more on enter", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);

	await page.goto("/");

	const form = page.locator("#configurator");
	const terminal = page.locator(".terminal");
	const note = page.locator(".note");

	/** Runs one action, waits for the banner it prints and returns the styles of that banner */
	async function printed(action: () => Promise<void>): Promise<string[]> {
		const messagePromise = page.waitForEvent("console", (message) => message.type() === "log");
		await action();
		const [, styles] = await expectStyledConsoleMessage(await messagePromise);

		return styles;
	}

	// picking the console prints right away, the terminal leaves the page and the note takes its place
	await expect(terminal).toBeVisible();
	await expect(note).toBeHidden();
	await printed(() => form.getByLabel("browser console").check());
	await expect(terminal).toBeHidden();
	await expect(note).toBeVisible();

	// a typed text waits for enter, so the fill alone prints nothing
	await form.getByLabel("text", { exact: true }).fill("Playwright");
	expect(messages).toHaveLength(1);

	await printed(() => form.locator("select[name='font']").selectOption("tiny"));
	const styles = await printed(() => form.getByLabel("background").press("Enter"));
	expect(styles.some((style) => style.includes("background:"))).toBe(false);

	// a background typed and entered paints every row
	await form.getByLabel("background").fill("red");
	const painted = await printed(() => form.getByLabel("background").press("Enter"));
	expect(painted.some((style) => style.includes("background:"))).toBe(true);

	// back in the browser the terminal returns and nothing prints any more
	await form.getByLabel("browser", { exact: true }).check();
	await expect(terminal).toBeVisible();
	await expect(note).toBeHidden();
	await form.locator("select[name='font']").selectOption("huge");
	expect(messages).toHaveLength(4);

	expect(errors.map((error) => error.message)).toEqual([]);
});
