import { release } from "node:os";

import { type ColorLevel, detectCanvasWidth, detectColorSupport } from "../../pkg/cfonts_wasm.js";
import { CliEnv } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import { normalizeRenderOverrides, type RenderContext, type RenderOverrides, randomSeed } from "../render-context.js";
import type { Host } from "./types.js";

/**
 * The Windows build number, which dates the console's palette
 *
 * Node's runtime switches escape processing on at startup, so the build is the
 * only console fact the classifier needs
 */
function windowsBuild(): number | undefined {
	if (process.platform !== "win32") {
		return undefined;
	}

	const build = Number(release().split(".")[2]);
	return Number.isInteger(build) && build >= 0 ? build : 0;
}

/**
 * The environment as parallel name/value arrays, the shape the boundary takes
 */
function environmentEntries(): [names: string[], values: string[]] {
	const names: string[] = [];
	const values: string[] = [];
	for (const [name, value] of Object.entries(process.env)) {
		if (value !== undefined) {
			names.push(name);
			values.push(value);
		}
	}

	return [names, values];
}

/**
 * The measured width of the stream still attached to a terminal: stdout, else
 * stderr — a zero-width stream measures nothing, matching the native probe
 */
function measuredColumns(): number | undefined {
	for (const stream of [process.stdout, process.stderr]) {
		if (typeof stream.columns === "number" && stream.columns > 0) {
			return stream.columns;
		}
	}

	return undefined;
}

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
		const [names, values] = environmentEntries();

		return Object.freeze({
			canvasWidth: this.#resolveCanvasWidth(names, values),
			colorLevel: this.#resolveColorLevel(names, values),
			seed: this.#overrides.seed ?? randomSeed(),
		});
	}

	#resolveCanvasWidth(names: string[], values: string[]): number | undefined {
		// the whole decision lives behind the boundary: FORCE_SIZE, the API
		// override, the measured width and the eighty-column fallback
		return detectCanvasWidth(measuredColumns(), names, values, this.#overrides.canvasWidth);
	}

	#resolveColorLevel(names: string[], values: string[]): ColorLevel | undefined {
		// the chain and the cascade both live behind the boundary: the
		// environment crosses whole, FORCE_COLOR and NO_COLOR included
		return detectColorSupport(
			process.stdout.isTTY === true,
			names,
			values,
			windowsBuild(),
			this.#overrides.color === false,
			this.#overrides.color === false ? undefined : this.#overrides.color,
		);
	}
}
