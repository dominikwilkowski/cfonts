import { ColorLevel } from "../../pkg/cfonts_wasm.js";
import type { Environment } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import { normalizeRenderOverrides, type RenderOverrides, randomSeed } from "../render-context.js";
import type { Host } from "./types.js";

/**
 * Decides instead of detecting, a page has no terminal to ask, and writes artifacts through console.log
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
		host.#overrides = normalizeRenderOverrides(overrides, "fromOverrides");
		return host;
	}

	render(composition: Cfonts, environment: Environment): Rendered {
		return composition.renderWith(environment, this.#resolve());
	}

	say(composition: Cfonts, environment: Environment): void {
		const rendered = composition.renderWith(environment, this.#resolve());

		if (rendered.styles.length > 0) {
			console.log(rendered.text, ...rendered.styles);
		} else {
			console.log(rendered.text);
		}
	}

	/**
	 * The three answers of this host, pinned so the render decides nothing
	 */
	#resolve(): RenderOverrides {
		return Object.freeze({
			// a page has no terminal to measure, so only a column count wraps
			canvasWidth: this.#overrides.canvasWidth,
			// pages always support full color unless told otherwise
			color: this.#overrides.color ?? ColorLevel.TrueColor,
			seed: this.#overrides.seed ?? randomSeed(),
		});
	}
}
