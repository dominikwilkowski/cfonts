import {
	Align,
	Color,
	ColorLevel,
	Font,
	GradientPreset,
	type Rendered,
	Valign,
	Cfonts as WasmCfonts,
} from "../pkg/cfonts_wasm.js";
import {
	type BackgroundColor,
	type BackgroundInput,
	type ColorInput,
	type ColorSlotInput,
	type GradientColor,
	type GradientInput,
	type GradientStopInput,
	type GradientStops,
	hexToRgb,
	normalizeBackground,
	normalizeColors,
	type RgbInput,
} from "./color-input.js";
import { BrowserConsoleEnv, BrowserEnv, CliEnv, type Environment, renderEnvironment } from "./environments/index.js";
import type { Host } from "./hosts/types.js";
import { normalizeRenderContext, type RenderContext, type RenderOverrides } from "./render-context.js";
import { expectEnum, expectString, expectU32 } from "./validation.js";

export type {
	BackgroundColor,
	BackgroundInput,
	ColorInput,
	ColorSlotInput,
	Environment,
	GradientColor,
	GradientInput,
	GradientStopInput,
	GradientStops,
	Host,
	RenderContext,
	Rendered,
	RenderOverrides,
	RgbInput,
};
export { Align, BrowserConsoleEnv, BrowserEnv, CliEnv, Color, ColorLevel, Font, GradientPreset, hexToRgb, Valign };

/**
 * A fluent cfonts composition builder
 */
export class Cfonts {
	readonly #inner: WasmCfonts;

	private constructor(inner: WasmCfonts) {
		this.#inner = inner;
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
		return new Cfonts(WasmCfonts.text(expectString(input, "text")));
	}

	/**
	 * Starts a new text block; block settings such as font and colors apply per block
	 *
	 * @example
	 * Cfonts.text("hello ").font(Font.Block).newText("world").font(Font.Tiny);
	 */
	newText(input: string): this {
		this.#inner.newText(expectString(input, "newText"));
		return this;
	}

	/**
	 * Sets the font for the current text block
	 *
	 * @example
	 * Cfonts.text("hello").font(Font.Block);
	 */
	font(font: Font): this {
		this.#inner.font(expectEnum<Font>(font, Font, "font"));
		return this;
	}

	/**
	 * Sets the space between letters for the current text block, in glyph columns
	 *
	 * @example
	 * Cfonts.text("hello").letterSpacing(2);
	 */
	letterSpacing(letterSpacing: number): this {
		this.#inner.letterSpacing(expectU32(letterSpacing, "letterSpacing"));
		return this;
	}

	/**
	 * Sets how many blank rows follow each rendered line of the current text block
	 *
	 * @example
	 * Cfonts.text("hello|world").lineHeight(0); // lines touch
	 */
	lineHeight(lineHeight: number): this {
		this.#inner.lineHeight(expectU32(lineHeight, "lineHeight"));
		return this;
	}

	/**
	 * Sets the colors for the current text block: one color per font color slot, or a gradient
	 *
	 * A gradient ramps one color per column, its stops take any color but system and candy,
	 * hex values, or channel values from `hexToRgb()`
	 *
	 * Any configured value overrides the global colors for this block
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
	 * Cfonts.text("hello").colors({ transition: [Color.Red, "#8899dd", hexToRgb("#00ff00")] });
	 *
	 * @example
	 * Cfonts.text("hello").colors({ preset: GradientPreset.Pride });
	 */
	colors(colors: ColorInput): this {
		const normalized = normalizeColors(colors, "colors");

		switch (normalized.kind) {
			case "list":
				this.#inner.colors(normalized.colors);
				break;
			case "preset":
				this.#inner.gradientPreset(normalized.preset);
				break;
			case "twoStop":
				this.#inner.gradient(normalized.start, normalized.end);
				break;
			case "transition":
				this.#inner.transition(normalized.stops);
				break;
		}

		return this;
	}

	/**
	 * Sets the horizontal alignment for the whole composition
	 *
	 * @example
	 * Cfonts.text("hello").align(Align.Center);
	 */
	align(align: Align): this {
		this.#inner.align(expectEnum<Align>(align, Align, "align"));
		return this;
	}

	/**
	 * Sets the vertical alignment of fonts with different heights on one line
	 *
	 * @example
	 * Cfonts.text("hello ").font(Font.Block).newText("world").font(Font.Tiny).valign(Valign.Bottom);
	 */
	valign(valign: Valign): this {
		this.#inner.valign(expectEnum<Valign>(valign, Valign, "valign"));
		return this;
	}

	/**
	 * Sets the maximum glyph count per line; zero disables the limit
	 *
	 * @example
	 * Cfonts.text("hello world").maxLength(8);
	 */
	maxLength(maxLength: number): this {
		this.#inner.maxLength(expectU32(maxLength, "maxLength"));
		return this;
	}

	/**
	 * Sets the colors across the whole composition: one color per font color slot, or a gradient
	 *
	 * Blocks with their own colors override it for their columns, a gradient ramps over every
	 * other column and resumes after such a block
	 *
	 * A gradient's stops take any color but system and candy, hex values, or channel values from `hexToRgb()`
	 *
	 * @example
	 * Cfonts.text("hello ").newText("world").globalColors([Color.Red, "#8899dd"]);
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ start: Color.Red, end: Color.Blue });
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ transition: [Color.Red, hexToRgb("#ff8800"), Color.Yellow] });
	 *
	 * @example
	 * Cfonts.text("hello").globalColors({ preset: GradientPreset.Transgender });
	 */
	globalColors(colors: ColorInput): this {
		const normalized = normalizeColors(colors, "globalColors");

		switch (normalized.kind) {
			case "list":
				this.#inner.globalColors(normalized.colors);
				break;
			case "preset":
				this.#inner.globalGradientPreset(normalized.preset);
				break;
			case "twoStop":
				this.#inner.globalGradient(normalized.start, normalized.end);
				break;
			case "transition":
				this.#inner.globalTransition(normalized.stops);
				break;
		}

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
	 * A preset goes in its object form, a bare `GradientPreset` value would read as a `Color`
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
	background(background: BackgroundInput): this {
		const normalized = normalizeBackground(background, "background");

		switch (normalized.kind) {
			case "color":
				this.#inner.background(normalized.color);
				break;
			case "preset":
				this.#inner.backgroundGradientPreset(normalized.preset);
				break;
			case "twoStop":
				this.#inner.backgroundGradient(normalized.start, normalized.end);
				break;
			case "transition":
				this.#inner.backgroundTransition(normalized.stops);
				break;
		}

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
	 * Renders through an explicit environment and resolved context
	 *
	 * This does not perform host discovery or output side effects
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(CliEnv);
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(BrowserEnv, { colorLevel: ColorLevel.TrueColor });
	 */
	renderWith(environment: Environment, context?: RenderContext): Rendered {
		return renderEnvironment(this.#inner, environment, normalizeRenderContext(context));
	}

	/**
	 * Renders through the supplied host without performing output
	 *
	 * @example
	 * const rendered = Cfonts.text("hello").render(host);
	 * console.log(rendered.text);
	 */
	render(host: Host): Rendered {
		if (host === null || typeof host !== "object" || typeof host.render !== "function") {
			throw new TypeError("`render()` expects a cfonts host");
		}

		return host.render(this);
	}

	/**
	 * Renders and delegates output to the supplied host
	 *
	 * @example
	 * Cfonts.text("hello").say(host); // NodeHost writes to stdout, BrowserHost to the console
	 */
	say(host: Host): void {
		if (host === null || typeof host !== "object" || typeof host.say !== "function") {
			throw new TypeError("`say()` expects a cfonts host");
		}

		host.say(this);
	}
}
