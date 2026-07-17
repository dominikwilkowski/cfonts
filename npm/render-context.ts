import { expectU32 } from "./validation.js";

export interface RenderContext {
	/**
	 * Resolved width in columns
	 *
	 * Undefined and zero mean unlimited
	 */
	readonly canvasWidth?: number;
}

export interface RenderOverrides {
	/**
	 * Width requested by the consumer
	 *
	 * Undefined means automatic detection and zero means unlimited
	 */
	readonly canvasWidth?: number;
}

const UNLIMITED_CONTEXT: RenderContext = Object.freeze({});

export function normalizeRenderContext(context?: RenderContext): RenderContext {
	if (context === undefined) {
		return UNLIMITED_CONTEXT;
	}

	if (context === null || typeof context !== "object" || Array.isArray(context)) {
		throw new TypeError("`renderWith()` expects a render context object");
	}

	const canvasWidth = context.canvasWidth;

	if (canvasWidth === undefined || expectU32(canvasWidth, "renderWith") === 0) {
		return UNLIMITED_CONTEXT;
	}

	return Object.freeze({ canvasWidth });
}

export function normalizeRenderOverrides(overrides: RenderOverrides): RenderOverrides {
	if (overrides === null || typeof overrides !== "object" || Array.isArray(overrides)) {
		throw new TypeError("`fromOverrides()` expects an overrides object");
	}

	const canvasWidth = overrides.canvasWidth;

	if (canvasWidth === undefined) {
		return Object.freeze({});
	}

	return Object.freeze({
		canvasWidth: expectU32(canvasWidth, "fromOverrides"),
	});
}

export function contextFromCanvasWidth(canvasWidth: number | undefined): RenderContext {
	if (canvasWidth === undefined || canvasWidth === 0) {
		return UNLIMITED_CONTEXT;
	}

	return Object.freeze({ canvasWidth });
}
