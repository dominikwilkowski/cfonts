import { ColorLevel } from "../pkg/cfonts_wasm.js";

import { expectEnum, expectU32 } from "./validation.js";

/**
 * What a consumer asks of a render: the one request type hosts and `renderWith()` take
 *
 * A host resolves these against its own detection, where the environment variables
 * `FORCE_SIZE`, `FORCE_COLOR` and `NO_COLOR` take precedence over every override
 * `renderWith()` detects nothing, so a value left out is off there: no canvas limit,
 * no color, the zero seed
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

export function normalizeRenderOverrides(overrides: RenderOverrides, method: string): RenderOverrides {
	if (overrides === null || typeof overrides !== "object" || Array.isArray(overrides)) {
		throw new TypeError(`\`${method}()\` expects an overrides object`);
	}

	const canvasWidth = overrides.canvasWidth === undefined ? undefined : expectU32(overrides.canvasWidth, method);
	const color =
		overrides.color === undefined || overrides.color === false
			? overrides.color
			: expectEnum<ColorLevel>(overrides.color, ColorLevel, method);
	const seed = overrides.seed === undefined ? undefined : expectU32(overrides.seed, method);

	return Object.freeze({ canvasWidth, color, seed });
}
