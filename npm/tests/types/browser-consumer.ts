import {
	BrowserConsoleEnv,
	BrowserEnv,
	BrowserHost,
	Cfonts,
	CliEnv,
	Color,
	Font,
	fontNames,
	GradientPreset,
	gradientColorNames,
	Rgb,
} from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = BrowserHost.fromOverrides({ canvasWidth: 80, seed: BrowserHost.entropy() });

const html = banner.render(host, BrowserEnv);
banner.say(host, BrowserConsoleEnv);
banner.say(new BrowserHost(), BrowserConsoleEnv);
banner.say(new BrowserHost({ canvasWidth: 80 }), BrowserConsoleEnv);

banner
	.colors([Color.RedBright, "#f80", Rgb.fromHex("#8899dd")])
	.colors("red-blue")
	.globalColors({ preset: GradientPreset.Bisexual })
	.background({ start: Color.Red, end: Color.Blue });

// a page fills its font picker from the names and passes the pick back by name
const picker: string[] = fontNames();
banner.font(picker[0]);

// a page fills a stop picker the same way, every name is a gradient stop
const stops: string[] = gradientColorNames();
banner.colors({ start: stops[0], end: stops[1] });

const consoleArtifact = banner.renderWith(BrowserConsoleEnv);
const terminal = banner.renderWith(CliEnv.rawMode()); // for a terminal emulator in the page

const text: string = html.text;
console.log(text, consoleArtifact.text, terminal.text);

// @ts-expect-error NodeHost is not exported from the browser entry
import { NodeHost } from "cfonts";
