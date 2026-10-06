import {
	Align,
	alignNames,
	type BackgroundColor,
	type BackgroundOption,
	BrowserConsoleEnv,
	BrowserEnv,
	backgroundColorNames,
	Cfonts,
	CliEnv,
	Color,
	type ColorOption,
	colorNames,
	Font,
	fontNames,
	type GradientColor,
	type GradientOption,
	GradientPreset,
	gradientColorNames,
	gradientPresetNames,
	NodeHost,
	type Preset,
	Rgb,
	type TextColor,
	type Transition,
	type TransitionStops,
	type TwoStop,
	Valign,
	valignNames,
} from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = NodeHost.fromOverrides({ canvasWidth: 80 });

const rendered = banner.render(host, CliEnv);
banner.say(host, CliEnv);

banner.say(NodeHost.fromOverrides({ seed: NodeHost.entropy() }), CliEnv);
banner.say(new NodeHost(), CliEnv);
banner.say(new NodeHost({ canvasWidth: 80 }), CliEnv);

banner.say(host, CliEnv.rawMode());
const raw = banner.renderWith(CliEnv.rawMode(), { canvasWidth: 80 });
console.log(raw.text);

// @ts-expect-error every render names its environment
banner.render(host);

// @ts-expect-error the browser environments have no raw mode
BrowserEnv.rawMode();

const artifact = banner.renderWith(BrowserConsoleEnv, {
	canvasWidth: 80,
});

// @ts-expect-error the overrides take a color, not a resolved level
banner.renderWith(BrowserConsoleEnv, { colorLevel: 3 });

// the enums and their command line names both pick, the names come from the core
banner.font("3d").align("center").valign("bottom").align(Align.Center).valign(Valign.Bottom);
const names: string[] = [
	...fontNames(),
	...alignNames(),
	...valignNames(),
	...gradientPresetNames(),
	...colorNames(),
	...backgroundColorNames(),
	...gradientColorNames(),
];
console.log(names.length);

const colorful = Cfonts.text("colors")
	.colors([Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }])
	.colors({ preset: GradientPreset.Pride })
	.globalColors({ start: "red", end: "#0000ff" })
	.independentGradient();

colorful.colors({ transition: ["red", { red: 0, green: 0, blue: 255 }, "#00ff00"] });
colorful.colors("red-blue"); // the command line spelling
colorful.colors({ start: Color.Red, end: Rgb.fromHex("#0000ff") });
colorful.colors({ transition: [Color.Red, Color.Gray, Rgb.fromHex("#8899dd")] });

// Rgb is the type of the channels and the value that parses them
const channels: Rgb = Rgb.fromHex("#ff8800");
const structural: { red: number; green: number; blue: number } = channels;
console.log(structural.red);

// the parsed channels are frozen and the declaration says so, the value still goes everywhere an Rgb goes
const rgb: Rgb = Rgb.fromHex("#f80");
colorful.colors([rgb]).colors({ start: rgb, end: Color.Blue }).background(rgb);
// @ts-expect-error the channels of Rgb.fromHex are frozen
Rgb.fromHex("#f80").red = 1;

colorful.colors({ preset: GradientPreset.Lesbian });
colorful.render(host, CliEnv);

// the types carry the Rust names, a consumer holds one of each
const stops: TransitionStops = [Color.Red, "#8899dd", channels];
const transition: Transition = { transition: stops };
const twoStop: TwoStop = { start: Color.RedBright, end: channels };
const preset: Preset = { preset: GradientPreset.Pride };
const gradient: GradientOption = twoStop;
const gradientColor: GradientColor = Color.Yellow;
const textColor: TextColor = Color.Candy;
const option: ColorOption = [textColor, "#ff8800", channels];
const backgroundColor: BackgroundColor = Color.System;
const background: BackgroundOption = gradient;
Cfonts.text("typed")
	.colors(option)
	.colors(transition)
	.colors(preset)
	.colors({ start: gradientColor, end: twoStop.end });
Cfonts.text("typed").globalColors(gradient).background(background);
Cfonts.text("typed").background(backgroundColor);

Cfonts.text("global").globalColors([Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }]);

Cfonts.text("banded").background(Color.Blue);
Cfonts.text("banded").background(Color.System);
Cfonts.text("banded").background("#222");
Cfonts.text("banded").background(Rgb.fromHex("#222222"));
Cfonts.text("banded").background({ start: Color.Red, end: "#0000ff" });
Cfonts.text("banded").background({ transition: [Color.Red, Color.WhiteBright] });
Cfonts.text("banded").background({ preset: GradientPreset.Pride });

// @ts-expect-error Candy rolls per segment and cannot fill a row
Cfonts.text("banded").background(Color.Candy);

// @ts-expect-error a bare preset would read as a Color, presets go in their object form
Cfonts.text("banded").background(GradientPreset.Pride);

// @ts-expect-error an empty object is not a gradient
banner.colors({});

// @ts-expect-error a bare preset is a number, presets go in their object form
banner.colors(GradientPreset.Pride);

// @ts-expect-error one color is still a list
banner.globalColors(Color.Red);

const text: string = rendered.text;
const styles: string[] = artifact.styles;
console.log(text, artifact.text, styles.length);

// @ts-expect-error BrowserHost is not exported from the Node entry
import { BrowserHost } from "cfonts";

// @ts-expect-error the Node entry must not inject DOM globals
document;

// @ts-expect-error System has no color to blend into a gradient
colorful.colors({ start: Color.System, end: Color.Blue });

// @ts-expect-error Candy rolls per segment and has no color to blend into a gradient
colorful.colors({ transition: [Color.Red, Color.Candy] });

// bright colors are stops like any other named color
colorful.colors({ transition: [Color.Red, Color.WhiteBright] });

// @ts-expect-error a transition holds at least two stops
colorful.colors({ transition: [Color.Red] });

// @ts-expect-error the independent flag is a builder setting, not a gradient field
colorful.colors({ start: Color.Red, end: Color.Blue, independentGradient: true });

// every single gradient shape and the channel shape compile, two shapes in one object do not
colorful.colors({ preset: GradientPreset.Pride }).colors({ start: Color.Red, end: Color.Blue });
colorful.colors({ transition: [Color.Red, Color.Blue] }).background({ red: 1, green: 2, blue: 3 });
// @ts-expect-error a preset beside a start and an end is two gradient shapes
colorful.colors({ preset: GradientPreset.Pride, start: Color.Red, end: Color.Blue });
// @ts-expect-error a preset beside a transition is two gradient shapes
colorful.colors({ transition: [Color.Red, Color.Blue], preset: GradientPreset.Pride });
// @ts-expect-error channels beside a preset are two background shapes
Cfonts.text("banded").background({ red: 1, green: 2, blue: 3, preset: GradientPreset.Pride });

// a value spelling two shapes is refused like the literal, through the never members alone
const presetAndStops = { preset: GradientPreset.Pride, start: Color.Red, end: Color.Blue } as const;
const channelsAndPreset = { red: 1, green: 2, blue: 3, preset: GradientPreset.Pride } as const;
// @ts-expect-error a preset beside a start and an end is two gradient shapes
colorful.colors(presetAndStops);
// @ts-expect-error channels beside a preset are two background shapes
Cfonts.text("banded").background(channelsAndPreset);

// a stored shape narrows by its member, every shape declares every key so `in` keeps every shape
const storedGradient: GradientOption = { preset: GradientPreset.Pride };
if (storedGradient.preset !== undefined) {
	const narrowed: GradientPreset = storedGradient.preset;
	console.log(narrowed);
}

// readonly color lists are accepted: the methods only read them
const readonlyColors = [Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }] as const;
Cfonts.text("frozen").colors(readonlyColors).globalColors(readonlyColors);

const readonlyTyped: readonly (Color | string)[] = [Color.Red, "#8899dd"];
Cfonts.text("frozen").colors(readonlyTyped);
