import { EnvironmentKind, type Rendered, type Cfonts as WasmCfonts } from "../../pkg/cfonts_wasm.js";

import type { RenderOverrides } from "../render-context.js";

const environmentKind = Symbol("cfonts.environment");
const rawMode = Symbol("cfonts.rawMode");

/**
 * A built-in environment that converts a composition into one artifact format
 *
 * This is a deliberately closed set: JavaScript cannot implement new environments
 * because all formatting runs inside the WASM. Custom artifact formats belong to
 * the Rust `Environment` trait; custom runtimes implement the open `Host` interface
 */
export interface Environment {
	readonly [environmentKind]: EnvironmentKind;
	readonly [rawMode]: boolean;
}

/**
 * The terminal environment, with the line ending as its one setting
 */
export interface TerminalEnvironment extends Environment {
	/**
	 * The terminal environment for raw mode, where every row ends with `\r\n`
	 *
	 * A terminal in raw mode, the way TUIs set it, needs the carriage return,
	 * and so does a terminal emulator in a page
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(CliEnv.rawMode());
	 *
	 * @example
	 * Cfonts.text("hello").say(host, CliEnv.rawMode());
	 */
	rawMode(): Environment;
}

function defineEnvironment(kind: EnvironmentKind, raw = false): Environment {
	return Object.freeze({
		[environmentKind]: kind,
		[rawMode]: raw,
	});
}

const rawTerminal = defineEnvironment(EnvironmentKind.Cli, true);

/** Formats ANSI-compatible terminal text */
export const CliEnv: TerminalEnvironment = Object.freeze({
	...defineEnvironment(EnvironmentKind.Cli),
	rawMode(): Environment {
		return rawTerminal;
	},
});

/**
 * The line ending a host writes after an artifact, the one the environment ends its rows with
 *
 * Only the terminal ends its artifact, a page or a console ends its own output
 */
export function lineEnd(environment: Environment): string {
	if (environment[environmentKind] !== EnvironmentKind.Cli) {
		return "";
	}

	return environment[rawMode] ? "\r\n" : "\n";
}

/** Formats a self-contained HTML fragment */
export const BrowserEnv = defineEnvironment(EnvironmentKind.Browser);

/**
 * Formats a browser-console artifact whose `%c` markers pair with the
 * rendered styles, ready to spread into `console.log`
 */
export const BrowserConsoleEnv = defineEnvironment(EnvironmentKind.BrowserConsole);

/**
 * Renders through the boundary with every override pinned: an override left out is off,
 * no canvas limit, no color, the zero seed
 */
export function renderEnvironment(builder: WasmCfonts, environment: Environment, overrides: RenderOverrides): Rendered {
	const kind = environment?.[environmentKind];

	if (typeof kind !== "number" || !Object.hasOwn(EnvironmentKind, kind)) {
		throw new TypeError("`renderWith()` expects a cfonts environment");
	}

	return builder.render(
		kind,
		overrides.canvasWidth,
		overrides.color === false ? undefined : overrides.color,
		overrides.seed,
		environment[rawMode] === true,
	);
}
