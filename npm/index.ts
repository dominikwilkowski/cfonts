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
	type ColorInput,
	type GradientColor,
	type GradientInput,
	type GradientStopInput,
	type GradientStops,
	hexToRgb,
	normalizeColorList,
	normalizeGradient,
	type RgbInput,
} from "./color-input.js";
import { BrowserConsoleEnv, BrowserEnv, CliEnv, type Environment, renderEnvironment } from "./environments/index.js";
import type { Host } from "./hosts/types.js";
import { normalizeRenderContext, type RenderContext, type RenderOverrides } from "./render-context.js";
import { expectEnum, expectString, expectU32 } from "./validation.js";

export type {
	ColorInput,
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
	 * Sets the colors for the current text block, one per font color slot
	 *
	 * Any configured value overrides the global color for this block
	 *
	 * @example
	 * Cfonts.text("hello").font(Font.Block).colors([Color.Red, Color.Blue]);
	 *
	 * @example
	 * Cfonts.text("hello").colors(["#ff8800", { red: 136, green: 153, blue: 221 }]);
	 *
	 * @example
	 * Cfonts.text("party").colors([Color.Candy]); // a fresh pick per painted segment
	 */
	colors(colors: ColorInput[]): this {
		this.#inner.colors(normalizeColorList(colors, "colors"));
		return this;
	}

	/**
	 * Sets a gradient for the current text block, one ramp color per column
	 *
	 * Stops take the base colors, hex values, or channel values from `hexToRgb()`;
	 * `independentGradient: true` gives each line its own gradient instead of
	 * one ramp across the widest line
	 *
	 * @example
	 * Cfonts.text("hello").gradient({ start: Color.Red, end: Color.Blue });
	 *
	 * @example
	 * Cfonts.text("hello").gradient({ transition: [Color.Red, "#8899dd", hexToRgb("#00ff00")] });
	 *
	 * @example
	 * Cfonts.text("hello").gradient(GradientPreset.Pride);
	 *
	 * @example
	 * Cfonts.text("hello|world").gradient({ preset: GradientPreset.Pride, independentGradient: true });
	 */
	gradient(gradient: GradientInput): this {
		const normalized = normalizeGradient(gradient, "gradient");

		switch (normalized.kind) {
			case "preset":
				this.#inner.gradientPreset(normalized.preset, normalized.independentGradient);
				break;
			case "twoStop":
				this.#inner.gradient(normalized.start, normalized.end, normalized.independentGradient);
				break;
			case "transition":
				this.#inner.transition(normalized.stops, normalized.independentGradient);
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
	 * Sets the colors across the whole composition, one per font color slot
	 *
	 * Blocks with their own colors override it for their columns;
	 * shares the one global color slot with `globalGradient`
	 *
	 * @example
	 * Cfonts.text("hello ").newText("world").globalColors([Color.Red, "#8899dd"]);
	 */
	globalColors(colors: ColorInput[]): this {
		this.#inner.globalColors(normalizeColorList(colors, "globalColors"));
		return this;
	}

	/**
	 * Sets a gradient across the whole composition, one ramp color per column
	 *
	 * Blocks with their own colors override it for their columns and the ramp resumes after;
	 * shares the one global color slot with `globalColors`
	 * Stops take the base colors, hex values, or channel values from `hexToRgb()`;
	 * `independentGradient: true` gives each line its own gradient instead of
	 * one ramp across the widest line
	 *
	 * @example
	 * Cfonts.text("hello").globalGradient({ start: Color.Red, end: Color.Blue });
	 *
	 * @example
	 * Cfonts.text("hello").globalGradient({ transition: [Color.Red, hexToRgb("#ff8800"), Color.Yellow] });
	 *
	 * @example
	 * Cfonts.text("hello|world").globalGradient({ preset: GradientPreset.Transgender, independentGradient: true });
	 */
	globalGradient(gradient: GradientInput): this {
		const normalized = normalizeGradient(gradient, "globalGradient");

		switch (normalized.kind) {
			case "preset":
				this.#inner.globalGradientPreset(normalized.preset, normalized.independentGradient);
				break;
			case "twoStop":
				this.#inner.globalGradient(normalized.start, normalized.end, normalized.independentGradient);
				break;
			case "transition":
				this.#inner.globalTransition(normalized.stops, normalized.independentGradient);
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
