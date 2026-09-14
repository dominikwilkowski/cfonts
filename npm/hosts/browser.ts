import { ColorLevel } from "../../pkg/cfonts_wasm.js";
import { BrowserConsoleEnv, BrowserEnv } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import { normalizeRenderOverrides, type RenderContext, type RenderOverrides, randomSeed } from "../render-context.js";
import type { Host } from "./types.js";

/**
 * Renders HTML for pages and writes browser-console artifacts through console.log
 */
export class BrowserHost implements Host {
	#overrides: RenderOverrides = Object.freeze({});

	/**
	 * A fresh seed for candy colors, the one a render rolls when no seed override is given
	 *
	 * Hosts and callers that hold a seed of their own take it from here, so the
	 * candy picks differ between runs and hold for as long as the seed is kept
	 *
	 * @example
	 * const seed = BrowserHost.entropy();
	 * const host = BrowserHost.fromOverrides({ seed });
	 */
	static entropy(): number {
		return randomSeed();
	}

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
