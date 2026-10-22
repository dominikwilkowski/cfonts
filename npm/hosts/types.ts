import type { Cfonts, Rendered } from "../index.js";
import type { Target } from "../targets/index.js";

/**
 * Answers where a render runs and how its output leaves the program
 *
 * The host answers what its runtime can show and owns the one write, the target
 * formats the artifact, the caller pairs them: every pair is legal
 *
 * Consumers may implement this interface for additional runtimes
 */
export interface Host {
	render(composition: Cfonts, target: Target): Rendered;
	say(composition: Cfonts, target: Target): void;
}
