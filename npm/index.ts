import {
	Align,
	alignNames,
	type BackgroundChannels,
	type BackgroundColor,
	type BackgroundOption,
	backgroundColorNames,
	Color,
	ColorLevel,
	type ColorOption,
	colorNames,
	Font,
	fontNames,
	type GradientColor,
	type GradientOption,
	GradientPreset,
	gradientColorNames,
	gradientPresetNames,
	type Preset,
	type Rendered,
	type RenderOverrides,
	rgbFromHex,
	type TextColor,
	type Transition,
	type TransitionStops,
	type TwoStop,
	Valign,
	valignNames,
	Cfonts as WasmCfonts,
	type Rgb as WasmRgb,
} from "../pkg/cfonts_wasm.js";
import { inner } from "./boundary.js";
import { BrowserConsoleEnv, BrowserEnv, CliEnv, type Environment, environmentArguments } from "./environments/index.js";
import type { Host } from "./hosts/types.js";

export type {
	BackgroundChannels,
	BackgroundColor,
	BackgroundOption,
	ColorOption,
	Environment,
	GradientColor,
	GradientOption,
	Host,
	Preset,
	Rendered,
	RenderOverrides,
	TextColor,
	Transition,
	TransitionStops,
	TwoStop,
};
export {
	Align,
	alignNames,
	BrowserConsoleEnv,
	BrowserEnv,
	backgroundColorNames,
	CliEnv,
	Color,
	ColorLevel,
	colorNames,
	Font,
	fontNames,
	GradientPreset,
	gradientColorNames,
	gradientPresetNames,
	Valign,
	valignNames,
};

/**
 * An RGB color as channel values, the shape every color place takes beside a `Color` value and a hex value
 */
export type Rgb = WasmRgb;

/**
 * The channel values of a hex value, `fromHex` parses three or six hex digits with an optional leading `#`
 * where the core parses every hex value and returns a frozen `{ red, green, blue }` object
 *
 * @example
 * Rgb.fromHex("#ff8800"); // { red: 255, green: 136, blue: 0 }
 *
 * @example
 * Cfonts.text("hello").colors({ start: Rgb.fromHex("#ff8800"), end: Color.Blue });
 */
export const Rgb = Object.freeze({ fromHex: rgbFromHex });

/**
 * A fluent cfonts composition builder
 */
export class Cfonts {
	readonly #inner: WasmCfonts;

	private constructor(builder: WasmCfonts) {
		this.#inner = builder;
	}

	/**
	 * The boundary builder behind this composition, for the package's own hosts
	 */
	[inner](): WasmCfonts {
		return this.#inner;
	}

	/**
	 * Starts a composition with its first text block
	 *
	 * The `|` character always breaks a line
	 *
	 * @example
	 * Cfonts.text("hello");
	 *
	 * @example
	 * Cfonts.text("hello|world"); // two lines
	 */
	static text(input: string): Cfonts {
		return new Cfonts(WasmCfonts.text(input));
	}

	/**
	 * Starts the next text block, block settings such as font and colors apply per block
	 *
	 * @example
	 * Cfonts.text("hello ").font(Font.Block).next("world").font(Font.Tiny);
	 */
	next(input: string): this {
		this.#inner.next(input);
		return this;
	}

	/**
	 * Sets the font for the current text block, from the enum or by its command line name
	 *
	 * @example
	 * Cfonts.text("hello").font(Font.Block);
	 *
	 * @example
	 * Cfonts.text("hello").font("3d");
	 */
	font(font: Font | string): this {
		this.#inner.font(font);
		return this;
	}

	/**
	 * Sets the space between letters for the current text block, in glyph columns
	 *
	 * @example
	 * Cfonts.text("hello").letterSpacing(2);
	 */
	letterSpacing(letterSpacing: number): this {
		this.#inner.letterSpacing(letterSpacing);
		return this;
	}

	/**
	 * Sets how many blank rows follow each rendered line of the current text block
	 *
	 * @example
	 * Cfonts.text("hello|world").lineHeight(0); // lines touch
	 */
	lineHeight(lineHeight: number): this {
		this.#inner.lineHeight(lineHeight);
		return this;
	}

	/**
	 * Sets the colors for the current text block: one color per font color slot, or a gradient
	 *
	 * A string is the command line spelling, `"red,blue"` one color per slot, `"red-blue"` a gradient,
	 * `"red:yellow:green"` a transition or a preset name, parsed where the command line parses it
	 *
	 * A gradient ramps one color per column, its stops take any color but system and candy,
	 * hex values, or channel values from `Rgb.fromHex()`
	 *
	 * Any configured value overrides the global colors for this block
	 *
	 * @example
	 * Cfonts.text("hello").colors("red-blue");
	 *
	 * @example
	 * Cfonts.text("hello").font(Font.Block).colors([Color.Red, Color.Blue]);
	 *
	 * @example
	 * Cfonts.text("hello").colors(["#ff8800", { red: 136, green: 153, blue: 221 }]);
	 *
	 * @example
	 * Cfonts.text("party").colors([Color.Candy]); // a fresh pick per painted segment
	 *
	 * @example
	 * Cfonts.text("hello").colors({ start: Color.Red, end: Color.Blue });
	 *
	 * @example
	 * Cfonts.text("hello").colors({ transition: [Color.Red, "#8899dd", Rgb.fromHex("#00ff00")] });
	 *
	 * @example
	 * Cfonts.text("hello").colors({ preset: GradientPreset.Pride });
	 */
	colors(colors: ColorOption): this {
		this.#inner.colors(colors);
		return this;
	}

	/**
	 * Sets the horizontal alignment for the whole composition, from the enum or by its name
	 *
	 * @example
	 * Cfonts.text("hello").align(Align.Center);
	 *
	 * @example
	 * Cfonts.text("hello").align("center");
	 */
	align(align: Align | string): this {
		this.#inner.align(align);
		return this;
	}

	/**
	 * Sets the vertical alignment of fonts with different heights on one line, from the enum or by its name
	 *
	 * @example
	 * Cfonts.text("hello ").font(Font.Block).next("world").font(Font.Tiny).valign(Valign.Bottom);
	 *
	 * @example
	 * Cfonts.text("hello ").font(Font.Block).next("world").font(Font.Tiny).valign("bottom");
	 */
	valign(valign: Valign | string): this {
		this.#inner.valign(valign);
		return this;
	}

	/**
	 * Sets the maximum glyph count per line, zero disables the limit
	 *
	 * @example
	 * Cfonts.text("hello world").maxLength(8);
	 */
	maxLength(maxLength: number): this {
		this.#inner.maxLength(maxLength);
		return this;
	}

	/**
	 * Sets the colors across the whole composition: one color per font color slot, or a gradient
	 *
	 * Blocks with their own colors override it for their columns, a gradient ramps over every
	 * other column and resumes after such a block
	 *
	 * A string is the command line spelling, `"red,blue"` one color per slot, `"red-blue"` a gradient,
	 * `"red:yellow:green"` a transition or a preset name, parsed where the command line parses it
	 *
	 * A gradient's stops take any color but system and candy, hex values, or channel values from `Rgb.fromHex()`
	 *
	 * @example
	 * Cfonts.text("hello ").next("world").globalColors("red,#8899dd");
	 *
	 * @example
	 * Cfonts.text("hello ").next("world").globalColors([Color.Red, "#8899dd"]);
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ start: Color.Red, end: Color.Blue });
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ transition: [Color.Red, Rgb.fromHex("#ff8800"), Color.Yellow] });
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ preset: GradientPreset.Transgender });
	 */
	globalColors(colors: ColorOption): this {
		this.#inner.globalColors(colors);
		return this;
	}

	/**
	 * Restarts every gradient on each line instead of ramping once across every line
	 *
	 * @example
	 * Cfonts.text("hello|world").globalColors({ preset: GradientPreset.Pride }).independentGradient();
	 */
	independentGradient(): this {
		this.#inner.independentGradient();
		return this;
	}

	/**
	 * Paints a background behind every row of the composition, the padding rows included
	 *
	 * One color fills every row, a gradient ramps from the top row down,
	 * `Color.System` paints nothing and `Color.Candy` is refused, it rolls per segment and cannot fill a row
	 *
	 * A string is the command line spelling, one color, `"red-blue"` a gradient, `"red:yellow:green"`
	 * a transition or a preset name, parsed where the command line parses it
	 *
	 * A preset goes in its object form, a bare `GradientPreset` value would read as a `Color`
	 *
	 * @example
	 * Cfonts.text("hello").background("red-blue");
	 *
	 * @example
	 * Cfonts.text("hello").background(Color.Blue);
	 *
	 * @example
	 * Cfonts.text("hello").background({ start: Color.Red, end: "#0000ff" });
	 *
	 * @example
	 * Cfonts.text("hello").background({ preset: GradientPreset.Pride });
	 */
	background(background: BackgroundOption): this {
		this.#inner.background(background);
		return this;
	}

	/**
	 * Enables word-aware wrapping for the current text block
	 *
	 * @example
	 * Cfonts.text("hello world").font(Font.Block).wordWrap();
	 */
	wordWrap(): this {
		this.#inner.wordWrap();
		return this;
	}

	/**
	 * Removes environment-specific outer spacing around the composition
	 *
	 * @example
	 * Cfonts.text("hello").spaceless();
	 */
	spaceless(): this {
		this.#inner.spaceless();
		return this;
	}

	/**
	 * Renders through an explicit environment without a host
	 *
	 * This does not perform host discovery or output side effects: nothing is decided here,
	 * so an override left out is off, no canvas limit, no color, the zero seed
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(CliEnv);
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(BrowserEnv, { color: ColorLevel.TrueColor });
	 */
	renderWith(environment: Environment, overrides?: RenderOverrides): Rendered {
		return this.#inner.render(overrides, ...environmentArguments(environment, "renderWith"));
	}

	/**
	 * Renders through the supplied host into the environment's format without performing output
	 *
	 * The host answers what its runtime can show, the environment formats the artifact
	 *
	 * @example
	 * const rendered = Cfonts.text("hello").render(host, CliEnv);
	 * console.log(rendered.text);
	 */
	render(host: Host, environment: Environment): Rendered {
		if (host === null || typeof host !== "object" || typeof host.render !== "function") {
			throw new TypeError("`render()` expects a cfonts host");
		}

		return host.render(this, environment);
	}

	/**
	 * Renders and delegates output to the supplied host
	 *
	 * @example
	 * Cfonts.text("hello").say(host, CliEnv); // NodeHost writes to stdout
	 *
	 * @example
	 * Cfonts.text("hello").say(host, BrowserConsoleEnv); // BrowserHost writes to the console
	 */
	say(host: Host, environment: Environment): void {
		if (host === null || typeof host !== "object" || typeof host.say !== "function") {
			throw new TypeError("`say()` expects a cfonts host");
		}

		host.say(this, environment);
	}
}
