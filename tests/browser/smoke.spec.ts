import {
	expect,
	test,
} from "@playwright/test";

test(
	"BrowserHost render initializes WASM without logging",
	async ({ page }) => {
		const messages: string[] = [];
		const errors: string[] = [];

		page.on("console", (message) => {
			if (message.text().includes("█")) {
				messages.push(message.text());
			}
		});
		page.on("pageerror", (error) => {
			errors.push(error.message);
		});

		await page.goto("/");

		await expect(
			page.locator("#banner div"),
		).toContainText("█");

		expect(messages).toEqual([]);
		expect(errors).toEqual([]);
	},
);

test(
	"BrowserHost say writes the browser-console artifact",
	async ({ page }) => {
		const messages: string[] = [];
		const errors: string[] = [];

		page.on("console", (message) => {
			if (message.text().includes("█")) {
				messages.push(message.text());
			}
		});
		page.on("pageerror", (error) => {
			errors.push(error.message);
		});

		await page.goto(
			"http://127.0.0.1:4174",
		);

		await expect
			.poll(() => messages.length)
			.toBe(2);

		expect(errors).toEqual([]);
	},
);
