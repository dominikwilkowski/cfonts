import type { Cfonts, Rendered } from "../index.js";
import { BrowserConsoleEnv, BrowserEnv } from "../environments/index.js";
import {
	contextFromCanvasWidth,
	normalizeRenderOverrides,
	type RenderContext,
	type RenderOverrides,
} from "../render-context.js";
import type { Host } from "./types.js";

/**
 * Renders HTML for pages and writes browser-console artifacts through console.log
 */
export class BrowserHost implements Host {
	#overrides: RenderOverrides = Object.freeze({});

	/**
	 * Creates a browser host with explicit capability overrides
	 */
	static fromOverrides(overrides: RenderOverrides): BrowserHost {
		const host = new BrowserHost();
		host.#overrides = normalizeRenderOverrides(overrides);
		return host;
	}

	render(composition: Cfonts): Rendered {
		const context = this.#resolveContext();

		return composition.renderWith(BrowserEnv, context);
	}

	say(composition: Cfonts): void {
		const context = this.#resolveContext();
		const rendered = composition.renderWith(BrowserConsoleEnv, context);

		console.log(rendered.text);

		// TODO(color): pass rendered.styles after the text argument
	}

	#resolveContext(): RenderContext {
		return contextFromCanvasWidth(this.#overrides.canvasWidth);
	}
}
