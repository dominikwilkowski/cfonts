import { Color, GradientPreset, hexToRgb as wasmHexToRgb } from "../pkg/cfonts_wasm.js";
import { expectBoolean, expectEnum, expectString, expectU8 } from "./validation.js";

/**
 * An RGB color as channel values
 */
export interface RgbInput {
	red: number;
	green: number;
	blue: number;
}

/**
 * One block color: a named `Color`, a hex value, or channel values
 */
export type ColorInput = Color | string | RgbInput;

/**
 * The base colors a gradient stop accepts
 *
 * System has no color to blend, Candy rolls per segment, and the bright variants
 * have no gradient names; hex values and channel values cover any other color
 */
export type GradientColor =
	| Color.Black
	| Color.Red
	| Color.Green
	| Color.Blue
	| Color.Yellow
	| Color.Magenta
	| Color.Cyan
	| Color.White
	| Color.Gray;

/**
 * One gradient stop: a base color, a stop name such as `"red"`, a hex value, or channel values
 */
export type GradientStopInput = GradientColor | string | RgbInput;

/**
 * Two or more gradient stops, with the minimum count part of the type
 */
export type GradientStops = readonly [GradientStopInput, GradientStopInput, ...GradientStopInput[]];

/**
 * A gradient: a preset, two stops, or a transition across two or more stops
 *
 * The `never` members make the shapes mutually exclusive at the type level;
 * the runtime shape check covers plain JavaScript
 */
export type GradientInput =
	| GradientPreset
	| { preset: GradientPreset; independentGradient?: boolean; start?: never; end?: never; transition?: never }
	| {
			start: GradientStopInput;
			end: GradientStopInput;
			independentGradient?: boolean;
			preset?: never;
			transition?: never;
	  }
	| { transition: GradientStops; independentGradient?: boolean; preset?: never; start?: never; end?: never };

/**
 * The gradient shapes after validation, each mapping to one boundary call
 */
export type NormalizedGradient =
	| { kind: "preset"; preset: GradientPreset; independentGradient: boolean }
	| { kind: "twoStop"; start: string; end: string; independentGradient: boolean }
	| { kind: "transition"; stops: string[]; independentGradient: boolean };

/**
 * Converts a hex value into RGB channel values
 *
 * Accepts three or six hex digits with an optional leading `#`;
 * the parsing itself happens once, in Rust
 * The result plugs into every color and gradient stop input
 *
 * @example
 * hexToRgb("#ff8800"); // { red: 255, green: 136, blue: 0 }
 *
 * @example
 * Cfonts.text("hello").gradient({ start: hexToRgb("#ff8800"), end: Color.Blue });
 */
export function hexToRgb(hex: string): RgbInput {
	const [red, green, blue] = wasmHexToRgb(expectString(hex, "hexToRgb"));

	return Object.freeze({ red, green, blue });
}

/**
 * Validates one color's shape and encodes it for the boundary
 *
 * Enum selections travel as their names and channel values as hex;
 * name and hex validity is checked once, in Rust
 */
export function normalizeColor(input: ColorInput, method: string): string {
	if (typeof input === "number") {
		return Color[expectEnum<Color>(input, Color, method)];
	}

	if (typeof input === "string") {
		return input;
	}

	if (input === null || typeof input !== "object") {
		throw new TypeError(
			`\`${method}()\` expects colors as Color values, names, hex values, or {red, green, blue} channels`,
		);
	}

	return encodeRgb(input, method);
}

/**
 * Validates a color list's shape and encodes each entry for the boundary
 */
export function normalizeColorList(colors: ColorInput[], method: string): string[] {
	if (!Array.isArray(colors)) {
		throw new TypeError(`\`${method}()\` expects an array of colors`);
	}

	return colors.map((color) => normalizeColor(color, method));
}

/**
 * Validates one gradient stop's shape and encodes it for the boundary
 *
 * Enum selections travel as their names and channel values as hex;
 * which colors may participate in a gradient is decided once, in Rust
 */
export function normalizeStop(input: GradientStopInput, method: string): string {
	if (typeof input === "number") {
		return Color[expectEnum<Color>(input, Color, method)];
	}

	if (typeof input === "string") {
		return input;
	}

	if (input === null || typeof input !== "object") {
		throw stopColorError(method);
	}

	return encodeRgb(input, method);
}

/**
 * Validates channel values and encodes them as the boundary's hex spelling
 */
function encodeRgb(input: RgbInput, method: string): string {
	const red = expectU8(input.red, method);
	const green = expectU8(input.green, method);
	const blue = expectU8(input.blue, method);

	return `#${hexByte(red)}${hexByte(green)}${hexByte(blue)}`;
}

function hexByte(value: number): string {
	return value.toString(16).padStart(2, "0");
}

function stopColorError(method: string): TypeError {
	return new TypeError(
		`\`${method}()\` gradient stops take the base colors Color.Black, Color.Red, Color.Green, Color.Blue, ` +
			`Color.Yellow, Color.Magenta, Color.Cyan, Color.White and Color.Gray, a stop name such as "red", ` +
			`a hex value such as "#ff8800", or {red, green, blue} channels from hexToRgb()`,
	);
}

function gradientShapeError(method: string): TypeError {
	return new TypeError(
		`\`${method}()\` expects exactly one gradient shape: {start: Color.Red, end: Color.Blue}, ` +
			`{transition: [Color.Red, "#8899dd", Color.Blue]}, {preset: GradientPreset.Pride}, ` +
			`or the GradientPreset value itself; every object shape also takes independentGradient: true ` +
			`to give each line its own gradient instead of one ramp across the widest line`,
	);
}

/**
 * Validates a gradient's shape and picks the boundary call it maps to
 */
export function normalizeGradient(input: GradientInput, method: string): NormalizedGradient {
	if (typeof input === "number") {
		return {
			kind: "preset",
			preset: expectEnum<GradientPreset>(input, GradientPreset, method),
			independentGradient: false,
		};
	}

	if (input === null || typeof input !== "object") {
		throw gradientShapeError(method);
	}

	const shapes = ["preset" in input, "start" in input || "end" in input, "transition" in input].filter(Boolean).length;

	if (shapes !== 1) {
		throw gradientShapeError(method);
	}

	const independentGradient =
		input.independentGradient === undefined ? false : expectBoolean(input.independentGradient, method);

	if ("preset" in input) {
		return {
			kind: "preset",
			preset: expectEnum<GradientPreset>(input.preset, GradientPreset, method),
			independentGradient,
		};
	}

	if ("transition" in input) {
		if (!Array.isArray(input.transition)) {
			throw new TypeError(
				`\`${method}()\` expects transition stops as an array of two or more colors, ` +
					`such as {transition: [Color.Red, Color.Green, "#0000ff"]}`,
			);
		}

		return {
			kind: "transition",
			stops: input.transition.map((stop) => normalizeStop(stop, method)),
			independentGradient,
		};
	}

	if (!("start" in input) || !("end" in input)) {
		throw new TypeError(
			`\`${method}()\` expects a gradient with both start and end, such as {start: Color.Red, end: "#8899dd"}`,
		);
	}

	return {
		kind: "twoStop",
		start: normalizeStop(input.start, method),
		end: normalizeStop(input.end, method),
		independentGradient,
	};
}
