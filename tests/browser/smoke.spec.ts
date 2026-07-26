import { type ConsoleMessage, expect, type Page, test } from "@playwright/test";

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

async function expectStyledConsoleMessage(message: ConsoleMessage): Promise<void> {
	const arguments_ = message.args();

	expect(arguments_.length).toBeGreaterThan(1);

	const [format, firstStyle] = await Promise.all([arguments_[0]?.jsonValue(), arguments_[1]?.jsonValue()]);

	if (typeof format !== "string") {
		throw new TypeError(`Expected a console format string, received ${typeof format}`);
	}

	if (typeof firstStyle !== "string") {
		throw new TypeError(`Expected a console style string, received ${typeof firstStyle}`);
	}

	expect(format).toContain("%c");
	expect(format.match(/%c/g) ?? []).toHaveLength(arguments_.length - 1);
	expect(firstStyle).toMatch(/^color:/);
}

test("BrowserHost render produces styled HTML", async ({ page }) => {
	const errors = capturePageErrors(page);

	await page.goto("/");

	const banner = page.locator("#banner > div");

	await expect(banner).toBeVisible();
	await expect(banner).toHaveCSS("white-space", "pre");
	await expect(page.locator("#banner span[style*='color:']").first()).toBeAttached();
	await expect(page.locator("#banner")).not.toContainText("%c");

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

test("BrowserHost say writes one styled console.log call", async ({ page }) => {
	const messages = captureMessages(page, "log");
	const errors = capturePageErrors(page);

	await page.goto("/");
	await page.getByLabel("Text").fill("Playwright");

	const messagePromise = page.waitForEvent("console", (message) => message.type() === "log");

	await page.getByRole("button", { name: "Show in devtool console" }).click();

	const message = await messagePromise;

	expect(messages).toHaveLength(1);
	await expectStyledConsoleMessage(message);
	expect(errors.map((error) => error.message)).toEqual([]);
});
