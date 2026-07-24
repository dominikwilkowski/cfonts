import { ColorLevel } from "../../pkg/cfonts_wasm.js";
import type { Cfonts, Rendered } from "../index.js";
import { BrowserConsoleEnv, BrowserEnv } from "../environments/index.js";
import { normalizeRenderOverrides, randomSeed, type RenderContext, type RenderOverrides } from "../render-context.js";
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
		return composition.renderWith(BrowserEnv, this.#resolveContext());
	}

	say(composition: Cfonts): void {
		const rendered = composition.renderWith(BrowserConsoleEnv, this.#resolveContext());

		if (rendered.styles.length > 0) {
			console.log(rendered.text, ...rendered.styles);
		} else {
			console.log(rendered.text);
		}
	}

	#resolveContext(): RenderContext {
		const override = this.#overrides.color;

		return Object.freeze({
			canvasWidth: this.#overrides.canvasWidth === 0 ? undefined : this.#overrides.canvasWidth,
			// pages always support full color unless told otherwise
			colorLevel: override === false ? undefined : (override ?? ColorLevel.TrueColor),
			seed: this.#overrides.seed ?? randomSeed(),
		});
	}
}
