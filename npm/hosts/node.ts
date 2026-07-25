import supportsColor from "supports-color";
// window-size's main export is a load-time snapshot that is undefined in pipes;
// its get() in utils always detects fresh, per render
import windowSize from "window-size/utils.js";

import { ColorLevel } from "../../pkg/cfonts_wasm.js";
import { CliEnv } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import { normalizeRenderOverrides, type RenderContext, type RenderOverrides, randomSeed } from "../render-context.js";
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
	 * FORCE_SIZE and FORCE_COLOR still take precedence over these values
	 */
	static fromOverrides(overrides: RenderOverrides): NodeHost {
		const host = new NodeHost();
		host.#overrides = normalizeRenderOverrides(overrides);
		return host;
	}

	render(composition: Cfonts): Rendered {
		return composition.renderWith(CliEnv, this.#resolveContext());
	}

	say(composition: Cfonts): void {
		const rendered = composition.renderWith(CliEnv, this.#resolveContext());

		process.stdout.write(`${rendered.text}\n`);
	}

	#resolveContext(): RenderContext {
		return Object.freeze({
			canvasWidth: this.#resolveCanvasWidth(),
			colorLevel: this.#resolveColorLevel(),
			seed: this.#overrides.seed ?? randomSeed(),
		});
	}

	#resolveCanvasWidth(): number | undefined {
		const forced = parseU32(process.env.FORCE_SIZE);

		if (forced !== undefined) {
			return forced === 0 ? undefined : forced;
		}

		if (this.#overrides.canvasWidth !== undefined) {
			return this.#overrides.canvasWidth === 0 ? undefined : this.#overrides.canvasWidth;
		}

		return windowSize.get()?.width ?? FALLBACK_WIDTH;
	}

	#resolveColorLevel(): ColorLevel | undefined {
		switch (process.env.FORCE_COLOR) {
			case "0":
				return undefined;
			case "1":
				return ColorLevel.Basic;
			case "2":
				return ColorLevel.Ansi256;
			case "3":
				return ColorLevel.TrueColor;
			default:
				// any other value is treated as absent
				break;
		}

		if (process.env.NO_COLOR !== undefined) {
			return undefined;
		}

		const override = this.#overrides.color;

		if (override === false) {
			return undefined;
		}

		if (override !== undefined) {
			return override;
		}

		// terminals that cannot be detected still get full color
		const detected = supportsColor.stdout;

		if (!detected) {
			return ColorLevel.TrueColor;
		}

		switch (detected.level) {
			case 1:
				return ColorLevel.Basic;
			case 2:
				return ColorLevel.Ansi256;
			default:
				return ColorLevel.TrueColor;
		}
	}
}
