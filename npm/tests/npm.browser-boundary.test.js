import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const ROOT = fileURLToPath(new URL("../..", import.meta.url));

const NODE_ONLY_MARKERS = [
	["NodeHost", /\bNodeHost\b/],
	["node:fs", /node:fs/],
	["window-size", /window-size/],
	["process.env", /process\.env/],
	["process.stdout", /process\.stdout/],
];

function readJavaScript(directory) {
	return readdirSync(directory, {
		withFileTypes: true,
	})
		.flatMap((entry) => {
			const path = join(directory, entry.name);

			if (entry.isDirectory()) {
				return readJavaScript(path);
			}

			return entry.name.endsWith(".js") ? [readFileSync(path, "utf8")] : [];
		})
		.join("\n");
}

function assertBrowserSafe(source, sourceName) {
	for (const [name, marker] of NODE_ONLY_MARKERS) {
		assert.doesNotMatch(source, marker, `${sourceName} contains the Node-only marker ${name}`);
	}
}

test("the browser entry contains no Node-only code", () => {
	const source = readFileSync(join(ROOT, "dist", "browser.js"), "utf8");

	assertBrowserSafe(source, "dist/browser.js");
});

test("the browser bundles contain no Node-only code", () => {
	for (const directory of ["target/browser-example", "target/browser-console-example"]) {
		const source = readJavaScript(join(ROOT, directory));

		assertBrowserSafe(source, directory);
	}
});
