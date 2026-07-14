import type { Rendered } from "../pkg/cfonts_wasm.js";

export type WidthDetection = () => number | undefined;

/** The render methods say() can pick its output from */
interface Renders {
	renderCli(): Rendered;
	renderBrowserConsole(): Rendered;
}

/**
 * What the active entry point wires up
 *
 * The defaults are browser-safe: no terminal code paths, and say() speaks to the devtools console;
 * node.ts overrides both with window-size detection and the terminal render
 */
export const entry: {
	width: WidthDetection;
	sayRender: (banner: Renders) => Rendered;
} = {
	width: () => undefined,
	sayRender: (banner) => banner.renderBrowserConsole(),
};
