import assert from "node:assert/strict";
import test from "node:test";

import * as packageExports from "cfonts";

// the Node entry seals the browser host away, the built module answers for it here
import { BrowserHost } from "../../dist/hosts/browser.js";
// the raw boundary takes the terminal facts as one object, so a test can hand it a terminal this process is not
import { EnvironmentKind, Cfonts as WasmCfonts, NodeHost as WasmNodeHost } from "../../pkg/cfonts_wasm.js";

const {
	Align,
	alignNames,
	backgroundColorNames,
	BrowserConsoleEnv,
	BrowserEnv,
	Cfonts,
	CliEnv,
	Color,
	ColorLevel,
	colorNames,
	Font,
	fontNames,
	GradientPreset,
	gradientColorNames,
	gradientPresetNames,
	NodeHost,
	Rgb,
	Valign,
	valignNames,
} = packageExports;

// the names a gradient stop takes, the stop vocabulary the leptos page lists between system and candy
const GRADIENT_COLOR_NAMES = [
	"black",
	"red",
	"green",
	"yellow",
	"blue",
	"magenta",
	"cyan",
	"white",
	"gray",
	"redbright",
	"greenbright",
	"yellowbright",
	"bluebright",
	"magentabright",
	"cyanbright",
	"whitebright",
];

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

// a string is a name and gets the name sentence, every other value is no enum member
const INVALID_ENUM_VALUES = [-1, 1.5, 99, Number.NaN, true, null, undefined];

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

// how many unseeded draws a freshness check takes: a seed is a u32 and a candy render of "AB" makes
// six picks from eleven colors, so eight draws agree only at 2^-224 or 11^-42, bounds no run meets,
// and the roll is guarded without a seam into the entropy source
const FRESH_DRAWS = 8;

function assertFresh(draw) {
	const draws = Array.from({ length: FRESH_DRAWS }, draw);

	assert.ok(new Set(draws).size > 1, "all draws agree");
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
	return colorBanner().renderWith(CliEnv, colorLevel === undefined ? undefined : { color: colorLevel }).text;
}

for (const [method, invoke] of [
	["text", (value) => Cfonts.text(value)],
	["next", (value) => Cfonts.text("A").next(value)],
]) {
	test(`${method} rejects non-string values`, () => {
		assertTypeErrors(INVALID_STRINGS, invoke, `\`${method}()\` expects a string`);
	});
}

test("text inputs accept strings including empty strings", () => {
	Cfonts.text("").next("");
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
	["font", Font, fontNames, "font", (banner, value) => banner.font(value)],
	["align", Align, alignNames, "alignment", (banner, value) => banner.align(value)],
	["valign", Valign, valignNames, "vertical alignment", (banner, value) => banner.valign(value)],
];

for (const [method, enumeration, names, what, invoke] of enumSetters) {
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

	test(`${method} accepts every name in any case and refuses an unknown name with its sentence`, () => {
		for (const name of names()) {
			invoke(Cfonts.text("A"), name);
			invoke(Cfonts.text("A"), name.toUpperCase());
		}

		// an unknown name is a refused value, a plain Error, where a wrong type is a TypeError
		for (const name of ["nope", "0", ""]) {
			assert.throws(() => invoke(Cfonts.text("A"), name), {
				name: "Error",
				message: `There is no ${what} called "${name}"`,
			});
		}
	});
}

test("a name picks the same variant as the enum", () => {
	const context = { color: ColorLevel.TrueColor };
	const render = (banner) => banner.renderWith(CliEnv, context).text;

	assert.equal(render(Cfonts.text("A").font("3d")), render(Cfonts.text("A").font(Font.Font3D)));
	assert.equal(render(Cfonts.text("A").font("Tiny")), render(Cfonts.text("A").font(Font.Tiny)));
	assert.notEqual(render(Cfonts.text("A").font("tiny")), render(Cfonts.text("A").font("3d")));

	const wide = { canvasWidth: 40 };
	assert.equal(
		Cfonts.text("A").align("center").renderWith(CliEnv, wide).text,
		Cfonts.text("A").align(Align.Center).renderWith(CliEnv, wide).text,
	);
	assert.notEqual(
		Cfonts.text("A").align("center").renderWith(CliEnv, wide).text,
		Cfonts.text("A").align("left").renderWith(CliEnv, wide).text,
	);

	const mixed = (valign) =>
		Cfonts.text("A").font(Font.Huge).next("B").font(Font.Tiny).valign(valign).renderWith(CliEnv).text;
	assert.equal(mixed("bottom"), mixed(Valign.Bottom));
	assert.notEqual(mixed("bottom"), mixed("top"));
});

test("the names come from the core in its order, one list per picker", () => {
	// the framework pages fill their pickers from the core's names, the enum keys spell every font but 3d the same way
	const fromEnum = Object.keys(Font)
		.filter((key) => Number.isNaN(Number(key)))
		.map((name) => (name === "Font3D" ? "3d" : name.toLowerCase()));
	assert.deepEqual(fontNames(), fromEnum);
	assert.ok(fontNames().includes("3d"));

	assert.deepEqual(alignNames(), ["left", "center", "right"]);
	assert.deepEqual(valignNames(), ["top", "middle", "bottom"]);
	assert.equal(gradientPresetNames()[0], "pride");
	assert.equal(gradientPresetNames().length, Object.keys(GradientPreset).length / 2);

	// a slot takes system and candy, a background takes system alone, both lists are the help's order
	assert.equal(colorNames()[0], "system");
	assert.equal(colorNames().at(-1), "candy");
	assert.equal(colorNames().length, Object.keys(Color).length / 2);
	assert.deepEqual(backgroundColorNames(), colorNames().slice(0, -1));

	// a gradient stop takes neither, the sixteen names the leptos page lists where a background drops system
	assert.deepEqual(gradientColorNames(), GRADIENT_COLOR_NAMES);
	assert.deepEqual(
		gradientColorNames(),
		backgroundColorNames().filter((name) => name !== "system"),
	);

	// every name is a value the setters take
	for (const name of gradientPresetNames()) {
		Cfonts.text("A").colors(name);
	}
	for (const name of colorNames()) {
		Cfonts.text("A").colors([name]);
	}
	for (const name of backgroundColorNames()) {
		Cfonts.text("A").background(name);
	}
	for (const name of gradientColorNames()) {
		Cfonts.text("A").colors({ start: name, end: "blue" });
		Cfonts.text("A").background({ transition: [name, "blue"] });
	}
});

test("a throwing array getter or proxy trap crosses back as itself and leaves the builder usable", () => {
	// a getter on the first entry of a colors list throws, the builder takes settings and renders afterwards
	const banner = Cfonts.text("A");
	const colors = [Color.Red];
	Object.defineProperty(colors, "0", {
		get() {
			throw new RangeError("array getter");
		},
	});
	assert.throws(() => banner.colors(colors), { name: "RangeError", message: "array getter" });
	assert.equal(banner.font(Font.Tiny), banner);
	assert.equal(banner.renderWith(CliEnv).text, Cfonts.text("A").font(Font.Tiny).renderWith(CliEnv).text);

	const thrown = new RangeError("the consumer's own");
	const same = (error) => error === thrown;
	const expected = Cfonts.text("A").font(Font.Tiny).renderWith(CliEnv).text;
	const lists = [
		[
			"a throwing getter on an entry",
			() =>
				Object.defineProperty([Color.Red, Color.Blue], "0", {
					get() {
						throw thrown;
					},
				}),
		],
		[
			"a proxy with throwing get, has and ownKeys traps",
			() =>
				new Proxy([Color.Red, Color.Blue], {
					get() {
						throw thrown;
					},
					has() {
						throw thrown;
					},
					ownKeys() {
						throw thrown;
					},
				}),
		],
	];
	const places = [
		["colors", (banner, list) => banner.colors(list)],
		["globalColors", (banner, list) => banner.globalColors(list)],
		["colors transition", (banner, list) => banner.colors({ transition: list })],
		["globalColors transition", (banner, list) => banner.globalColors({ transition: list })],
		["background transition", (banner, list) => banner.background({ transition: list })],
	];
	for (const [what, list] of lists) {
		for (const [place, call] of places) {
			const banner = Cfonts.text("A");
			// the exception is the consumer's own object, and it crosses as a result, so the borrow of the builder
			// releases and every later call works
			assert.throws(() => call(banner, list()), same, `${place} with ${what}`);
			assert.equal(banner.font(Font.Tiny), banner, `${place} with ${what}`);
			assert.equal(banner.renderWith(CliEnv).text, expected, `${place} with ${what}`);
		}
	}

	// a throwing getter on a member of the gradient object crosses the same way
	const members = [
		[
			"colors start",
			(banner) =>
				banner.colors({
					get start() {
						throw thrown;
					},
					end: Color.Blue,
				}),
		],
		[
			"colors end",
			(banner) =>
				banner.colors({
					start: Color.Red,
					get end() {
						throw thrown;
					},
				}),
		],
		[
			"colors transition",
			(banner) =>
				banner.colors({
					get transition() {
						throw thrown;
					},
				}),
		],
		[
			"background start",
			(banner) =>
				banner.background({
					get start() {
						throw thrown;
					},
					end: Color.Blue,
				}),
		],
		[
			"background transition",
			(banner) =>
				banner.background({
					get transition() {
						throw thrown;
					},
				}),
		],
	];
	for (const [place, call] of members) {
		const banner = Cfonts.text("A");
		assert.throws(() => call(banner), same, place);
		assert.equal(banner.font(Font.Tiny), banner, place);
		assert.equal(banner.renderWith(CliEnv).text, expected, place);
	}

	// the iterator of a list is never invoked, so a throwing one changes nothing, nor do a has or an ownKeys trap alone
	const iterator = () =>
		Object.defineProperty([Color.Red, Color.Blue], Symbol.iterator, {
			value() {
				throw thrown;
			},
		});
	const untouched = () =>
		new Proxy([Color.Red, Color.Blue], {
			has() {
				throw thrown;
			},
			ownKeys() {
				throw thrown;
			},
		});
	for (const [place, call] of places) {
		assert.equal(call(Cfonts.text("A").font(Font.Tiny), iterator()).renderWith(CliEnv).text, expected, place);
		assert.equal(call(Cfonts.text("A").font(Font.Tiny), untouched()).renderWith(CliEnv).text, expected, place);
	}

	// a proxy answering no number for its length reads as zero entries, no index is read and the string "2"
	// is not read as two, so a transition over it holds zero stops
	const reads = { index: 0 };
	const unnumbered = new Proxy([Color.Red, Color.Blue], {
		get(target, key, receiver) {
			if (key === "length") {
				return "2";
			}
			if (typeof key === "string") {
				reads.index += 1;
			}
			return Reflect.get(target, key, receiver);
		},
	});
	assert.equal(Cfonts.text("A").font(Font.Tiny).colors(unnumbered).renderWith(CliEnv).text, expected);
	assert.throws(() => Cfonts.text("A").colors({ transition: unnumbered }), {
		name: "Error",
		message: "A transition gradient holds at least two stops, this one holds 0",
	});
	assert.equal(reads.index, 0);

	// a revoked proxy makes Array.isArray itself throw, the engine's TypeError crosses the same way
	const revoked = () => {
		const { proxy, revoke } = Proxy.revocable([Color.Red], {});
		revoke();
		return proxy;
	};
	for (const call of [(banner) => banner.colors(revoked()), (banner) => banner.renderWith(CliEnv, revoked())]) {
		const banner = Cfonts.text("A");
		assert.throws(() => call(banner), { name: "TypeError" });
		assert.equal(banner.font(Font.Tiny), banner);
		assert.equal(banner.renderWith(CliEnv).text, expected);
	}
});

test("colors and background read every member of an object once", () => {
	// an object whose getters answer the given values and count their reads
	const counting = (keys, values) => {
		const counts = {};
		const object = {};
		for (const key of keys) {
			counts[key] = 0;
			Object.defineProperty(object, key, {
				enumerable: true,
				get() {
					counts[key] += 1;
					return values[key];
				},
			});
		}
		return [object, counts];
	};
	const once = (counts, what) => {
		for (const [key, count] of Object.entries(counts)) {
			assert.equal(count, 1, `${what} reads ${key} ${count} times`);
		}
	};
	const GRADIENT = ["preset", "start", "end", "transition"];
	const CHANNELS = ["red", "green", "blue"];
	const readers = [
		["colors", (object) => Cfonts.text("A").colors(object)],
		["globalColors", (object) => Cfonts.text("A").globalColors(object)],
		["background", (object) => Cfonts.text("A").background(object)],
	];

	// a gradient shape, where every gradient member is read to tell the shapes apart
	for (const shape of [
		{ preset: GradientPreset.Pride },
		{ start: Color.Red, end: Color.Blue },
		{ transition: [Color.Red, Color.Blue] },
	]) {
		for (const [method, read] of readers) {
			const [object, counts] = counting(GRADIENT, shape);
			read(object);
			once(counts, `${method}({${Object.keys(shape)}})`);
		}
	}

	// channels as a background, where the four gradient members are read to tell the shapes apart
	const [channels, counts] = counting([...CHANNELS, ...GRADIENT], { red: 1, green: 2, blue: 3 });
	Cfonts.text("A").background(channels);
	once(counts, "background(channels)");

	// channels in a slot list, as a stop and in a transition
	for (const [place, wrap] of [
		["a slot", (object) => [object]],
		["a start", (object) => ({ start: object, end: Color.Blue })],
		["an end", (object) => ({ start: Color.Red, end: object })],
		["a transition stop", (object) => ({ transition: [object, Color.Blue] })],
	]) {
		for (const [method, read] of readers.slice(0, 2)) {
			const [object, counts] = counting(CHANNELS, { red: 1, green: 2, blue: 3 });
			read(wrap(object));
			once(counts, `${method} with channels as ${place}`);
		}
	}
});

test("a shape error in any position wins over a refused value in any position, the first of each in order", () => {
	const enumSentence = (method) => ({ name: "TypeError", message: `\`${method}()\` expects a supported enum value` });
	const channelSentence = (method) => ({
		name: "TypeError",
		message: `\`${method}()\` expects RGB channel values as integers between 0 and 255`,
	});
	const unknown = (input) => ({
		name: "Error",
		message: `"${input}": A color is either a color name or a hex value like #ff8800`,
	});
	const notAStop = (input) => ({
		name: "Error",
		message: `"${input}": A gradient stop is any color name or a hex value like #ff8800 except system and candy`,
	});

	// a list: the shape of every entry is checked before any value is parsed
	assert.throws(() => Cfonts.text("A").colors(["nonsense", 42e9]), enumSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors([{ red: 1 }, "red"]), channelSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors([{ red: 1 }, 42e9]), channelSentence("colors")); // the first shape error
	assert.throws(() => Cfonts.text("A").colors([42e9, { red: 1 }]), enumSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors(["nonsense", "#zz"]), unknown("nonsense")); // the first refused value
	assert.throws(() => Cfonts.text("A").colors(["#zz", "nonsense"]), {
		name: "Error",
		message: '"#zz": A hex color can only hold hex digits 0-9 and A-F',
	});
	assert.throws(() => Cfonts.text("A").colors([{ red: 1, green: 2, blue: 3 }, "nonsense"]), unknown("nonsense"));
	assert.throws(() => Cfonts.text("A").globalColors(["nonsense", 42e9]), enumSentence("globalColors"));

	// two stops: both shapes are checked before either value is parsed
	assert.throws(() => Cfonts.text("A").colors({ start: "nonsense", end: 42e9 }), enumSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors({ start: "nonsense", end: { red: 1 } }), channelSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors({ start: Color.Candy, end: "nonsense" }), notAStop("candy"));
	assert.throws(() => Cfonts.text("A").colors({ start: "nonsense", end: Color.Candy }), unknown("nonsense"));
	assert.throws(() => Cfonts.text("A").background({ start: 42e9, end: "nonsense" }), enumSentence("background"));
	assert.throws(() => Cfonts.text("A").background({ start: Color.System, end: "red" }), notAStop("system"));

	// a transition: every stop is parsed before the count is checked
	assert.throws(() => Cfonts.text("A").colors({ transition: ["nonsense", 42e9] }), enumSentence("colors"));
	assert.throws(() => Cfonts.text("A").colors({ transition: [Color.Candy, "nonsense"] }), notAStop("candy"));
	assert.throws(() => Cfonts.text("A").colors({ transition: ["nonsense"] }), unknown("nonsense"));
	assert.throws(() => Cfonts.text("A").colors({ transition: [{ red: 1, green: 2, blue: 3 }] }), {
		name: "Error",
		message: "A transition gradient holds at least two stops, this one holds 1",
	});
	assert.throws(
		() => Cfonts.text("A").colors({ transition: [Color.System, { red: 1, green: 2, blue: 3 }] }),
		notAStop("system"),
	);

	// a background: one color by its value or its spelling, channels, or a gradient shape
	assert.throws(() => Cfonts.text("A").background(Color.Candy), unknown("candy"));
	assert.throws(() => Cfonts.text("A").background(42e9), enumSentence("background"));
	assert.throws(() => Cfonts.text("A").background({ red: 1, green: 2, blue: 3, preset: 0 }), {
		name: "TypeError",
		message: /^`background\(\)` expects a background as a Color value/,
	});
	assert.throws(() => Cfonts.text("A").background({ red: 1 }), channelSentence("background"));
	Cfonts.text("A").background(Color.System);
});

test("channel values build what the hex spelling builds in every place", () => {
	// the channels convert without a spelling
	const context = { color: ColorLevel.TrueColor };
	const render = (banner) => banner.renderWith(CliEnv, context).text;
	const channels = { red: 1, green: 2, blue: 3 };
	assert.equal(render(Cfonts.text("A").colors([channels])), render(Cfonts.text("A").colors(["#010203"])));
	assert.equal(render(Cfonts.text("A").background(channels)), render(Cfonts.text("A").background("#010203")));
	assert.equal(
		render(Cfonts.text("A").colors({ start: channels, end: Color.Blue })),
		render(Cfonts.text("A").colors({ start: "#010203", end: Color.Blue })),
	);
	assert.equal(
		render(Cfonts.text("A").colors({ transition: [Color.Red, channels] })),
		render(Cfonts.text("A").colors({ transition: [Color.Red, "#010203"] })),
	);
	assert.notEqual(render(Cfonts.text("A").background(channels)), render(Cfonts.text("A"))); // the channels paint
});

test("a wrong shape is a TypeError and a refused value a plain Error", () => {
	// the shape sentences name the method, the value sentences are the core's
	assert.throws(() => Cfonts.text("A").colors([true]), {
		name: "TypeError",
		message: "`colors()` expects colors as Color values, names, hex values, or {red, green, blue} channels",
	});
	assert.throws(() => Cfonts.text("A").colors([{ red: 256, green: 0, blue: 0 }]), {
		name: "TypeError",
		message: "`colors()` expects RGB channel values as integers between 0 and 255",
	});
	assert.throws(() => Cfonts.text("A").colors({ start: true, end: "blue" }), {
		name: "TypeError",
		message:
			'`colors()` gradient stops take any Color but Color.System and Color.Candy, a stop name such as "red", ' +
			'a hex value such as "#ff8800", or {red, green, blue} channels from Rgb.fromHex()',
	});
	assert.throws(() => Cfonts.text("A").background(true), {
		name: "TypeError",
		message:
			'`background()` expects a background as a Color value, the command line spelling such as "red-blue", ' +
			"{red, green, blue} channels, or a gradient shape such as {start: Color.Red, end: Color.Blue} " +
			"or {preset: GradientPreset.Pride}",
	});
	assert.throws(() => Cfonts.text("A").globalColors(true), {
		name: "TypeError",
		message:
			'`globalColors()` expects an array of colors such as [Color.Red, "#8899dd"], ' +
			"exactly one gradient shape such as {start: Color.Red, end: Color.Blue}, " +
			'{transition: [Color.Red, "#8899dd", Color.Blue]} or {preset: GradientPreset.Pride}, ' +
			'or the command line spelling such as "red-blue"',
	});

	assert.throws(() => Cfonts.text("A").colors(["reed"]), {
		name: "Error",
		message: '"reed": A color is either a color name or a hex value like #ff8800',
	});
	assert.throws(() => Cfonts.text("A").colors({ transition: [] }), {
		name: "Error",
		message: "A transition gradient holds at least two stops, this one holds 0",
	});
	assert.throws(() => Cfonts.text("A").background({ red: 1, green: 2, blue: 3 }).background("blue"), {
		name: "Error",
		message: "`background()` has already been set",
	});
});

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
		'<div style="font-family:ui-monospace,Menlo,Consolas,DejaVu Sans Mono,monospace;white-space:pre;text-align:left;max-width:100%;overflow:auto">▄▀█ ▄▀█<br>█▀█ █▀█</div>',
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
	const kind = Object.getOwnPropertySymbols(CliEnv).find((symbol) => symbol.description === "cfonts.environment");
	const forged = Object.freeze({ [kind]: 99 });
	assert.throws(() => wrappingBanner().renderWith(forged), {
		name: "TypeError",
		message: "`renderWith()` expects a cfonts environment",
	});
});

test("renderWith rejects invalid overrides", () => {
	for (const context of [null, 0, true, "", [], () => {}]) {
		assert.throws(() => wrappingBanner().renderWith(CliEnv, context), {
			name: "TypeError",
			message: "`renderWith()` expects an overrides object",
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

test("custom hosts receive the composition and the environment exactly once", () => {
	const composition = Cfonts.text("A");
	let renderCalls = 0;
	let sayCalls = 0;

	const host = {
		render(received, environment) {
			assert.equal(received, composition);
			assert.equal(environment, CliEnv);
			renderCalls += 1;

			return {
				text: "custom render",
			};
		},
		say(received, environment) {
			assert.equal(received, composition);
			assert.equal(environment, CliEnv);
			sayCalls += 1;
		},
	};

	assert.deepEqual(composition.render(host, CliEnv), { text: "custom render" });
	composition.say(host, CliEnv);

	assert.equal(renderCalls, 1);
	assert.equal(sayCalls, 1);
});

test("render rejects values without a render method", () => {
	const composition = Cfonts.text("A");

	for (const host of [undefined, null, 0, true, "", {}, [], { say() {} }]) {
		assert.throws(() => composition.render(host, CliEnv), {
			name: "TypeError",
			message: "`render()` expects a cfonts host",
		});
	}
});

test("say rejects values without a say method", () => {
	const composition = Cfonts.text("A");

	for (const host of [undefined, null, 0, true, "", {}, [], { render() {} }]) {
		assert.throws(() => composition.say(host, CliEnv), {
			name: "TypeError",
			message: "`say()` expects a cfonts host",
		});
	}
});

test("NodeHost validates override objects", () => {
	for (const overrides of [null, 0, true, "", [], () => {}]) {
		assert.throws(() => NodeHost.fromOverrides(overrides), {
			name: "TypeError",
			message: "`fromOverrides()` expects an overrides object",
		});
		assert.throws(() => new NodeHost(overrides), {
			name: "TypeError",
			message: "`fromOverrides()` expects an overrides object",
		});
	}
});

test("the constructor takes the overrides object fromOverrides takes", () => {
	const banner = Cfonts.text("AAAA");
	const render = (host) => withTerminal(120, undefined, () => banner.render(host, CliEnv).text);

	// no object and undefined build the detecting host, an object pins what it names either way
	assert.equal(render(new NodeHost(undefined)), render(new NodeHost()));
	assert.equal(render(NodeHost.fromOverrides(undefined)), render(new NodeHost()));
	assert.equal(render(new NodeHost({ canvasWidth: 13 })), render(NodeHost.fromOverrides({ canvasWidth: 13 })));
	assert.notEqual(render(new NodeHost({ canvasWidth: 13 })), render(new NodeHost()));
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

	const narrow = withTerminal(13, undefined, () => banner.render(host, CliEnv).text);
	const wide = withTerminal(120, undefined, () => banner.render(host, CliEnv).text);

	assert.notEqual(narrow, wide);
});

test("stdout answers before stderr", () => {
	const restoreStdout = overrideProperty(process.stdout, "columns", 120);
	const restoreStderr = overrideProperty(process.stderr, "columns", 13);

	try {
		const preferred = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
		const viaStdout = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

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
		const viaStderr = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
		const reference = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

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
		const viaStderr = withEnv("FORCE_SIZE", undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
		const viaStdout = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

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
		const fallback = withEnv(
			"FORCE_SIZE",
			undefined,
			() => Cfonts.text("AAAAAAAAAA").render(new NodeHost(), CliEnv).text,
		);
		const eighty = withEnv(
			"FORCE_SIZE",
			undefined,
			() => Cfonts.text("AAAAAAAAAA").render(NodeHost.fromOverrides({ canvasWidth: 80 }), CliEnv).text,
		);
		const unlimited = withEnv(
			"FORCE_SIZE",
			undefined,
			() => Cfonts.text("AAAAAAAAAA").render(NodeHost.fromOverrides({ canvasWidth: 0 }), CliEnv).text,
		);

		assert.equal(fallback, eighty);
		assert.notEqual(fallback, unlimited);
	} finally {
		restoreStderr();
		restoreStdout();
	}
});

test("FORCE_SIZE overrides terminal detection", () => {
	const forced = withTerminal(120, "13", () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

	const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

	assert.equal(forced, detected);
});

test("FORCE_SIZE overrides an API width", () => {
	const forcedHost = NodeHost.fromOverrides({
		canvasWidth: 120,
	});

	const expectedHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const forced = withTerminal(120, "13", () => Cfonts.text("AAAA").render(forcedHost, CliEnv).text);

	const expected = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(expectedHost, CliEnv).text);

	assert.equal(forced, expected);
});

test("FORCE_SIZE zero overrides an API width with unlimited output", () => {
	const forcedHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const unlimitedHost = NodeHost.fromOverrides({
		canvasWidth: 0,
	});

	const forced = withTerminal(13, "0", () => Cfonts.text("AAAA").render(forcedHost, CliEnv).text);

	const expected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(unlimitedHost, CliEnv).text);

	assert.equal(forced, expected);
});

test("an API width overrides terminal detection", () => {
	const explicitHost = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	const explicit = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(explicitHost, CliEnv).text);

	const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

	assert.equal(explicit, detected);
});

test("an API width of zero means unlimited", () => {
	const unlimitedHost = NodeHost.fromOverrides({
		canvasWidth: 0,
	});

	const unlimited = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(unlimitedHost, CliEnv).text);

	const wide = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

	assert.equal(unlimited, wide);
});

test("invalid FORCE_SIZE falls through to the API override", () => {
	const host = NodeHost.fromOverrides({
		canvasWidth: 13,
	});

	for (const garbage of ["", "abc", "-1", "12.5", "4294967296"]) {
		const ignored = withTerminal(120, garbage, () => Cfonts.text("AAAA").render(host, CliEnv).text);

		const expected = withTerminal(120, undefined, () => Cfonts.text("AAAA").render(host, CliEnv).text);

		assert.equal(ignored, expected, `FORCE_SIZE=${JSON.stringify(garbage)} must fall through`);
	}
});

test("NodeHost render does not write to stdout", () => {
	withTerminal(80, undefined, () => {
		const writes = captureStdout(() => {
			Cfonts.text("A").render(new NodeHost(), CliEnv);
		});

		assert.deepEqual(writes, []);
	});
});

test("NodeHost say writes exactly once", () => {
	withTerminal(80, undefined, () => {
		const banner = Cfonts.text("A");
		const host = new NodeHost();
		const expected = banner.render(host, CliEnv).text;

		const writes = captureStdout(() => {
			banner.say(host, CliEnv);
		});

		assert.deepEqual(writes, [`${expected}\n`]);
	});
});

test("NodeHost say ends a browser artifact without a line end", () => {
	withTerminal(80, undefined, () => {
		const banner = Cfonts.text("A");
		const host = new NodeHost();
		const expected = banner.render(host, BrowserEnv).text;

		const writes = captureStdout(() => {
			banner.say(host, BrowserEnv);
		});

		assert.deepEqual(writes, [expected]);
	});
});

test("raw mode changes nothing but the line endings", () => {
	withTerminal(120, undefined, () => {
		// blank rows from lineHeight and the paddings travel through the same endings as the glyph rows,
		// and the host keeps its overrides, here a width the banner wraps at
		const banner = Cfonts.text("AAAA").font(Font.Tiny).lineHeight(2);
		const host = NodeHost.fromOverrides({ canvasWidth: 13 });
		const plain = banner.render(host, CliEnv).text;

		const raw = banner.render(host, CliEnv.rawMode()).text;
		assert.ok(raw.includes("\r\n"));
		assert.deepEqual(raw.split("\r\n"), plain.split("\n"));
		assert.notEqual(plain, banner.render(new NodeHost(), CliEnv).text); // the override is the host's
	});
});

test("raw mode reaches a manual render through the terminal environment", () => {
	const banner = Cfonts.text("AB").font(Font.Tiny);
	const raw = banner.renderWith(CliEnv.rawMode()).text;
	const plain = banner.renderWith(CliEnv).text;

	assert.deepEqual(raw.split("\r\n"), plain.split("\n"));
	assert.equal(CliEnv.rawMode(), CliEnv.rawMode()); // the raw terminal is a single value
	for (const environment of [CliEnv, CliEnv.rawMode(), BrowserEnv, BrowserConsoleEnv]) {
		assert.ok(Object.isFrozen(environment)); // shared values that nobody can reshape
	}
});

test("say ends raw output with a carriage return line feed", () => {
	withTerminal(80, undefined, () => {
		const banner = Cfonts.text("A");
		const host = new NodeHost();
		const expected = banner.render(host, CliEnv.rawMode()).text;

		const writes = captureStdout(() => {
			banner.say(host, CliEnv.rawMode());
		});

		assert.deepEqual(writes, [`${expected}\r\n`]);
	});
});

test("FORCE_SIZE zero means unlimited", () => {
	const unlimited = withTerminal(13, "0", () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
	const wide = withTerminal(13, "120", () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

	assert.equal(unlimited, wide);
});

test("FORCE_SIZE garbage falls through to detection", () => {
	for (const garbage of ["", "abc", "-1", "12.5"]) {
		const ignored = withTerminal(13, garbage, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
		const detected = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);

		assert.equal(ignored, detected, `FORCE_SIZE=${JSON.stringify(garbage)} must fall through`);
	}
});

test("color overrides are validated", () => {
	assert.throws(() => NodeHost.fromOverrides({ color: 99 }), TypeError);
	assert.throws(() => NodeHost.fromOverrides({ seed: -1 }), TypeError);
	NodeHost.fromOverrides({ color: false });
	NodeHost.fromOverrides({ color: ColorLevel.Basic, seed: 42 });
});

test("renderWith validates the color override fields", () => {
	assert.throws(() => Cfonts.text("A").renderWith(CliEnv, { color: 99 }), TypeError);
	assert.throws(() => Cfonts.text("A").renderWith(CliEnv, { seed: 1.5 }), TypeError);
});

test("a color level without color options paints nothing", () => {
	const plain = withTerminal(13, undefined, () => Cfonts.text("AAAA").render(new NodeHost(), CliEnv).text);
	const leveled = withTerminal(
		13,
		undefined,
		() => Cfonts.text("AAAA").render(NodeHost.fromOverrides({ color: ColorLevel.TrueColor, seed: 42 }), CliEnv).text,
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

test("a string is the command line spelling, parsed where the command line parses it", () => {
	const context = { color: ColorLevel.TrueColor };
	// a font with two color slots, so a list reaches past its first color
	const render = (banner) => banner.font(Font.Block).renderWith(CliEnv, context).text;

	assert.equal(
		render(Cfonts.text("A").colors("red-blue")),
		render(Cfonts.text("A").colors({ start: "red", end: "blue" })),
	);
	assert.equal(
		render(Cfonts.text("A").globalColors("red,#8899dd")),
		render(Cfonts.text("A").globalColors(["red", "#8899dd"])),
	);
	assert.equal(
		render(Cfonts.text("A").background("red-blue")),
		render(Cfonts.text("A").background({ start: "red", end: "blue" })),
	);
	assert.equal(
		render(Cfonts.text("A").colors("pride")),
		render(Cfonts.text("A").colors({ preset: GradientPreset.Pride })),
	);
	assert.notEqual(render(Cfonts.text("A").colors("pride")), render(Cfonts.text("A"))); // the spelling paints

	// a refused spelling carries the core's sentence, the one every page prints
	assert.throws(() => Cfonts.text("A").colors("red-blue-green"), {
		message: '"red-blue-green": A gradient holds exactly two colors, this one holds 3',
	});
	assert.throws(() => Cfonts.text("A").globalColors("red-blue-green"), {
		message: '"red-blue-green": A gradient holds exactly two colors, this one holds 3',
	});
	assert.throws(() => Cfonts.text("A").background("red,blue"), {
		message: '"red,blue": A background takes one color, a gradient or a preset, not a list',
	});
});

test("colors validates its input", () => {
	assert.throws(() => Cfonts.text("A").colors([99]), TypeError); // not a Color
	assert.throws(() => Cfonts.text("A").colors([{ red: 256, green: 0, blue: 0 }]), TypeError); // not a channel value
	assert.throws(() => Cfonts.text("A").colors([true]), TypeError);
	assert.throws(() => Cfonts.text("A").colors(["reed"]), Error); // unknown name, rejected in Rust
	assert.throws(() => Cfonts.text("A").colors(["#ff88"]), Error); // invalid hex, rejected in Rust
});

test("gradient shapes are validated", () => {
	assert.throws(() => Cfonts.text("A").colors({}), TypeError); // no shape
	assert.throws(() => Cfonts.text("A").colors({ start: "red", transition: ["red", "blue"] }), TypeError); // two shapes
	assert.throws(() => Cfonts.text("A").colors({ start: "red" }), TypeError); // missing end
	assert.throws(() => Cfonts.text("A").colors({ transition: "red" }), TypeError); // not an array
	assert.throws(() => Cfonts.text("A").colors({ preset: 99 }), TypeError); // not a preset
	assert.throws(() => Cfonts.text("A").colors(99), TypeError);
	assert.throws(() => Cfonts.text("A").colors(GradientPreset.Pride), TypeError); // a bare preset is a number, presets go in their object form
	assert.throws(() => Cfonts.text("A").globalColors(GradientPreset.Pride), TypeError);
	assert.throws(() => Cfonts.text("A").colors({ transition: [] }), /at least two stops, this one holds 0/); // empty, rejected in Rust
	assert.throws(() => Cfonts.text("A").colors({ transition: ["red"] }), /at least two stops, this one holds 1/); // one stop, rejected in Rust
	assert.throws(() => Cfonts.text("A").globalColors({ transition: ["red"] }), /at least two stops, this one holds 1/);
	assert.throws(() => Cfonts.text("A").colors({ start: "system", end: "blue" }), /"system": A gradient stop/); // system is not a gradient stop

	// a member left undefined is no shape, as the types say
	Cfonts.text("A").colors({ preset: GradientPreset.Pride, start: undefined });
	Cfonts.text("A").background({ red: 1, green: 2, blue: 3, preset: undefined });
});

test("gradient stops accept the base Color values", () => {
	const context = { color: ColorLevel.TrueColor };
	const named = Cfonts.text("A").colors({ start: "red", end: "blue" }).renderWith(CliEnv, context).text;
	const typed = Cfonts.text("A").colors({ start: Color.Red, end: Color.Blue }).renderWith(CliEnv, context).text;
	assert.equal(typed, named);

	const transition = Cfonts.text("A")
		.colors({ transition: [Color.Red, "#8899dd", { red: 0, green: 0, blue: 255 }] })
		.renderWith(CliEnv, context).text;
	const spelled = Cfonts.text("A")
		.colors({ transition: ["red", "#8899dd", "#0000ff"] })
		.renderWith(CliEnv, context).text;
	assert.equal(transition, spelled);

	const global = Cfonts.text("A").globalColors({ start: Color.Yellow, end: Color.Gray }).renderWith(CliEnv, context);
	assert.ok(global.text.includes("\u001b[38;2;"));
});

test("gradient stops outside the blendable palette are rejected in Rust", () => {
	// valid enum members travel as their names; which colors may blend is Rust's decision
	for (const color of [Color.System, Color.Candy]) {
		assert.throws(() => Cfonts.text("A").colors({ start: color, end: Color.Blue }), /except system and candy/);
	}

	assert.throws(() => Cfonts.text("A").colors({ transition: [Color.Red, Color.Candy] }), /except system and candy/);

	// an unknown number is still a shape error, caught at the boundary
	assert.throws(() => Cfonts.text("A").colors({ start: 99, end: Color.Blue }), {
		name: "TypeError",
		message: /supported enum value/,
	});
});

test("a bright color is a gradient stop with the value of its slot color", () => {
	const context = { color: ColorLevel.TrueColor };
	const bright = Cfonts.text("A").font(Font.Tiny).colors({ start: Color.RedBright, end: Color.Blue });
	const spelled = Cfonts.text("A").font(Font.Tiny).colors({ start: "#ee776d", end: Color.Blue });

	assert.ok(bright.renderWith(CliEnv, context).text.includes("\u001b[38;2;"));
	assert.equal(bright.renderWith(CliEnv, context).text, spelled.renderWith(CliEnv, context).text);
});

test("gradient shape errors teach the shapes", () => {
	assert.throws(() => Cfonts.text("A").colors({}), {
		name: "TypeError",
		message: /`colors\(\)` expects an array of colors.*start: Color\.Red.*preset: GradientPreset\.Pride/,
	});
	assert.throws(() => Cfonts.text("A").globalColors(GradientPreset.Pride), {
		name: "TypeError",
		message: /`globalColors\(\)` expects an array of colors.*preset: GradientPreset\.Pride/,
	});
	assert.throws(() => Cfonts.text("A").colors({ start: Color.Red }), {
		name: "TypeError",
		message: /both start and end/,
	});
	assert.throws(() => Cfonts.text("A").colors({ transition: "red" }), {
		name: "TypeError",
		message: /two or more/,
	});
});

test("Rgb.fromHex converts hex values into channels", () => {
	assert.deepEqual(Rgb.fromHex("#ff8800"), { red: 255, green: 136, blue: 0 });
	assert.deepEqual(Rgb.fromHex("f80"), { red: 255, green: 136, blue: 0 });
	assert.ok(Object.isFrozen(Rgb.fromHex("#ff8800")));
	assert.ok(Object.isFrozen(Rgb)); // the one method hangs off a value nobody can reshape
	assert.equal("rgbFromHex" in packageExports, false); // the raw boundary function stays behind the method

	assert.throws(() => Rgb.fromHex("#ff88"), Error); // four digits are invalid
	assert.throws(() => Rgb.fromHex("teal"), Error); // names are not hex values
	assert.throws(() => Rgb.fromHex(42), { name: "TypeError", message: "`Rgb.fromHex()` expects a string" });

	const context = { color: ColorLevel.TrueColor };
	const channeled = Cfonts.text("A")
		.colors({ start: Rgb.fromHex("#ff8800"), end: Color.Blue })
		.renderWith(CliEnv, context).text;
	const spelled = Cfonts.text("A").colors({ start: "#ff8800", end: "blue" }).renderWith(CliEnv, context).text;
	assert.equal(channeled, spelled);
});

test("gradients accept every shape and paint nothing without a color level", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;

	const preset = Cfonts.text("A").colors({ preset: GradientPreset.Pride }).renderWith(CliEnv).text;
	const twoStop = Cfonts.text("A")
		.colors({ start: "red", end: "#0000ff" })
		.independentGradient()
		.renderWith(CliEnv).text;
	const transition = Cfonts.text("A")
		.colors({ transition: ["red", { red: 0, green: 0, blue: 255 }, "gray"] })
		.renderWith(CliEnv).text;
	const global = Cfonts.text("A")
		.globalColors({ preset: GradientPreset.Transgender })
		.independentGradient()
		.renderWith(CliEnv).text;

	for (const rendered of [preset, twoStop, transition, global]) {
		assert.equal(rendered, plain);
	}
});

test("globalColors accepts colors and paints nothing without a color level", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;
	const global = Cfonts.text("A").globalColors([Color.Red, "#ff8800"]).renderWith(CliEnv).text;

	assert.equal(global, plain);
	assert.throws(() => Cfonts.text("A").globalColors(true), TypeError); // neither a spelling, a list nor a gradient shape
	assert.throws(() => Cfonts.text("A").globalColors(["reed"]), Error); // unknown name, rejected in Rust
});

test("global colors and a global gradient share the one global slot", () => {
	const shapes = [{ preset: GradientPreset.Pride }, { start: "red", end: "blue" }, { transition: ["red", "blue"] }];

	const colored = Cfonts.text("A").globalColors([Color.Red]);
	for (const shape of shapes) {
		assert.throws(() => colored.globalColors(shape), /global color has already been set/);
	}

	for (const shape of shapes) {
		const ramped = Cfonts.text("A").globalColors(shape);
		assert.throws(() => ramped.globalColors([Color.Red]), /global color has already been set/);
	}
});

test("the global colors can only be set once", () => {
	const banner = Cfonts.text("A").globalColors({ preset: GradientPreset.Pride });

	assert.throws(() => banner.globalColors({ preset: GradientPreset.Agender }), Error);
	assert.throws(() => banner.globalColors({ start: "red", end: "blue" }), Error);
});

test("a failed global gradient does not claim the slot", () => {
	const banner = Cfonts.text("A");

	assert.throws(() => banner.globalColors({ start: "reed", end: "blue" }), Error);
	banner.globalColors({ start: "red", end: "blue" }); // the slot is still available
});

test("background accepts every shape and paints nothing without a color level", () => {
	const plain = Cfonts.text("A").renderWith(CliEnv).text;
	const shapes = [
		Color.Blue,
		Color.System,
		"blue",
		"system",
		"#222",
		{ red: 1, green: 2, blue: 3 },
		{ start: "red", end: "blue" },
		{ transition: [Color.Red, "#8899dd", Color.Blue] },
		{ preset: GradientPreset.Pride },
	];

	for (const background of shapes) {
		assert.equal(Cfonts.text("A").background(background).renderWith(CliEnv).text, plain);
	}
});

test("a background bands every row in every environment", () => {
	const context = { color: ColorLevel.TrueColor };
	const banner = Cfonts.text("A").font(Font.Tiny).spaceless().background(Color.Blue);

	assert.equal(
		banner.renderWith(CliEnv, context).text,
		"\u001b[44m\u001b[K▄▀█\u001b[49m\n\u001b[44m\u001b[K█▀█\u001b[49m",
	);
	assert.ok(
		banner.renderWith(BrowserEnv, context).text.includes('<div style="background:#0020f5;min-height:1lh">▄▀█</div>'),
	);
	assert.deepEqual(banner.renderWith(BrowserConsoleEnv, context).styles, [
		"background:#0020f5",
		"",
		"background:#0020f5",
		"",
	]);
});

test("a background gradient ramps from the top row down", () => {
	const context = { color: ColorLevel.TrueColor };
	const render = (background) =>
		Cfonts.text("A").font(Font.Tiny).spaceless().background(background).renderWith(CliEnv, context).text;

	const rows = render({ start: "red", end: "blue" }).split("\n");
	assert.ok(rows[0].startsWith("\u001b[48;2;255;0;0m"));
	assert.ok(rows[1].startsWith("\u001b[48;2;0;0;255m"));

	// a transition over the same two stops walks the same ramp, a preset walks its own
	assert.equal(render({ transition: ["red", "blue"] }), render({ start: "red", end: "blue" }));
	const preset = render({ preset: GradientPreset.Pride }).split("\n");
	assert.ok(preset.every((row) => row.startsWith("\u001b[48;2;")));
	assert.notEqual(preset.join("\n"), rows.join("\n"));
});

test("background validates its input", () => {
	assert.throws(() => Cfonts.text("A").background(true), TypeError);
	assert.throws(() => Cfonts.text("A").background({}), TypeError); // no shape
	assert.throws(() => Cfonts.text("A").background({ red: 256, green: 0, blue: 0 }), TypeError); // not a channel value
	assert.throws(() => Cfonts.text("A").background({ start: "red" }), TypeError); // missing end
	assert.throws(() => Cfonts.text("A").background({ red: 1, green: 2, blue: 3, start: "red", end: "blue" }), TypeError); // two shapes
	assert.throws(
		() => Cfonts.text("A").background({ preset: GradientPreset.Pride, start: "red", end: "blue" }),
		/expects a background/,
	); // two gradient shapes teach the background shapes
	assert.throws(() => Cfonts.text("A").background("reed"), /"reed": A color is either a color name/); // unknown name, rejected in Rust
	// candy cannot fill a row, refused like an unknown name and echoed by its command line name, the core's spelling
	assert.throws(() => Cfonts.text("A").background(Color.Candy), {
		name: "Error",
		message: '"candy": A color is either a color name or a hex value like #ff8800',
	});
	assert.throws(() => Cfonts.text("A").background({ start: "system", end: "blue" }), /except system and candy/);
});

test("the background can only be set once", () => {
	const banner = Cfonts.text("A").background(Color.Blue);
	assert.throws(() => banner.background(Color.Red), /`background\(\)` has already been set/);
	assert.throws(() => banner.background({ preset: GradientPreset.Pride }), /has already been set/);

	const failed = Cfonts.text("A");
	assert.throws(() => failed.background("reed"), Error);
	failed.background(Color.Blue); // a failed call leaves the slot available
});

test("block colors and the background have slots of their own beside the global colors", () => {
	const shapes = [
		[Color.Red],
		{ preset: GradientPreset.Pride },
		{ start: "red", end: "blue" },
		{ transition: ["red", "blue"] },
	];

	for (const shape of shapes) {
		Cfonts.text("A")
			.globalColors({ preset: GradientPreset.Pride })
			.colors(shape)
			.background(Color.Blue)
			.independentGradient();
		Cfonts.text("A").colors(shape).background(Color.Blue).globalColors([Color.Red]);
	}
});

test("independentGradient restarts every line and can only be set once", () => {
	const context = { color: ColorLevel.TrueColor };
	const banner = Cfonts.text("A|AB").font(Font.Tiny).lineHeight(0).colors({ start: "red", end: "blue" });
	const fixed = banner.renderWith(CliEnv, context).text;
	const independent = banner.independentGradient().renderWith(CliEnv, context).text;

	assert.notEqual(independent, fixed);
	for (const row of independent.split("\n").filter((row) => row.length > 0)) {
		const last = row.slice(row.lastIndexOf("\u001b[38;2;"));
		assert.ok(last.startsWith("\u001b[38;2;0;0;255m"), row); // every line reaches the end stop
	}

	assert.throws(() => banner.independentGradient(), /`independentGradient\(\)` has already been set/);
});

test("renderWith paints with an explicit color level", () => {
	const cli = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(CliEnv, {
		color: ColorLevel.TrueColor,
	}).text;
	assert.ok(cli.includes("\u001b[31m"));

	const browser = Cfonts.text("A").font(Font.Tiny).colors(["#ff8800"]).renderWith(BrowserEnv, {
		color: ColorLevel.TrueColor,
	}).text;
	assert.ok(browser.includes('<span style="color:#f80">'));
});

test("colors paint through the node host", () => {
	const rendered = withEnv("FORCE_COLOR", "3", () =>
		withTerminal(
			80,
			undefined,
			() => Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).render(new NodeHost(), CliEnv).text,
		),
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
					NodeHost.fromOverrides({ canvasWidth: 0, color: false }).render(colorBanner(), CliEnv),
				);
				assert.equal(rendered.text, reference(expected), `FORCE_COLOR=${JSON.stringify(forced)}`);
			}

			// NO_COLOR and the API override resolve without detection
			const noColor = withColorEnv(undefined, "1", () =>
				NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
			);
			assert.equal(noColor.text, reference(undefined));

			const overridden = withColorEnv(undefined, undefined, () =>
				NodeHost.fromOverrides({ canvasWidth: 0, color: ColorLevel.Basic }).render(colorBanner(), CliEnv),
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
			const empty = withColorEnv(undefined, "", () =>
				NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
			);
			assert.equal(empty.text, reference(ColorLevel.Ansi256));

			// any non-empty value counts, zero included
			const zero = withColorEnv(undefined, "0", () =>
				NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
			);
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
		// a terminal that refuses escape codes stays plain, the fallback paints only an undetected one
		[{ TERM: "dumb" }, undefined],
	]) {
		const restoreTty = overrideProperty(process.stdout, "isTTY", true);

		try {
			withDetectionEnv(vars, () => {
				const rendered = withColorEnv(undefined, undefined, () =>
					NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
				);
				assert.equal(rendered.text, reference(expected), JSON.stringify(vars));
			});
		} finally {
			restoreTty();
		}
	}
});

test("a dumb terminal carries no ANSI through the Node host", { skip: process.platform === "win32" }, () => {
	const restoreTty = overrideProperty(process.stdout, "isTTY", true);

	try {
		// FORCE_SIZE precedes the width override, so the shell's value is cleared for the comparisons
		withEnv("FORCE_SIZE", undefined, () =>
			withColorEnv(undefined, undefined, () => {
				// the terminal refuses escape codes, so the render fallback never paints it
				const dumb = withDetectionEnv({ TERM: "dumb" }, () =>
					new NodeHost({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
				);
				assert.ok(!dumb.text.includes("\u001b["));
				assert.equal(dumb.text, reference(undefined));

				// an unset or unknown TERM leaves the terminal undetected and the fallback paints it
				for (const vars of [{}, { TERM: "fail" }]) {
					const painted = withDetectionEnv(vars, () => new NodeHost({ canvasWidth: 0 }).render(colorBanner(), CliEnv));
					assert.ok(painted.text.includes("\u001b[38;2;"), JSON.stringify(vars));
					assert.equal(painted.text, reference(ColorLevel.TrueColor), JSON.stringify(vars));
				}
			}),
		);
	} finally {
		restoreTty();
	}
});

/**
 * A lookup over the given variables, the function member of the terminal facts
 */
function lookup(variables) {
	return (name) => (Object.hasOwn(variables, name) ? variables[name] : undefined);
}

/**
 * The color banner rendered by the raw Node host under the given terminal facts, the stream unlimited
 * so only the color decision shows
 */
function rawRender(terminal, overrides = {}) {
	const banner = WasmCfonts.text("AB");
	banner.font(Font.Tiny);
	banner.colors(["#ff8800"]);

	return WasmNodeHost.fromOverrides({ canvasWidth: 0, ...overrides }).render(banner, EnvironmentKind.Cli, false, {
		stdoutColumns: 80,
		attached: true,
		platform: "darwin",
		release: "25.6.0",
		environment: lookup({}),
		...terminal,
	}).text;
}

test("the chain crosses the boundary with the terminal facts", () => {
	// FORCE_COLOR wins over everything the cascade would say
	assert.equal(
		rawRender({ environment: lookup({ TERM: "xterm-256color", FORCE_COLOR: "3" }) }),
		reference(ColorLevel.TrueColor),
	);

	// NO_COLOR silences an otherwise colorful terminal
	assert.equal(rawRender({ environment: lookup({ TERM: "xterm-256color", NO_COLOR: "1" }) }), reference(undefined));

	// an empty NO_COLOR is not set: the cascade answers
	assert.equal(
		rawRender({ environment: lookup({ TERM: "xterm-256color", NO_COLOR: "" }) }),
		reference(ColorLevel.Ansi256),
	);

	// a terminal the facts describe as detached has no terminal to ask and falls back to full color
	assert.equal(rawRender({ attached: false, environment: lookup({ TERM: "ansi" }) }), reference(ColorLevel.TrueColor));
	assert.equal(rawRender({ environment: lookup({ TERM: "ansi" }) }), reference(ColorLevel.Basic));

	// a terminal that refuses escape codes takes no fallback
	assert.equal(rawRender({ environment: lookup({ TERM: "dumb" }) }), reference(undefined));

	// an answer that is no string reads as absent, NO_COLOR answered as a number silences nothing
	assert.equal(
		rawRender({ environment: (name) => (name === "NO_COLOR" ? 1 : undefined) }),
		reference(ColorLevel.TrueColor),
	);

	// the facts are the package's own, a malformed object is refused at the boundary with the sentence of its type,
	// a lookup that is no function with the sentence of the facts
	const banner = WasmCfonts.text("A");
	assert.throws(() => WasmNodeHost.fromOverrides().render(banner, EnvironmentKind.Cli, false, []), {
		name: "TypeError",
		message: "`render()` expects the terminal facts",
	});
	assert.throws(() => WasmNodeHost.fromOverrides().render(banner, EnvironmentKind.Cli, false, {}), {
		name: "TypeError",
		message: "`render()` expects a boolean",
	});
	for (const environment of [undefined, "TERM", ["TERM"], {}]) {
		assert.throws(
			() =>
				WasmNodeHost.fromOverrides().render(banner, EnvironmentKind.Cli, false, {
					attached: true,
					platform: "darwin",
					release: "25.6.0",
					environment,
				}),
			{ name: "TypeError", message: "`render()` expects the terminal facts" },
			`environment ${JSON.stringify(environment)}`,
		);
	}
});

test("the boundary answers the windows console by the build of the release", () => {
	const windows = (release, terminal = {}) => rawRender({ platform: "win32", release, ...terminal });

	assert.equal(windows("10.0.22631"), reference(ColorLevel.TrueColor));
	assert.equal(windows("10.0.10586"), reference(ColorLevel.Ansi256));
	assert.equal(windows("6.3.9600"), reference(ColorLevel.Basic));

	// a detached stream still paints the render fallback, a disabled override never paints
	assert.equal(windows("10.0.22631", { attached: false }), reference(ColorLevel.TrueColor));
	assert.equal(rawRender({ platform: "win32", release: "10.0.22631" }, { color: false }), reference(undefined));
});

test("piped output has no terminal to ask and falls back to full color", () => {
	const restoreTty = overrideProperty(process.stdout, "isTTY", false);

	try {
		const rendered = withColorEnv(undefined, undefined, () =>
			NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv),
		);
		assert.equal(rendered.text, colorBanner().renderWith(CliEnv, { color: ColorLevel.TrueColor }).text);
	} finally {
		restoreTty();
	}
});

/**
 * Runs the operation with `process.env` replaced by the given object, the process environment restored after
 */
function withProcessEnv(replacement, operation) {
	const original = process.env;
	process.env = replacement;

	try {
		return operation();
	} finally {
		process.env = original;
	}
}

test("a variable is read through the runtime's own lookup on process.env", () => {
	// a store that answers case insensitively, the way a Windows main thread answers, on every platform:
	// only a lowercase no_color is set, so NO_COLOR silences the render only when the read is process.env's own,
	// and the color override keeps the cascade out so the test holds on every platform
	const store = { no_color: "1" };
	const caseless = new Proxy(store, {
		get(target, key) {
			if (typeof key !== "string") {
				return Reflect.get(target, key);
			}
			const match = Object.keys(target).find((candidate) => candidate.toLowerCase() === key.toLowerCase());
			return match === undefined ? undefined : target[match];
		},
	});
	const host = NodeHost.fromOverrides({ canvasWidth: 0, color: ColorLevel.Basic });

	// NO_COLOR precedes the override, so the caseless read silences the render
	assert.equal(
		withProcessEnv(caseless, () => host.render(colorBanner(), CliEnv).text),
		reference(undefined),
	);
	// the same store read exactly leaves no_color unread and the override paints
	assert.equal(
		withProcessEnv(store, () => host.render(colorBanner(), CliEnv).text),
		reference(ColorLevel.Basic),
	);
});

test("a lowercase no_color silences the render on a windows main thread alone, the runtime's rule", () => {
	// the package job runs on Linux, so the win32 branch runs only on a Windows machine
	const restoreTty = overrideProperty(process.stdout, "isTTY", true);

	try {
		withDetectionEnv({ TERM: "xterm-256color" }, () =>
			withColorEnv(undefined, undefined, () => {
				const render = () => NodeHost.fromOverrides({ canvasWidth: 0 }).render(colorBanner(), CliEnv).text;
				const painted = render();
				assert.notEqual(painted, reference(undefined));

				const lowercase = withEnv("no_color", "1", render);
				if (process.platform === "win32") {
					assert.equal(lowercase, reference(undefined));
				} else {
					assert.equal(lowercase, painted);
				}
			}),
		);
	} finally {
		restoreTty();
	}
});

test("an exception thrown while a variable is read crosses back as itself once and leaves the host usable", () => {
	// FORCE_SIZE precedes the width override, so the shell's value is cleared for the comparisons
	withEnv("FORCE_SIZE", undefined, () => {
		const thrown = new RangeError("the consumer's own");
		let reads = 0;
		const throwing = new Proxy(
			{},
			{
				get(target, key) {
					if (typeof key !== "string") {
						return Reflect.get(target, key);
					}
					reads += 1;
					throw thrown;
				},
			},
		);
		const host = NodeHost.fromOverrides({ canvasWidth: 0 });
		const expected = withColorEnv(undefined, undefined, () => host.render(colorBanner(), CliEnv).text);

		// the render throws the very object in place of an artifact resolved over facts it failed to read,
		// and asks for no variable after the first failure
		assert.throws(
			() => withProcessEnv(throwing, () => host.render(colorBanner(), CliEnv)),
			(error) => error === thrown,
		);
		assert.equal(reads, 1);

		// the same host renders again, and a variable set between two of its renders is seen
		assert.equal(
			withColorEnv(undefined, undefined, () => host.render(colorBanner(), CliEnv).text),
			expected,
		);
		assert.equal(
			withColorEnv(undefined, "1", () => host.render(colorBanner(), CliEnv).text),
			reference(undefined),
		);
	});
});

test("console styles pair with their markers through renderWith", () => {
	const unstyled = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(BrowserConsoleEnv);
	assert.ok(!unstyled.text.includes("%c"));
	assert.deepEqual(unstyled.styles, []);

	const styled = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]).renderWith(BrowserConsoleEnv, {
		color: ColorLevel.TrueColor,
	});
	assert.equal(styled.text.match(/%c/g).length, styled.styles.length);
	assert.ok(styled.styles.includes("color:#ea3223"));
	assert.ok(styled.styles.includes(""));
});

test("a kept seed repeats candy through the node host and another seed draws differently", () => {
	// FORCE_COLOR and NO_COLOR precede the override, so the shell's values are cleared for the painted comparison
	withColorEnv(undefined, undefined, () => {
		const seed = NodeHost.entropy();
		assert.ok(Number.isInteger(seed) && seed >= 0 && seed <= 0xffff_ffff); // the boundary's seed type

		const party = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]);
		const pinned = NodeHost.fromOverrides({ color: ColorLevel.TrueColor, seed });
		assert.equal(party.render(pinned, CliEnv).text, party.render(pinned, CliEnv).text);

		// the pair is fixed because the core's roll is deterministic per seed: 42 and 43 draw different assortments
		assert.notEqual(
			party.render(NodeHost.fromOverrides({ color: ColorLevel.TrueColor, seed: 42 }), CliEnv).text,
			party.render(NodeHost.fromOverrides({ color: ColorLevel.TrueColor, seed: 43 }), CliEnv).text,
		);
	});
});

test("the node host rolls a fresh seed per render", () => {
	withColorEnv(undefined, undefined, () => {
		assertFresh(() => NodeHost.entropy());

		const party = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]);
		assertFresh(() => party.render(NodeHost.fromOverrides({ color: ColorLevel.TrueColor }), CliEnv).text);
	});
});

test("the browser host decides behind the boundary and writes to the console", () => {
	const banner = Cfonts.text("A").font(Font.Tiny).colors([Color.Red]);

	// a page paints in true color unless told otherwise, and only a column count wraps it
	assert.ok(banner.render(new BrowserHost(), BrowserEnv).text.includes('<span style="color:#ea3223">'));
	assert.ok(!banner.render(BrowserHost.fromOverrides({ color: false }), BrowserEnv).text.includes("<span"));
	assert.notEqual(
		wrappingBanner().render(BrowserHost.fromOverrides({ canvasWidth: 3 }), BrowserEnv).text,
		wrappingBanner().render(new BrowserHost(), BrowserEnv).text,
	);
	assert.equal(
		wrappingBanner().render(new BrowserHost({ canvasWidth: 3 }), BrowserEnv).text,
		wrappingBanner().render(BrowserHost.fromOverrides({ canvasWidth: 3 }), BrowserEnv).text,
	);

	// say spreads the text and the styles into console.log, once
	const host = new BrowserHost();
	const expected = banner.render(host, BrowserConsoleEnv);
	const calls = captureConsoleLogs(() => banner.say(host, BrowserConsoleEnv));
	assert.deepEqual(calls, [[expected.text, ...expected.styles]]);
	assert.ok(expected.styles.length > 0);

	// every render rolls its own candy unless a seed is pinned, and the pinned pair is fixed because
	// the core's roll is deterministic per seed: 42 and 43 draw different assortments
	const party = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]);
	assertFresh(() => party.render(new BrowserHost(), BrowserEnv).text);
	const pinned = BrowserHost.fromOverrides({ seed: BrowserHost.entropy() });
	assert.equal(party.render(pinned, BrowserEnv).text, party.render(pinned, BrowserEnv).text);
	assert.notEqual(
		party.render(BrowserHost.fromOverrides({ seed: 42 }), BrowserEnv).text,
		party.render(BrowserHost.fromOverrides({ seed: 43 }), BrowserEnv).text,
	);

	assert.throws(() => banner.render(new BrowserHost(), {}), {
		name: "TypeError",
		message: "`render()` expects a cfonts environment",
	});
	assert.throws(() => banner.say(new BrowserHost(), {}), {
		name: "TypeError",
		message: "`say()` expects a cfonts environment",
	});
});

test("the package hosts refuse a stray environment in the same words", () => {
	const banner = Cfonts.text("A").font(Font.Tiny);

	for (const host of [new NodeHost(), new BrowserHost()]) {
		assert.throws(() => banner.render(host, {}), {
			name: "TypeError",
			message: "`render()` expects a cfonts environment",
		});
		assert.throws(() => banner.say(host, {}), { name: "TypeError", message: "`say()` expects a cfonts environment" });
	}
	assert.throws(() => banner.renderWith({}), {
		name: "TypeError",
		message: "`renderWith()` expects a cfonts environment",
	});
});

test("candy seeds are deterministic through renderWith", () => {
	const seeded = { color: ColorLevel.TrueColor, seed: 42 };

	const one = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, seeded).text;
	const two = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, seeded).text;
	const other = Cfonts.text("AB").font(Font.Tiny).colors([Color.Candy]).renderWith(CliEnv, {
		color: ColorLevel.TrueColor,
		seed: 43,
	}).text;

	assert.equal(one, two);
	assert.notEqual(one, other);
	assert.ok(one.includes("\u001b["));

	const consoleArtifact = Cfonts.text("A").font(Font.Tiny).colors([Color.Candy]).renderWith(BrowserConsoleEnv, seeded);
	assert.ok(consoleArtifact.styles.length > 0);
});

test("gradients paint through renderWith with a color level", () => {
	const context = { color: ColorLevel.TrueColor };
	const ramped = Cfonts.text("A")
		.font(Font.Tiny)
		.colors({ start: "red", end: "blue" })
		.renderWith(CliEnv, context).text;
	assert.ok(ramped.includes("\u001b[38;2;255;0;0m"));

	// every gradient shape of a block paints the text and never a background
	for (const shape of [
		{ preset: GradientPreset.Pride },
		{ start: "red", end: "blue" },
		{ transition: ["red", "blue"] },
	]) {
		const text = Cfonts.text("A").font(Font.Tiny).colors(shape).renderWith(CliEnv, context).text;
		assert.ok(text.includes("\u001b[38;2;"), `${JSON.stringify(shape)} paints a ramp`);
		assert.ok(!text.includes("\u001b[48;2;"), `${JSON.stringify(shape)} paints no background`);
	}

	const globalRamp = Cfonts.text("A")
		.font(Font.Tiny)
		.globalColors({ preset: GradientPreset.Pride })
		.renderWith(BrowserConsoleEnv, context);
	assert.equal(globalRamp.text.match(/%c/g).length, globalRamp.styles.length);
	assert.ok(globalRamp.styles.includes("color:#750787"));
});
