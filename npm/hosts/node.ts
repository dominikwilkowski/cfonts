import { release } from "node:os";

import { entropy, type RenderOverrides, type Terminal, NodeHost as WasmNodeHost } from "../../pkg/cfonts_wasm.js";
import { inner } from "../boundary.js";
import { type Environment, environmentArguments, lineEnd } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import type { Host } from "./types.js";

/**
 * The Rust Node host behind the boundary: it resolves the terminal width and the color support
 * from the facts of the process and writes artifacts to stdout
 *
 * TypeScript gathers the facts Rust cannot read in Node and keeps the one write, the decisions live in the core
 */
export class NodeHost implements Host {
	readonly #inner: WasmNodeHost;

	/**
	 * Creates the Node host that detects everything from the terminal, or one with explicit capability overrides
	 *
	 * FORCE_SIZE, FORCE_COLOR and NO_COLOR still take precedence over the overrides,
	 * the object crosses the boundary as it is, the core reads it
	 *
	 * @example
	 * new NodeHost();
	 *
	 * @example
	 * new NodeHost({ canvasWidth: 40, color: ColorLevel.Basic });
	 */
	constructor(overrides?: RenderOverrides) {
		this.#inner = WasmNodeHost.fromOverrides(overrides);
	}

	/**
	 * A fresh seed for candy colors, the one a render rolls when no seed override is given
	 *
	 * Hosts and callers that hold a seed of their own take it from here, so the
	 * candy picks differ between runs and hold for as long as the seed is kept
	 *
	 * @example
	 * const seed = NodeHost.entropy();
	 * const host = NodeHost.fromOverrides({ seed });
	 */
	static entropy(): number {
		return entropy();
	}

	/**
	 * Creates a Node host with explicit capability overrides, the constructor with its object
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
		return new NodeHost(overrides);
	}

	render(composition: Cfonts, environment: Environment): Rendered {
		return this.#inner.render(composition[inner](), ...environmentArguments(environment, "render"), this.#terminal());
	}

	say(composition: Cfonts, environment: Environment): void {
		const rendered = this.#inner.render(
			composition[inner](),
			...environmentArguments(environment, "say"),
			this.#terminal(),
		);

		process.stdout.write(`${rendered.text}${lineEnd(environment)}`);
	}

	/**
	 * The facts of the process the resolution reads, gathered at every render so a resized terminal is seen
	 *
	 * A variable is read through `process.env` itself at the moment the resolution asks for it, so the lookup
	 * carries Node's own rule, case insensitive on a Windows main thread and exact elsewhere, and no name list
	 * lives here
	 */
	#terminal(): Terminal {
		return {
			stdoutColumns: process.stdout.columns,
			stderrColumns: process.stderr.columns,
			attached: process.stdout.isTTY === true,
			platform: process.platform,
			release: release(),
			environment: (name) => process.env[name],
		};
	}
}
