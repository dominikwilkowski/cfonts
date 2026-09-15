import { ColorLevel } from "../pkg/cfonts_wasm.js";

import { expectEnum, expectU32 } from "./validation.js";

/** Resolved capabilities for one render */
export interface RenderContext {
	/**
	 * Resolved width in columns
	 *
	 * Undefined and zero mean unlimited
	 */
	readonly canvasWidth?: number;

	/**
	 * Resolved color support
	 *
	 * Undefined paints nothing
	 */
	readonly colorLevel?: ColorLevel;

	/** The seed that makes candy colors reproducible */
	readonly seed?: number;
}

/**
 * Capability requests a host resolves against its own detection
 *
 * The environment variables `FORCE_SIZE`, `FORCE_COLOR` and `NO_COLOR`
 * take precedence over every override
 */
export interface RenderOverrides {
	/**
	 * Width requested by the consumer
	 *
	 * Undefined means automatic detection and zero means unlimited
	 */
	readonly canvasWidth?: number;

	/**
	 * Color support requested by the consumer
	 *
	 * Undefined means automatic detection and false disables colors
	 */
	readonly color?: ColorLevel | false;

	/** Overrides the host's entropy for reproducible candy colors */
	readonly seed?: number;
}

const UNLIMITED_CONTEXT: RenderContext = Object.freeze({});

export function normalizeRenderContext(context?: RenderContext): RenderContext {
	if (context === undefined) {
		return UNLIMITED_CONTEXT;
	}

	if (context === null || typeof context !== "object" || Array.isArray(context)) {
		throw new TypeError("`renderWith()` expects a render context object");
	}

	const canvasWidth =
		context.canvasWidth === undefined || expectU32(context.canvasWidth, "renderWith") === 0
			? undefined
			: context.canvasWidth;
	const colorLevel =
		context.colorLevel === undefined ? undefined : expectEnum<ColorLevel>(context.colorLevel, ColorLevel, "renderWith");
	const seed = context.seed === undefined ? undefined : expectU32(context.seed, "renderWith");

	if (canvasWidth === undefined && colorLevel === undefined && seed === undefined) {
		return UNLIMITED_CONTEXT;
	}

	return Object.freeze({ canvasWidth, colorLevel, seed });
}

export function normalizeRenderOverrides(overrides: RenderOverrides): RenderOverrides {
	if (overrides === null || typeof overrides !== "object" || Array.isArray(overrides)) {
		throw new TypeError("`fromOverrides()` expects an overrides object");
	}

	const canvasWidth =
		overrides.canvasWidth === undefined ? undefined : expectU32(overrides.canvasWidth, "fromOverrides");
	const color =
		overrides.color === undefined || overrides.color === false
			? overrides.color
			: expectEnum<ColorLevel>(overrides.color, ColorLevel, "fromOverrides");
	const seed = overrides.seed === undefined ? undefined : expectU32(overrides.seed, "fromOverrides");

	return Object.freeze({ canvasWidth, color, seed });
}

/** Fresh entropy for candy colors */
export function randomSeed(): number {
	return Math.floor(Math.random() * 0x1_0000_0000);
}
