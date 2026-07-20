import type { Cfonts as WasmCfonts, Rendered } from "../../pkg/cfonts_wasm.js";

import type { RenderContext } from "../render-context.js";

const environmentKind = Symbol("cfonts.environment");

const EnvironmentKind = {
	Cli: 0,
	Browser: 1,
	BrowserConsole: 2,
} as const;

type EnvironmentKind = (typeof EnvironmentKind)[keyof typeof EnvironmentKind];

/**
 * A built-in environment that converts a composition into one artifact format
 *
 * This is a deliberately closed set: JavaScript cannot implement new environments
 * because all formatting runs inside the WASM. Custom artifact formats belong to
 * the Rust `Environment` trait; custom runtimes implement the open `Host` interface
 */
export interface Environment {
	readonly [environmentKind]: EnvironmentKind;
}

function defineEnvironment(kind: EnvironmentKind): Environment {
	return Object.freeze({
		[environmentKind]: kind,
	});
}

/** Formats ANSI-compatible terminal text */
export const CliEnv = defineEnvironment(EnvironmentKind.Cli);

/** Formats a self-contained HTML fragment */
export const BrowserEnv = defineEnvironment(EnvironmentKind.Browser);

/**
 * Formats a browser-console artifact
 *
 * TODO(color): add the `%c` format string and style arguments
 */
export const BrowserConsoleEnv = defineEnvironment(EnvironmentKind.BrowserConsole);

export function renderEnvironment(builder: WasmCfonts, environment: Environment, context: RenderContext): Rendered {
	switch (environment?.[environmentKind]) {
		case EnvironmentKind.Cli:
			return builder.renderCli(context.canvasWidth, context.colorLevel, context.seed);

		case EnvironmentKind.Browser:
			return builder.renderBrowser(context.canvasWidth, context.colorLevel, context.seed);

		case EnvironmentKind.BrowserConsole:
			return builder.renderBrowserConsole(context.canvasWidth, context.colorLevel, context.seed);

		default:
			throw new TypeError("`renderWith()` expects a cfonts environment");
	}
}
