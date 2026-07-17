import windowSize from "window-size/utils.js";

import type { Cfonts, Rendered } from "../index.js";
import { CliEnv } from "../environments/index.js";
import {
	contextFromCanvasWidth,
	normalizeRenderOverrides,
	type RenderContext,
	type RenderOverrides,
} from "../render-context.js";
import { parseU32 } from "../validation.js";
import type { Host } from "./types.js";

const FALLBACK_WIDTH = 80;

/**
 * Resolves Node terminal capabilities and writes CLI artifacts to stdout
 */
export class NodeHost implements Host {
	#overrides: RenderOverrides = Object.freeze({});

	/**
	 * Creates a Node host with explicit capability overrides
	 *
	 * FORCE_SIZE still takes precedence over these values
	 */
	static fromOverrides(overrides: RenderOverrides): NodeHost {
		const host = new NodeHost();
		host.#overrides = normalizeRenderOverrides(overrides);
		return host;
	}

	render(composition: Cfonts): Rendered {
		const context = this.#resolveContext();

		return composition.renderWith(CliEnv, context);
	}

	say(composition: Cfonts): void {
		const context = this.#resolveContext();
		const rendered = composition.renderWith(CliEnv, context);

		process.stdout.write(`${rendered.text}\n`);
	}

	#resolveContext(): RenderContext {
		const forced = parseU32(process.env.FORCE_SIZE);

		if (forced !== undefined) {
			return contextFromCanvasWidth(forced);
		}

		if (this.#overrides.canvasWidth !== undefined) {
			return contextFromCanvasWidth(this.#overrides.canvasWidth);
		}

		const detected = windowSize.get()?.width;

		return contextFromCanvasWidth(detected ?? FALLBACK_WIDTH);
	}
}
