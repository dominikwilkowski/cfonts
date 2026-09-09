import { Color, GradientPreset, hexToRgb as wasmHexToRgb } from "../pkg/cfonts_wasm.js";
import { expectEnum, expectString, expectU8 } from "./validation.js";

/**
 * An RGB color as channel values
 */
export interface RgbInput {
	red: number;
	green: number;
	blue: number;
}

/**
 * The color of one font color slot: a named `Color`, a hex value, or channel values
 */
export type ColorSlotInput = Color | string | RgbInput;

/**
 * The named colors a gradient stop accepts
 *
 * System has no color to blend and Candy rolls per segment,
 * hex values and channel values cover any other color
 */
export type GradientColor = Exclude<Color, Color.System | Color.Candy>;

/**
 * One gradient stop: a named color, a stop name such as `"red"`, a hex value, or channel values
 */
export type GradientStopInput = GradientColor | string | RgbInput;

/**
 * Two or more gradient stops, with the minimum count part of the type
 */
export type GradientStops = readonly [GradientStopInput, GradientStopInput, ...GradientStopInput[]];

/**
 * A gradient: a preset, two stops, or a transition across two or more stops
 *
 * A preset goes in its object form, `{ preset: GradientPreset.Pride }`,
 * a bare enum value is a number and would read as a Color
 *
 * The `never` members make the shapes mutually exclusive at the type level;
 * the runtime shape check covers plain JavaScript
 */
export type GradientInput =
	| { preset: GradientPreset; start?: never; end?: never; transition?: never }
	| { start: GradientStopInput; end: GradientStopInput; preset?: never; transition?: never }
	| { transition: GradientStops; preset?: never; start?: never; end?: never };

/**
 * The colors of a text block or of the whole composition: one color per font color slot, or a gradient
 */
export type ColorInput = readonly ColorSlotInput[] | GradientInput;

/**
 * The colors shapes after validation, one color list or one gradient shape
 */
export type NormalizedColors = { kind: "list"; colors: string[] } | NormalizedGradient;

/**
 * The gradient shapes after validation, each mapping to one boundary call
 */
export type NormalizedGradient =
	| { kind: "preset"; preset: GradientPreset }
	| { kind: "twoStop"; start: string; end: string }
	| { kind: "transition"; stops: string[] };

/**
 * The named colors a background accepts
 *
 * Candy rolls per segment and cannot fill a row, System paints nothing
 */
export type BackgroundColor = Exclude<Color, Color.Candy>;

/**
 * A background: one color behind every row, or a gradient from the top row down
 */
export type BackgroundInput =
	| BackgroundColor
	| string
	| (RgbInput & { preset?: never; start?: never; end?: never; transition?: never })
	| GradientInput;

/**
 * The background shapes after validation, one color or one gradient shape
 */
export type NormalizedBackground = { kind: "color"; color: string } | NormalizedGradient;

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
 * Cfonts.text("hello").colors({ start: hexToRgb("#ff8800"), end: Color.Blue });
 */
export function hexToRgb(hex: string): RgbInput {
	const [red, green, blue] = wasmHexToRgb(expectString(hex, "hexToRgb"));

	return Object.freeze({ red, green, blue });
}

/**
 * Routes one color-like input by shape and encodes it for the boundary
 */
function normalizeColorLike(
	input: ColorSlotInput | GradientStopInput,
	method: string,
	shapeError: (method: string) => TypeError,
): string {
	if (typeof input === "number") {
		return Color[expectEnum<Color>(input, Color, method)];
	}

	if (typeof input === "string") {
		return input;
	}

	if (input === null || typeof input !== "object") {
		throw shapeError(method);
	}

	return encodeRgb(input, method);
}

/**
 * The teaching error for a color input of the wrong shape
 */
function colorShapeError(method: string): TypeError {
	return new TypeError(
		`\`${method}()\` expects colors as Color values, names, hex values, or {red, green, blue} channels`,
	);
}

/**
 * Validates one color's shape and encodes it for the boundary
 */
export function normalizeColor(input: ColorSlotInput, method: string): string {
	return normalizeColorLike(input, method, colorShapeError);
}

/**
 * Validates a colors input's shape and picks the boundary call it maps to
 *
 * An array is one color per slot, any gradient shape is a gradient
 */
export function normalizeColors(input: ColorInput, method: string): NormalizedColors {
	if (Array.isArray(input)) {
		return { kind: "list", colors: input.map((color: ColorSlotInput) => normalizeColor(color, method)) };
	}

	// Array.isArray narrows mutable arrays only, so the readonly list has to be stated out of the union here
	return normalizeGradient(input as GradientInput, method, colorsShapeError);
}

/**
 * Validates one gradient stop's shape and encodes it for the boundary
 *
 * Enum selections travel as their names and channel values as hex;
 * which colors may participate in a gradient is decided once, in Rust
 */
export function normalizeStop(input: GradientStopInput, method: string): string {
	return normalizeColorLike(input, method, stopColorError);
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
		`\`${method}()\` gradient stops take any Color but Color.System and Color.Candy, a stop name such as "red", ` +
			`a hex value such as "#ff8800", or {red, green, blue} channels from hexToRgb()`,
	);
}

function backgroundShapeError(method: string): TypeError {
	return new TypeError(
		`\`${method}()\` expects a background as a Color value, a name, a hex value, {red, green, blue} channels, ` +
			`or a gradient shape such as {start: Color.Red, end: Color.Blue} or {preset: GradientPreset.Pride}`,
	);
}

function colorsShapeError(method: string): TypeError {
	return new TypeError(
		`\`${method}()\` expects an array of colors such as [Color.Red, "#8899dd"], ` +
			`or exactly one gradient shape such as {start: Color.Red, end: Color.Blue}, ` +
			`{transition: [Color.Red, "#8899dd", Color.Blue]} or {preset: GradientPreset.Pride}`,
	);
}

/**
 * Validates a gradient's shape and picks the boundary call it maps to
 *
 * The shape error names the shapes the calling method accepts
 */
export function normalizeGradient(
	input: GradientInput,
	method: string,
	shapeError: (method: string) => TypeError,
): NormalizedGradient {
	if (input === null || typeof input !== "object") {
		throw shapeError(method);
	}

	// a member left undefined is no shape, as the types say
	const shapes = [
		input.preset !== undefined,
		input.start !== undefined || input.end !== undefined,
		input.transition !== undefined,
	].filter(Boolean).length;

	if (shapes !== 1) {
		throw shapeError(method);
	}

	if (input.preset !== undefined) {
		return { kind: "preset", preset: expectEnum<GradientPreset>(input.preset, GradientPreset, method) };
	}

	if (input.transition !== undefined) {
		if (!Array.isArray(input.transition)) {
			throw new TypeError(
				`\`${method}()\` expects transition stops as an array of two or more colors, ` +
					`such as {transition: [Color.Red, Color.Green, "#0000ff"]}`,
			);
		}

		return { kind: "transition", stops: input.transition.map((stop) => normalizeStop(stop, method)) };
	}

	if (input.start === undefined || input.end === undefined) {
		throw new TypeError(
			`\`${method}()\` expects a gradient with both start and end, such as {start: Color.Red, end: "#8899dd"}`,
		);
	}

	return { kind: "twoStop", start: normalizeStop(input.start, method), end: normalizeStop(input.end, method) };
}

/**
 * Validates a background's shape and picks the boundary call it maps to
 *
 * A number or a string is one color, channels are one color, any gradient shape is a gradient,
 * which colors may fill a row is decided once, in Rust
 */
export function normalizeBackground(input: BackgroundInput, method: string): NormalizedBackground {
	if (typeof input === "number" || typeof input === "string") {
		return { kind: "color", color: normalizeColorLike(input, method, backgroundShapeError) };
	}

	if (input === null || typeof input !== "object") {
		throw backgroundShapeError(method);
	}

	const channels = "red" in input;
	const gradient =
		input.preset !== undefined ||
		input.start !== undefined ||
		input.end !== undefined ||
		input.transition !== undefined;

	if (channels === gradient) {
		throw backgroundShapeError(method);
	}

	if (channels) {
		return { kind: "color", color: encodeRgb(input, method) };
	}

	return normalizeGradient(input, method, backgroundShapeError);
}
