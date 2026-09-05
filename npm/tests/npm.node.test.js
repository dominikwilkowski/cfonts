import assert from "node:assert/strict";
import test from "node:test";
import * as packageExports from "cfonts";
import { detectColorSupport } from "../../pkg/cfonts_wasm.js";

const {
	Align,
	BrowserConsoleEnv,
	BrowserEnv,
	Cfonts,
	CliEnv,
	Color,
	ColorLevel,
	Font,
	GradientPreset,
	hexToRgb,
	NodeHost,
	Valign,
} = packageExports;

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

// helpers

function withEnv(name, value, operation) {
	const original = process.env[name];

	if (value === undefined) {
		delete process.env[name];
	} else {
		process.env[name] = value;
	}

	try {
		return operation();
	} finally {
		if (original === undefined) {
			delete process.env[name];
		} else {
			process.env[name] = original;
		}
	}
}

function withColorEnv(forceColor, noColor, operation) {
	return withEnv("FORCE_COLOR", forceColor, () => withEnv("NO_COLOR", noColor, operation));
}

// every variable the detection cascade reads, cleared so rows are
// deterministic in any shell or CI runner
const DETECTION_VARS = ["TERM", "COLORTERM", "TMUX", "CI", "TF_BUILD", "TEAMCITY_VERSION", "TERM_PROGRAM"];

function withDetectionEnv(vars, operation) {
	const apply = (index) => {
		if (index >= DETECTION_VARS.length) {
			return operation();
		}
		return withEnv(DETECTION_VARS[index], vars[DETECTION_VARS[index]], () => apply(index + 1));
	};
	return apply(0);
}

function withTerminal(columns, forceSize, operation) {
	return withEnv("FORCE_SIZE", forceSize, () => {
		const restoreStdout = overrideProperty(process.stdout, "columns", columns);
		const restoreStderr = overrideProperty(process.stderr, "columns", undefined);

		try {
			return operation();
		} finally {
			restoreStderr();
			restoreStdout();
		}
	});
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

function overrideProperty(target, property, value) {
	const descriptor = Object.getOwnPropertyDescriptor(target, property);

	Object.defineProperty(target, property, {
		configurable: true,
		writable: true,
		value,
	});

	return () => {
		if (descriptor === undefined) {
			delete target[property];
		} else {
			Object.defineProperty(target, property, descriptor);
		}
	};
}

function captureStdout(operation) {
	const writes = [];

	const restore = overrideProperty(process.stdout, "write", (chunk) => {
		writes.push(String(chunk));
		return true;
	});

	try {
		operation();
		return writes;
	} finally {
		restore();
	}
}

function captureConsoleLogs(operation) {
	const calls = [];

	const restore = overrideProperty(console, "log", (...arguments_) => {
		calls.push(arguments_);
	});

	try {
		operation();
		return calls;
	} finally {
		restore();
	}
}

function wrappingBanner() {
	return Cfonts.text("AA").font(Font.Tiny).lineHeight(0).spaceless();
}

/**
 * A one block hex colored banner for the color precedence tests
 */
function colorBanner() {
	return Cfonts.text("AB").font(Font.Tiny).colors(["#ff8800"]);
}

/**
 * The banner rendered at one explicit color level, as the reference output
 */
function reference(colorLevel) {
	return colorBanner().renderWith(CliEnv, colorLevel === undefined ? undefined : { colorLevel }).text;
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

test("the Node entry exports NodeHost but not BrowserHost", () => {
	assert.equal(typeof packageExports.NodeHost, "function");
	assert.equal("BrowserHost" in packageExports, false);
});

test("renderWith selects each environment", () => {
	const banner = wrappingBanner();

	assert.equal(banner.renderWith(CliEnv).text, "▄▀█ ▄▀█\n█▀█ █▀█");
	assert.equal(banner.renderWith(BrowserConsoleEnv).text, "▄▀█ ▄▀█\n█▀█ █▀█");
	assert.equal(
		banner.renderWith(BrowserEnv).text,
		'<div style="font-family:monospace;white-space:pre;text-align:left;max-width:100%;overflow:scroll;background:">▄▀█ ▄▀█<br>█▀█ █▀█</div>',
	);
});

test("renderWith applies width to every environment", () => {
	for (const [name, environment] of [
		["CLI", CliEnv],
		["browser", BrowserEnv],
		["browser console", BrowserConsoleEnv],
	]) {
		const banner = wrappingBanner();

		const unlimited = banner.renderWith(environment).text;

		const zero = banner.renderWith(environment, { canvasWidth: 0 }).text;

		const narrow = banner.renderWith(environment, { canvasWidth: 3 }).text;

		assert.equal(zero, unlimited, `${name} zero must mean unlimited`);
		assert.notEqual(narrow, unlimited, `${name} must receive the fixed width`);
	}
});

test("renderWith BrowserConsoleEnv returns without logging", () => {
	let artifact;

	const calls = captureConsoleLogs(() => {
		artifact = wrappingBanner().renderWith(BrowserConsoleEnv);
	});

	assert.deepEqual(calls, []);
	assert.equal(artifact.text, "▄▀█ ▄▀█\n█▀█ █▀█");
});

test("renderWith rejects unsupported environments", () => {
	for (const environment of [undefined, null, 0, true, "", {}, []]) {
		assert.throws(() => wrappingBanner().renderWith(environment), {
			name: "TypeError",
			message: "`renderWith()` expects a cfonts environment",
		});
	}

	// even the private symbol with a forged kind stays outside the closed set
	const forged = Object.freeze({ [Object.getOwnPropertySymbols(CliEnv)[0]]: 99 });
	assert.throws(() => wrappingBanner().renderWith(forged), {
		name: "TypeError",
		message: "`renderWith()` expects a cfonts environment",
	});
});

test("renderWith rejects invalid contexts", () => {
	for (const context of [null, 0, true, "", [], () => {}]) {
		assert.throws(() => wrappingBanner().renderWith(CliEnv, context), {
			name: "TypeError",
			message: "`renderWith()` expects a render context object",
		});
	}
});

test("renderWith validates canvasWidth at runtime", () => {
	for (const canvasWidth of INVALID_U32_VALUES.filter((value) => value !== undefined)) {
		assert.throws(() => wrappingBanner().renderWith(CliEnv, { canvasWidth }), {
			name: "TypeError",
			message: "`renderWith()` expects an unsigned 32-bit integer",
		});
	}

	wrappingBanner().renderWith(CliEnv, { canvasWidth: 0 });
	wrappingBanner().renderWith(CliEnv, { canvasWidth: 0xffff_ffff });
});

test("custom hosts receive the composition exactly once", () => {
	const composition = Cfonts.text("A");
	let renderCalls = 0;
	let sayCalls = 0;

	const host = {
		render(received) {
			assert.equal(received, composition);
			renderCalls += 1;

			return {
				text: "custom render",
			};
		},
		say(received) {
			assert.equal(received, composition);
			sayCalls += 1;
		},
	};

	assert.deepEqual(composition.render(host), { text: "custom render" });
	composition.say(host);

	assert.equal(renderCalls, 1);
	assert.equal(sayCalls, 1);
});

test("render rejects values without a render method", () => {
	const composition = Cfonts.text("A");

	for (const host of [undefined, null, 0, true, "", {}, [], { say() {} }]) {
		assert.throws(() => composition.render(host), {
			name: "TypeError",
			message: "`render()` expects a cfonts host",
		});
	}
});

test("say rejects values without a say method", () => {
	const composition = Cfonts.text("A");

	for (const host of [undefined, null, 0, true, "", {}, [], { render() {} }]) {
		assert.throws(() => composition.say(host), {
			name: "TypeError",
			message: "`say()` expects a cfonts host",
		});
	}
});

test("NodeHost validates override objects", () => {
	for (const overrides of [undefined, null, 0, true, "", [], () => {}]) {
		assert.throws(() => NodeHost.fromOverrides(overrides), {
			name: "TypeError",
			message: "`fromOverrides()` expects an overrides object",
		});
	}
});

test("NodeHost validates override widths", () => {
	for (const canvasWidth of INVALID_U32_VALUES.filter((value) => value !== undefined)) {
		assert.throws(
			() =>
				NodeHost.fromOverrides({
					canvasWidth,
				}),
			{
				name: "TypeError",
				message: "`fromOverrides()` expects an unsigned 32-bit integer",
			},
		);
	}

	NodeHost.fromOverrides({
		canvasWidth: 0,
	});
	NodeHost.fromOverrides({
		canvasWidth: 0xffff_ffff,
	});
});

test("NodeHost detects width on each render", () => {
	const host = new NodeHost();
	const banner = Cfonts.text("AAAA");

	const narrow = withTerminal(13, undefined, () => banner.render(host).text);
	const wide = withTerminal(120, undefined, () => banner.render(host).text);

	assert.notEqual(narrow, wide);
});

test("stdout answers before stderr", () => {
	const restoreStdout = overrideProperty(process.stdout, "columns", 120);
	const restoreStderr = overrideProperty(process.stderr, "columns", 13);

	try {
		const preferred = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);
		const viaStdout = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

		assert.equal(preferred, viaStdout);
	} finally {
		restoreStderr();
		restoreStdout();
	}
});

test("a zero-width stream measures nothing", () => {
	const restoreStdout = overrideProperty(process.stdout, "columns", 0);
	const restoreStderr = overrideProperty(process.stderr, "columns", 13);

	try {
		const viaStderr = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);
		const reference = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

		assert.equal(viaStderr, reference);
	} finally {
		restoreStderr();
		restoreStdout();
	}
});

test("the measurement falls back to stderr when stdout is redirected", () => {
	const restoreStdout = overrideProperty(process.stdout, "columns", undefined);
	const restoreStderr = overrideProperty(process.stderr, "columns", 13);

	try {
		const viaStderr = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);
		const viaStdout = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

		assert.equal(viaStderr, viaStdout);
	} finally {
		restoreStderr();
		restoreStdout();
	}
});

test("a fully redirected process falls back to eighty columns", () => {
	const restoreStdout = overrideProperty(process.stdout, "columns", undefined);
	const restoreStderr = overrideProperty(process.stderr, "columns", undefined);

	try {
		const fallback = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAAAAAAAA").render(new NodeHost()).text);
		const eighty = withEnv(
			"FORCE_SIZE",
			undefined,
			() => Cfonts.text("AAAAAAAAAA").render(NodeHost.fromOverrides({ canvasWidth: 80 })).text,
		);
		const unlimited = withEnv(
			"FORCE_SIZE",
			undefined,
			() => Cfonts.text("AAAAAAAAAA").render(NodeHost.fromOverrides({ canvasWidth: 0 })).text,
		);

		assert.equal(fallback, eighty);
		assert.notEqual(fallback, unlimited);
	} finally {
		restoreStderr();
		restoreStdout();
	}
});

test("FORCE_SIZE overrides terminal detection", () => {
	const forced = withTerminal(120, "13", () => Cfonts.text("AAAA").render(new NodeHost()).text);

	const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

	assert.equal(forced, detected);
});

test("FORCE_SIZE overrides an API width", () => {
	const forcedHost = NodeHost.fromOverrides({
		canvasWidth: 120,
	});

	const expectedHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const forced = withTerminal(120, "13", () => Cfonts.text("AAAA").render(forcedHost).text);

	const expected = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(expectedHost).text);

	assert.equal(forced, expected);
});

test("FORCE_SIZE zero overrides an API width with unlimited output", () => {
	const forcedHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const unlimitedHost = NodeHost.fromOverrides({
		canvasWidth: 0,
	});

	const forced = withTerminal(13, "0", () => Cfonts.text("AAAA").render(forcedHost).text);

	const expected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(unlimitedHost).text);

	assert.equal(forced, expected);
});

test("an API width overrides terminal detection", () => {
	const explicitHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const explicit = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(explicitHost).text);

	const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

	assert.equal(explicit, detected);
});

test("an API width of zero means unlimited", () => {
	const unlimitedHost = NodeHost.fromOverrides({
		canvasWidth: 0,
	});

	const unlimited = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(unlimitedHost).text);

	const wide = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

	assert.equal(unlimited, wide);
});

test("invalid FORCE_SIZE falls through to the API override", () => {
	const host = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	for (const garbage of ["", "abc", "-1", "12.5", "4294967296"]) {
		const ignored = withTerminal(120, garbage, () => Cfonts.text("AAAA").render(host).text);

		const expected = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(host).text);

		assert.equal(ignored, expected, `FORCE_SIZE=${JSON.stringify(garbage)} must fall through`);
	}
});

test("NodeHost render does not write to stdout", () => {
	withTerminal(80, undefined, () => {
		const writes = captureStdout(() => {
			Cfonts.text("A").render(new NodeHost());
		});

		assert.deepEqual(writes, []);
	});
});

test("NodeHost say writes exactly once", () => {
	withTerminal(80, undefined, () => {
		const banner = Cfonts.text("A");
		const host = new NodeHost();
		const expected = banner.render(host).text;

		const writes = captureStdout(() => {
			banner.say(host);
		});

		assert.deepEqual(writes, [`${expected}\n`]);
	});
});

test("FORCE_SIZE zero means unlimited", () => {
	const unlimited = withTerminal(13, "0", () => Cfonts.text("AAAA").render(new NodeHost()).text);
	const wide = withTerminal(13, "120", () => Cfonts.text("AAAA").render(new NodeHost()).text);

	assert.equal(unlimited, wide);
});

test("FORCE_SIZE garbage falls through to detection", () => {
	for (const garbage of ["", "abc", "-1", "12.5"]) {
		const ignored = withTerminal(13, garbage, () => Cfonts.text("AAAA").render(new NodeHost()).text);
		const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);

		assert.equal(ignored, detected, `FORCE_SIZE=${JSON.stringify(garbage)} must fall through`);
	}
});

test("color overrides are validated", () => {
	assert.throws(() => NodeHost.fromOverrides({ color: 99 }), TypeError);
	assert.throws(() => NodeHost.fromOverrides({ seed: -1 }), TypeError);
	NodeHost.fromOverrides({ color: false });
	NodeHost.fromOverrides({ color: ColorLevel.Basic, seed: 42 });
});

test("renderWith validates color context fields", () => {
	assert.throws(() => Cfonts.text("A").renderWith(CliEnv, { colorLevel: 99 }), TypeError);
	assert.throws(() => Cfonts.text("A").renderWith(CliEnv, { seed: 1.5 }), TypeError);
});

test("a color level without color options paints nothing", () => {
	const plain = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost()).text);
	const leveled = withTerminal(
		13,
		undefined,
		() => Cfonts.text("AAAA").render(NodeHost.fromOverrides({ color: ColorLevel.TrueColor, seed: 42 })).text,
	);

	assert.equal(plain, leveled);
});

test("colors accepts enums hex values and channel objects", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;
	const colored = Cfonts.text("A")
		.colors([Color.Red, "#ff8800", "f80", { red: 1, green: 2, blue: 3 }, Color.Candy, "grey"])
		.renderWith(CliEnv).text;
	const empty = Cfonts.text("A").colors([]).renderWith(CliEnv).text;

	assert.equal(colored, plain); // renderWith without a color level paints nothing
	assert.equal(empty, plain); // an empty list is still a configured color
});

test("colors validates its input", () => {
	assert.throws(() => Cfonts.text("A").colors("red"), TypeError); // not an array
	assert.throws(() => Cfonts.text("A").colors([99]), TypeError); // not a Color
	assert.throws(() => Cfonts.text("A").colors([{ red: 256, green: 0, blue: 0 }]), TypeError); // not a channel value
	assert.throws(() => Cfonts.text("A").colors([true]), TypeError);
	assert.throws(() => Cfonts.text("A").colors(["reed"]), Error); // unknown name, rejected in Rust
	assert.throws(() => Cfonts.text("A").colors(["#ff88"]), Error); // invalid hex, rejected in Rust
});

test("gradient shapes are validated", () => {
	assert.throws(() => Cfonts.text("A").gradient({}), TypeError); // no shape
	assert.throws(() => Cfonts.text("A").gradient({ start: "red", transition: ["red", "blue"] }), TypeError); // two shapes
	assert.throws(() => Cfonts.text("A").gradient({ start: "red" }), TypeError); // missing end
	assert.throws(() => Cfonts.text("A").gradient({ start: "red", end: "blue", independentGradient: 1 }), TypeError);
	assert.throws(() => Cfonts.text("A").gradient({ transition: "red" }), TypeError); // not an array
	assert.throws(() => Cfonts.text("A").gradient({ preset: 99 }), TypeError); // not a preset
	assert.throws(() => Cfonts.text("A").gradient(99), TypeError);
	assert.throws(() => Cfonts.text("A").gradient({ transition: [] }), /at least two stops, this one holds 0/); // empty, rejected in Rust
	assert.throws(() => Cfonts.text("A").gradient({ transition: ["red"] }), /at least two stops, this one holds 1/); // one stop, rejected in Rust
	assert.throws(() => Cfonts.text("A").gradient({ start: "system", end: "blue" }), /Unsupported color/); // system is not a gradient stop
});

test("gradient stops accept the base Color values", () => {
	const context = { colorLevel: ColorLevel.TrueColor };
	const named = Cfonts.text("A").gradient({ start: "red", end: "blue" }).renderWith(CliEnv, context).text;
	const typed = Cfonts.text("A").gradient({ start: Color.Red, end: Color.Blue }).renderWith(CliEnv, context).text;
	assert.equal(typed, named);

	const transition = Cfonts.text("A")
		.gradient({ transition: [Color.Red, "#8899dd", { red: 0, green: 0, blue: 255 }] })
		.renderWith(CliEnv, context).text;
	const spelled = Cfonts.text("A")
		.gradient({ transition: ["red", "#8899dd", "#0000ff"] })
		.renderWith(CliEnv, context).text;
	assert.equal(transition, spelled);

	const global = Cfonts.text("A").globalGradient({ start: Color.Yellow, end: Color.Gray }).renderWith(CliEnv, context);
	assert.ok(global.text.includes("\u001b[38;2;"));
});

test("gradient stops outside the base palette are rejected in Rust", () => {
	// valid enum members travel as their names; which colors may blend is Rust's decision
	for (const color of [Color.System, Color.Candy, Color.RedBright]) {
		assert.throws(() => Cfonts.text("A").gradient({ start: color, end: Color.Blue }), /Unsupported color/);
	}

	assert.throws(() => Cfonts.text("A").gradient({ transition: [Color.Red, Color.WhiteBright] }), /Unsupported color/);

	// an unknown number is still a shape error, caught at the boundary
	assert.throws(() => Cfonts.text("A").gradient({ start: 99, end: Color.Blue }), {
		name: "TypeError",
		message: /supported enum value/,
	});
});

test("gradient shape errors teach the shapes", () => {
	assert.throws(() => Cfonts.text("A").gradient({}), {
		name: "TypeError",
		message: /start: Color\.Red.*independentGradient/,
	});
	assert.throws(() => Cfonts.text("A").gradient({ start: Color.Red }), {
		name: "TypeError",
		message: /both start and end/,
	});
	assert.throws(() => Cfonts.text("A").gradient({ transition: "red" }), {
		name: "TypeError",
		message: /two or more/,
	});
});

test("hexToRgb converts hex values into channels", () => {
	assert.deepEqual(hexToRgb("#ff8800"), { red: 255, green: 136, blue: 0 });
	assert.deepEqual(hexToRgb("f80"), { red: 255, green: 136, blue: 0 });
	assert.ok(Object.isFrozen(hexToRgb("#ff8800")));

	assert.throws(() => hexToRgb("#ff88"), Error); // four digits are invalid
	assert.throws(() => hexToRgb("teal"), Error); // names are not hex values
	assert.throws(() => hexToRgb(42), TypeError);

	const context = { colorLevel: ColorLevel.TrueColor };
	const channeled = Cfonts.text("A")
		.gradient({ start: hexToRgb("#ff8800"), end: Color.Blue })
		.renderWith(CliEnv, context).text;
	const spelled = Cfonts.text("A").gradient({ start: "#ff8800", end: "blue" }).renderWith(CliEnv, context).text;
	assert.equal(channeled, spelled);
});

test("gradients accept every shape and paint nothing without a color level", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;

	const preset = Cfonts.text("A").gradient(GradientPreset.Pride).renderWith(CliEnv).text;
	const twoStop = Cfonts.text("A")
		.gradient({ start: "red", end: "#0000ff", independentGradient: true })
		.renderWith(CliEnv).text;
	const transition = Cfonts.text("A")
		.gradient({ transition: ["red", { red: 0, green: 0, blue: 255 }, "gray"] })
		.renderWith(CliEnv).text;
	const global = Cfonts.text("A")
		.globalGradient({ preset: GradientPreset.Transgender, independentGradient: true })
		.renderWith(CliEnv).text;

	for (const rendered of [preset, twoStop, transition, global]) {
		assert.equal(rendered, plain);
	}
});

test("globalColors accepts colors and paints nothing without a color level", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;
	const global = Cfonts.text("A").globalColors([Color.Red, "#ff8800"]).renderWith(CliEnv).text;

	assert.equal(global, plain);
	assert.throws(() => Cfonts.text("A").globalColors("red"), TypeError); // not an array
	assert.throws(() => Cfonts.text("A").globalColors(["reed"]), Error); // unknown name, rejected in Rust
});

test("globalColors and globalGradient share the one global slot", () => {
	const colored = Cfonts.text("A").globalColors([Color.Red]);
	assert.throws(() => colored.globalGradient(GradientPreset.Pride), /global color has already been set/);

	const ramped = Cfonts.text("A").globalGradient(GradientPreset.Pride);
	assert.throws(() => ramped.globalColors([Color.Red]), /global color has already been set/);
});

test("the global gradient can only be set once", () => {
	const banner = Cfonts.text("A").globalGradient(GradientPreset.Pride);

	assert.throws(() => banner.globalGradient(GradientPreset.Agender), Error);
	assert.throws(() => banner.globalGradient({ start: "red", end: "blue" }), Error);
});

test("a failed global gradient does not claim the slot", () => {
	const banner = Cfonts.text("A");

	assert.throws(() => banner.globalGradient({ start: "reed", end: "blue" }), Error);
	banner.globalGradient({ start: "red", end: "blue" }); // the slot is still available
});

test("renderWith paints with an explicit color level", () => {
	const cli = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(CliEnv, {
		colorLevel: ColorLevel.TrueColor,
	}).text;
	assert.ok(cli.includes("\u001b[31m"));

	const browser = Cfonts.text("A").font(Font.Tiny).colors(["#ff8800"]).renderWith(BrowserEnv, {
		colorLevel: ColorLevel.TrueColor,
	}).text;
	assert.ok(browser.includes('<span style="color:#ff8800">'));
});

test("colors paint through the node host", () => {
	const rendered = withEnv("FORCE_COLOR", "3", () =>
		withTerminal(80, undefined, () => Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).render(new NodeHost()).text),
	);

	assert.ok(rendered.includes("\u001b[31m"));
});

test("the host delegates color precedence to the shared chain", () => {
	// a tty whose cascade would answer Ansi256: any resolved row that renders
	// something else proves detection never ran
	const restoreTty = overrideProperty(process.stdout, "isTTY", true);

	try {
		withDetectionEnv({ TERM: "xterm-256color" }, () => {
			// the raw value crosses the boundary untouched: the shared chain reads it, not this host,
			// and it beats both NO_COLOR and a disabled override
			for (const [forced, expected] of [
				["3", ColorLevel.TrueColor],
				["2", ColorLevel.Ansi256],
				["junk", ColorLevel.Basic],
				["", ColorLevel.Basic],
				["false", undefined],
			]) {
				const rendered = withColorEnv(forced, "1", () =>
					NodeHost.fromOverrides({ canvasWidth: 0, color: false }).render(colorBanner()),
				);
				assert.equal(rendered.text, reference(expected), `FORCE_COLOR=${JSON.stringify(forced)}`);
			}

			// NO_COLOR and the API override resolve without detection
			const noColor = withColorEnv(undefined, "1", () =>
				NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner()),
			);
			assert.equal(noColor.text, reference(undefined));

			const overridden = withColorEnv(undefined, undefined, () =>
				NodeHost.fromOverrides({ canvasWidth: 0, color: ColorLevel.Basic }).render(colorBanner()),
			);
			assert.equal(overridden.text, reference(ColorLevel.Basic));
		});
	} finally {
		restoreTty();
	}
});

test("NO_COLOR counts only when present and non-empty", { skip: process.platform === "win32" }, () => {
	const restoreTty = overrideProperty(process.stdout, "isTTY", true);

	try {
		withDetectionEnv({ TERM: "xterm-256color" }, () => {
			// an empty value is not set: the chain falls through to detection,
			// which answers the terminal and never the leftover variable
			const empty = withColorEnv(undefined, "", () => NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner()));
			assert.equal(empty.text, reference(ColorLevel.Ansi256));

			// any non-empty value counts, zero included
			const zero = withColorEnv(undefined, "0", () => NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner()));
			assert.equal(zero.text, reference(undefined));
		});
	} finally {
		restoreTty();
	}
});

test("detection runs the shared cascade", { skip: process.platform === "win32" }, () => {
	for (const [vars, expected] of [
		[{ TERM: "ansi" }, ColorLevel.Basic],
		[{ TERM: "xterm-256color" }, ColorLevel.Ansi256],
		[{ COLORTERM: "truecolor" }, ColorLevel.TrueColor],
		// an undetectable terminal still gets full color
		[{ TERM: "fail" }, ColorLevel.TrueColor],
	]) {
		const restoreTty = overrideProperty(process.stdout, "isTTY", true);

		try {
			withDetectionEnv(vars, () => {
				const rendered = withColorEnv(undefined, undefined, () =>
					NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner()),
				);
				assert.equal(rendered.text, reference(expected), JSON.stringify(vars));
			});
		} finally {
			restoreTty();
		}
	}
});

test("the chain crosses the boundary with the environment", () => {
	// FORCE_COLOR wins over everything the cascade would say
	assert.equal(
		detectColorSupport(true, ["TERM", "FORCE_COLOR"], ["xterm-256color", "3"], undefined, false, undefined),
		ColorLevel.TrueColor,
	);

	// NO_COLOR silences an otherwise colorful terminal
	assert.equal(
		detectColorSupport(true, ["TERM", "NO_COLOR"], ["xterm-256color", "1"], undefined, false, undefined),
		undefined,
	);

	// an empty NO_COLOR is not set: the cascade answers
	assert.equal(
		detectColorSupport(true, ["TERM", "NO_COLOR"], ["xterm-256color", ""], undefined, false, undefined),
		ColorLevel.Ansi256,
	);
});

test("the boundary answers the windows console by build", () => {
	assert.equal(detectColorSupport(true, [], [], 22631, false, undefined), ColorLevel.TrueColor);
	assert.equal(detectColorSupport(true, [], [], 10586, false, undefined), ColorLevel.Ansi256);
	assert.equal(detectColorSupport(true, [], [], 9600, false, undefined), ColorLevel.Basic);

	// a detached stream still paints the render fallback, a disabled override never paints
	assert.equal(detectColorSupport(false, [], [], 22631, false, undefined), ColorLevel.TrueColor);
	assert.equal(detectColorSupport(true, [], [], 22631, true, undefined), undefined);
});

test("piped output has no terminal to ask and falls back to full color", () => {
	const restoreTty = overrideProperty(process.stdout, "isTTY", false);

	try {
		const rendered = withColorEnv(undefined, undefined, () =>
			NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner()),
		);
		assert.equal(rendered.text, colorBanner().renderWith(CliEnv, { colorLevel: ColorLevel.TrueColor }).text);
	} finally {
		restoreTty();
	}
});

test("console styles pair with their markers through renderWith", () => {
	const unstyled = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(BrowserConsoleEnv);
	assert.ok(!unstyled.text.includes("%c"));
	assert.deepEqual(unstyled.styles, []);

	const styled = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(BrowserConsoleEnv, {
		colorLevel: ColorLevel.TrueColor,
	});
	assert.equal(styled.text.match(/%c/g).length, styled.styles.length);
	assert.ok(styled.styles.includes("color:#ea3223"));
	assert.ok(styled.styles.includes(""));
});

test("candy seeds are deterministic through renderWith", () => {
	const seeded = { colorLevel: ColorLevel.TrueColor, seed: 42 };

	const one = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, seeded).text;
	const two = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, seeded).text;
	const other = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, {
		colorLevel: ColorLevel.TrueColor,
		seed: 43,
	}).text;

	assert.equal(one, two);
	assert.notEqual(one, other);
	assert.ok(one.includes("\u001b["));

	const consoleArtifact = Cfonts.text("A").font(Font.Tiny).colors([Color.Candy]).renderWith(BrowserConsoleEnv, seeded);
	assert.ok(consoleArtifact.styles.length > 0);
});

test("gradients paint through renderWith with a color level", () => {
	const ramped = Cfonts.text("A").font(Font.Tiny).gradient({ start: "red", end: "blue" }).renderWith(CliEnv, {
		colorLevel: ColorLevel.TrueColor,
	}).text;
	assert.ok(ramped.includes("\u001b[38;2;255;0;0m"));

	const globalRamp = Cfonts.text("A")
		.font(Font.Tiny)
		.globalGradient(GradientPreset.Pride)
		.renderWith(BrowserConsoleEnv, { colorLevel: ColorLevel.TrueColor });
	assert.equal(globalRamp.text.match(/%c/g).length, globalRamp.styles.length);
	assert.ok(globalRamp.styles.includes("color:#750787"));
});
