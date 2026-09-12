import type { Cfonts, Rendered } from "../index.js";

/**
 * Resolves runtime capabilities and owns output side effects
 *
 * Consumers may implement this interface for additional runtimes
 */
export interface Host {
	render(composition: Cfonts): Rendered;
	say(composition: Cfonts): void;
}
