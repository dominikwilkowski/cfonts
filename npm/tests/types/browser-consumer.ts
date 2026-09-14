import { BrowserConsoleEnv, BrowserHost, Cfonts, CliEnv, Color, Font, GradientPreset } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = BrowserHost.fromOverrides({ canvasWidth: 80, seed: BrowserHost.entropy() });

const html = banner.render(host);
banner.say(host);

banner
	.colors([Color.RedBright, "#f80"])
	.globalColors({ preset: GradientPreset.Bisexual })
	.background({ start: Color.Red, end: Color.Blue });

const consoleArtifact = banner.renderWith(BrowserConsoleEnv);
const terminal = banner.renderWith(CliEnv.withRawMode(true)); // for a terminal emulator in the page

const text: string = html.text;
console.log(text, consoleArtifact.text, terminal.text);

// @ts-expect-error NodeHost is not exported from the browser entry
import { NodeHost } from "cfonts";
