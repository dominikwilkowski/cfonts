// window-size's main export is a load-time snapshot that is undefined in pipes;
// its get() in utils always detects fresh, per render
import windowSize from "window-size/utils.js";

import { ColorLevel, decideColor, decideDetected } from "../../pkg/cfonts_wasm.js";
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
	 * FORCE_SIZE, FORCE_COLOR and NO_COLOR still take precedence over these values
	 *
	 * @example
	 * NodeHost.fromOverrides({ canvasWidth: 40, color: ColorLevel.Basic });
	 *
	 * @example
	 * NodeHost.fromOverrides({ color: false }); // paints nothing
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
		const decision = decideColor(
			process.env.FORCE_COLOR,
			process.env.NO_COLOR !== undefined,
			this.#overrides.color === false,
			this.#overrides.color === false ? undefined : this.#overrides.color,
		);

		if (!decision.detect) {
			return decision.level;
		}

		return decideDetected(this.#detectColorLevel());
	}

	#detectColorLevel(): ColorLevel | undefined {
		// piped output has no terminal to ask: only tty streams carry the classifier
		if (typeof process.stdout.getColorDepth !== "function") {
			return undefined;
		}

		// Node's classifier reads FORCE_COLOR and NO_COLOR from the environment it
		// is given, so the variables this host interprets are removed from the copy:
		// detection stays pure capability, whatever context it runs in
		const { FORCE_COLOR: _force, NO_COLOR: _no, ...environment } = process.env;

		switch (process.stdout.getColorDepth(environment)) {
			case 4:
				return ColorLevel.Basic;
			case 8:
				return ColorLevel.Ansi256;
			case 24:
				return ColorLevel.TrueColor;
			default:
				// depth 1: the terminal reports no color support
				return undefined;
		}
	}
}
