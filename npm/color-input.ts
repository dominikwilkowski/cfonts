import { Color, GradientPreset } from "../pkg/cfonts_wasm.js";
import { expectBoolean, expectEnum, expectU8 } from "./validation.js";

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
 * One gradient stop: a stop name such as `"red"`, a hex value, or channel values
 */
export type GradientStopInput = string | RgbInput;

/**
 * A gradient: a preset, two stops, or a transition across two or more stops
 */
export type GradientInput =
	| GradientPreset
	| { preset: GradientPreset; independentGradient?: boolean }
	| { start: GradientStopInput; end: GradientStopInput; independentGradient?: boolean }
	| { transition: GradientStopInput[]; independentGradient?: boolean };

/**
 * The gradient shapes after validation, each mapping to one boundary call
 */
export type NormalizedGradient =
	| { kind: "preset"; preset: GradientPreset; independentGradient: boolean }
	| { kind: "twoStop"; start: string; end: string; independentGradient: boolean }
	| { kind: "transition"; stops: string[]; independentGradient: boolean };

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

	return normalizeStop(input, method);
}

/**
 * Validates one gradient stop's shape and encodes it for the boundary
 */
export function normalizeStop(input: GradientStopInput, method: string): string {
	if (typeof input === "string") {
		return input;
	}

	if (input === null || typeof input !== "object") {
		throw new TypeError(`\`${method}()\` expects colors as names, hex values, or RGB objects`);
	}

	const red = expectU8(input.red, method);
	const green = expectU8(input.green, method);
	const blue = expectU8(input.blue, method);

	return `#${hexByte(red)}${hexByte(green)}${hexByte(blue)}`;
}

function hexByte(value: number): string {
	return value.toString(16).padStart(2, "0");
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
		throw new TypeError(`\`${method}()\` expects a gradient preset, {start, end}, {transition}, or {preset}`);
	}

	const shapes = ["preset" in input, "start" in input || "end" in input, "transition" in input].filter(Boolean).length;

	if (shapes !== 1) {
		throw new TypeError(`\`${method}()\` expects exactly one gradient shape: {start, end}, {transition}, or {preset}`);
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
			throw new TypeError(`\`${method}()\` expects transition stops as an array`);
		}

		return {
			kind: "transition",
			stops: input.transition.map((stop) => normalizeStop(stop, method)),
			independentGradient,
		};
	}

	if (!("start" in input) || !("end" in input)) {
		throw new TypeError(`\`${method}()\` expects a gradient with both start and end`);
	}

	return {
		kind: "twoStop",
		start: normalizeStop(input.start, method),
		end: normalizeStop(input.end, method),
		independentGradient,
	};
}
