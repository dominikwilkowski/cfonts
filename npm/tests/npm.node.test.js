import assert from "node:assert/strict";
import test from "node:test";

import { Align, Cfonts, Env, Font, Valign } from "cfonts";

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
	["env", Env, (banner, value) => banner.env(value)],
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
