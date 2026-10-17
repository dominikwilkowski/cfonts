import { entropy, type RenderOverrides, BrowserHost as WasmBrowserHost } from "../../pkg/cfonts_wasm.js";
import { inner } from "../boundary.js";
import { type Environment, environmentArguments } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";
import type { Host } from "./types.js";

/**
 * The Rust page host behind the boundary: it decides instead of detecting, a page has no
 * terminal to ask, and writes artifacts through the page console
 *
 * TypeScript forwards, the decisions and the console write live in the core
 */
export class BrowserHost implements Host {
	readonly #inner: WasmBrowserHost;

	/**
	 * Creates the page host that decides everything itself, or one with explicit capability overrides
	 *
	 * The overrides object crosses the boundary as it is, the core reads it
	 *
	 * @example
	 * new BrowserHost();
	 *
	 * @example
	 * new BrowserHost({ canvasWidth: 80, seed: 42 });
	 */
	constructor(overrides?: RenderOverrides) {
		this.#inner = WasmBrowserHost.fromOverrides(overrides);
	}

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
		return entropy();
	}

	/**
	 * Creates a browser host with explicit capability overrides, the constructor with its object
	 */
	static fromOverrides(overrides: RenderOverrides): BrowserHost {
		return new BrowserHost(overrides);
	}

	render(composition: Cfonts, environment: Environment): Rendered {
		return this.#inner.render(composition[inner](), ...environmentArguments(environment, "render"));
	}

	say(composition: Cfonts, environment: Environment): void {
		this.#inner.say(composition[inner](), ...environmentArguments(environment, "say"));
	}
}
