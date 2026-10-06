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

// the parsed channels are frozen and the declaration says so, the value still goes everywhere an Rgb goes
const rgb: Rgb = Rgb.fromHex("#f80");
banner.colors([rgb]).colors({ start: rgb, end: Color.Blue }).background(rgb);
// @ts-expect-error the channels of Rgb.fromHex are frozen
Rgb.fromHex("#f80").red = 1;

// every single gradient shape and the channel shape compile, two shapes in one object do not
banner.colors({ preset: GradientPreset.Pride }).colors({ start: Color.Red, end: Color.Blue });
banner.colors({ transition: [Color.Red, Color.Blue] }).background({ red: 1, green: 2, blue: 3 });
// @ts-expect-error a preset beside a start and an end is two gradient shapes
banner.colors({ preset: GradientPreset.Pride, start: Color.Red, end: Color.Blue });
// @ts-expect-error a preset beside a transition is two gradient shapes
banner.colors({ transition: [Color.Red, Color.Blue], preset: GradientPreset.Pride });
// @ts-expect-error channels beside a preset are two background shapes
banner.background({ red: 1, green: 2, blue: 3, preset: GradientPreset.Pride });

// a value spelling two shapes is refused like the literal, through the never members alone
const presetAndStops = { preset: GradientPreset.Pride, start: Color.Red, end: Color.Blue } as const;
const channelsAndPreset = { red: 1, green: 2, blue: 3, preset: GradientPreset.Pride } as const;
// @ts-expect-error a preset beside a start and an end is two gradient shapes
banner.colors(presetAndStops);
// @ts-expect-error channels beside a preset are two background shapes
banner.background(channelsAndPreset);

const consoleArtifact = banner.renderWith(BrowserConsoleEnv);
const terminal = banner.renderWith(CliEnv.rawMode()); // for a terminal emulator in the page

const text: string = html.text;
console.log(text, consoleArtifact.text, terminal.text);

// @ts-expect-error NodeHost is not exported from the browser entry
import { NodeHost } from "cfonts";
