import {
	Align,
	Cfonts as WasmCfonts,
	Color,
	ColorLevel,
	Font,
	GradientPreset,
	Valign,
	type Rendered,
} from "../pkg/cfonts_wasm.js";
import {
	normalizeColor,
	normalizeGradient,
	type ColorInput,
	type GradientInput,
	type GradientStopInput,
	type RgbInput,
} from "./color-input.js";
import { BrowserConsoleEnv, BrowserEnv, CliEnv, renderEnvironment, type Environment } from "./environments/index.js";
import type { Host } from "./hosts/types.js";
import { normalizeRenderContext, type RenderContext, type RenderOverrides } from "./render-context.js";
import { expectEnum, expectString, expectU32 } from "./validation.js";

export { Align, BrowserConsoleEnv, BrowserEnv, CliEnv, Color, ColorLevel, Font, GradientPreset, Valign };

export type {
	ColorInput,
	Environment,
	GradientInput,
	GradientStopInput,
	Host,
	RenderContext,
	Rendered,
	RenderOverrides,
	RgbInput,
};

/**
 * A fluent cfonts composition builder
 */
export class Cfonts {
	readonly #inner: WasmCfonts;

	private constructor(inner: WasmCfonts) {
		this.#inner = inner;
	}

	static text(input: string): Cfonts {
		return new Cfonts(WasmCfonts.text(expectString(input, "text")));
	}

	newText(input: string): this {
		this.#inner.newText(expectString(input, "newText"));
		return this;
	}

	font(font: Font): this {
		this.#inner.font(expectEnum<Font>(font, Font, "font"));
		return this;
	}

	letterSpacing(letterSpacing: number): this {
		this.#inner.letterSpacing(expectU32(letterSpacing, "letterSpacing"));
		return this;
	}

	lineHeight(lineHeight: number): this {
		this.#inner.lineHeight(expectU32(lineHeight, "lineHeight"));
		return this;
	}

	/**
	 * Sets the colors for the current text block, one per font color slot
	 */
	colors(colors: ColorInput[]): this {
		if (!Array.isArray(colors)) {
			throw new TypeError("`colors()` expects an array of colors");
		}

		this.#inner.colors(colors.map((color) => normalizeColor(color, "colors")));
		return this;
	}

	/**
	 * Sets a gradient for the current text block
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

	align(align: Align): this {
		this.#inner.align(expectEnum<Align>(align, Align, "align"));
		return this;
	}

	valign(valign: Valign): this {
		this.#inner.valign(expectEnum<Valign>(valign, Valign, "valign"));
		return this;
	}

	maxLength(maxLength: number): this {
		this.#inner.maxLength(expectU32(maxLength, "maxLength"));
		return this;
	}

	/**
	 * Sets the colors across the whole composition, one per font color slot
	 *
	 * Blocks with their own colors override it for their columns;
	 * shares the one global color slot with `globalGradient`
	 */
	globalColors(colors: ColorInput[]): this {
		if (!Array.isArray(colors)) {
			throw new TypeError("`globalColors()` expects an array of colors");
		}

		this.#inner.globalColors(colors.map((color) => normalizeColor(color, "globalColors")));
		return this;
	}

	/**
	 * Sets a gradient across the whole composition
	 *
	 * Blocks with their own colors override it for their columns;
	 * shares the one global color slot with `globalColors`
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

	wordWrap(): this {
		this.#inner.wordWrap();
		return this;
	}

	spaceless(): this {
		this.#inner.spaceless();
		return this;
	}

	/**
	 * Renders through an explicit environment and resolved context
	 *
	 * This does not perform host discovery or output side effects
	 */
	renderWith(environment: Environment, context?: RenderContext): Rendered {
		return renderEnvironment(this.#inner, environment, normalizeRenderContext(context));
	}

	/**
	 * Renders through the supplied host without performing output
	 */
	render(host: Host): Rendered {
		if (host === null || typeof host !== "object" || typeof host.render !== "function") {
			throw new TypeError("`render()` expects a cfonts host");
		}

		return host.render(this);
	}

	/**
	 * Renders and delegates output to the supplied host
	 */
	say(host: Host): void {
		if (host === null || typeof host !== "object" || typeof host.say !== "function") {
			throw new TypeError("`say()` expects a cfonts host");
		}

		host.say(this);
	}
}
