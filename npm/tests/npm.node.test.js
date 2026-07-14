import assert from "node:assert/strict";
import test from "node:test";

import { Align, Cfonts, Font, Valign } from "cfonts";

const INVALID_STRINGS = [0, true, null, undefined, {}, []];

const INVALID_U32_VALUES = [
	-1,
	1.5,
	2 ** 32,
	Number.NaN,
	Number.POSITIVE_INFINITY,
	Number.NEGATIVE_INFINITY,
	"1",
	true,
	null,
	undefined,
	1n,
];

const INVALID_ENUM_VALUES = [-1, 1.5, 99, Number.NaN, "0", "Block", true, null, undefined];

function withTerminal(columns, forceSize, render) {
	const originalColumns = process.stdout.columns;
	const originalRows = process.stdout.rows;
	const originalForceSize = process.env.FORCE_SIZE;
	process.stdout.columns = columns;
	// window-size only accepts a stream size when both dimensions exist
	// exists purely to satisfy window-size's validity check
	process.stdout.rows = 24;

	if (forceSize === undefined) {
		delete process.env.FORCE_SIZE;
	} else {
		process.env.FORCE_SIZE = forceSize;
	}

	try {
		return render();
	} finally {
		process.stdout.columns = originalColumns;
		process.stdout.rows = originalRows;

		if (originalForceSize === undefined) {
			delete process.env.FORCE_SIZE;
		} else {
			process.env.FORCE_SIZE = originalForceSize;
		}
	}
}

function assertTypeErrors(values, invoke, message) {
	for (const value of values) {
		assert.throws(
			() => invoke(value),
			{
				name: "TypeError",
				message,
			},
			`unexpectedly accepted ${String(value)}`,
		);
	}
}

for (const [method, invoke] of [
	["text", (value) => Cfonts.text(value)],
	["newText", (value) => Cfonts.text("A").newText(value)],
]) {
	test(`${method} rejects non-string values`, () => {
		assertTypeErrors(INVALID_STRINGS, invoke, `\`${method}()\` expects a string`);
	});
}

test("text inputs accept strings including empty strings", () => {
	Cfonts.text("").newText("");
});

const u32Setters = [
	["letterSpacing", (banner, value) => banner.letterSpacing(value)],
	["lineHeight", (banner, value) => banner.lineHeight(value)],
	["maxLength", (banner, value) => banner.maxLength(value)],
];

for (const [method, invoke] of u32Setters) {
	test(`${method} rejects values that cannot be represented by u32`, () => {
		assertTypeErrors(
			INVALID_U32_VALUES,
			(value) => invoke(Cfonts.text("A"), value),
			`\`${method}()\` expects an unsigned 32-bit integer`,
		);
	});

	test(`${method} accepts the u32 boundaries`, () => {
		invoke(Cfonts.text("A"), 0);
		invoke(Cfonts.text("A"), 0xffff_ffff);
	});
}

const enumSetters = [
	["font", Font, (banner, value) => banner.font(value)],
	["align", Align, (banner, value) => banner.align(value)],
	["valign", Valign, (banner, value) => banner.valign(value)],
];

for (const [method, enumeration, invoke] of enumSetters) {
	test(`${method} rejects unsupported enum values`, () => {
		assertTypeErrors(
			INVALID_ENUM_VALUES,
			(value) => invoke(Cfonts.text("A"), value),
			`\`${method}()\` expects a supported enum value`,
		);
	});

	test(`${method} accepts every exported enum member`, () => {
		for (const value of Object.values(enumeration)) {
			if (typeof value === "number") {
				invoke(Cfonts.text("A"), value);
			}
		}
	});
}

test("renderCli detects the terminal width", () => {
	const narrow = withTerminal(13, undefined, () => Cfonts.text("AAAA").renderCli().text);
	const wide = withTerminal(120, undefined, () => Cfonts.text("AAAA").renderCli().text);

	assert.notEqual(narrow, wide);
});

test("FORCE_SIZE overrides the terminal width detection", () => {
	const forced = withTerminal(120, "13", () => Cfonts.text("AAAA").renderCli().text);
	const narrow = withTerminal(13, undefined, () => Cfonts.text("AAAA").renderCli().text);

	assert.equal(forced, narrow);
});

test("FORCE_SIZE zero means unlimited", () => {
	const unlimited = withTerminal(13, "0", () => Cfonts.text("AAAA").renderCli().text);
	const wide = withTerminal(13, "120", () => Cfonts.text("AAAA").renderCli().text);

	assert.equal(unlimited, wide);
});

test("FORCE_SIZE garbage falls through to detection", () => {
	for (const garbage of ["", "abc", "-1", "12.5"]) {
		const ignored = withTerminal(13, garbage, () => Cfonts.text("AAAA").renderCli().text);
		const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").renderCli().text);

		assert.equal(ignored, detected, `FORCE_SIZE=${JSON.stringify(garbage)} must fall through`);
	}
});
