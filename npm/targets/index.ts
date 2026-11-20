import { TargetKind } from "../../pkg/cfonts_wasm.js";

const targetKind = Symbol("cfonts.target");
const rawMode = Symbol("cfonts.rawMode");

/**
 * A built-in target that converts a composition into one artifact format
 *
 * This is a deliberately closed set: JavaScript cannot implement new targets
 * because all formatting runs inside the WASM. Custom artifact formats belong to
 * the Rust `Target` trait; custom runtimes implement the open `Host` interface
 */
export interface Target {
	readonly [targetKind]: TargetKind;
	readonly [rawMode]: boolean;
}

/**
 * The terminal target, with the line ending as its one setting
 */
export interface TerminalTarget extends Target {
	/**
	 * The terminal target for raw mode, where every row ends with `\r\n`
	 *
	 * A terminal in raw mode, the way TUIs set it, needs the carriage return,
	 * and so does a terminal emulator in a page
	 *
	 * @example
	 * Cfonts.text("hello").renderWith(TerminalTarget.rawMode());
	 *
	 * @example
	 * Cfonts.text("hello").say(host, TerminalTarget.rawMode());
	 */
	rawMode(): Target;
}

function defineTarget(kind: TargetKind, raw = false): Target {
	return Object.freeze({
		[targetKind]: kind,
		[rawMode]: raw,
	});
}

const rawTerminal = defineTarget(TargetKind.Terminal, true);

/** Formats ANSI-compatible terminal text */
export const TerminalTarget: TerminalTarget = Object.freeze({
	...defineTarget(TargetKind.Terminal),
	rawMode(): Target {
		return rawTerminal;
	},
});

/** Formats a self-contained HTML fragment */
export const BrowserTarget = defineTarget(TargetKind.Browser);

/**
 * Formats a browser-console artifact whose `%c` markers pair with the
 * rendered styles, ready to spread into `console.log`
 */
export const BrowserConsoleTarget = defineTarget(TargetKind.BrowserConsole);

/**
 * The two values a target crosses the boundary as, its kind and its raw flag
 *
 * The check names the calling method, so `renderWith()` and the package hosts refuse a stray value in the same words
 */
export function targetArguments(target: Target, method: string): [kind: TargetKind, rawMode: boolean] {
	const kind = target?.[targetKind];

	if (typeof kind !== "number" || !Object.hasOwn(TargetKind, kind)) {
		throw new TypeError(`\`${method}()\` expects a cfonts target`);
	}

	return [kind, target[rawMode]];
}
