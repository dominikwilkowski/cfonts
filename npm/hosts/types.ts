import type { Environment } from "../environments/index.js";
import type { Cfonts, Rendered } from "../index.js";

/**
 * Answers where a render runs and how its output leaves the program
 *
 * The host answers what its runtime can show and owns the one write, the environment
 * formats the artifact, the caller pairs them: every pair is legal
 *
 * Consumers may implement this interface for additional runtimes
 */
export interface Host {
	render(composition: Cfonts, environment: Environment): Rendered;
	say(composition: Cfonts, environment: Environment): void;
}
