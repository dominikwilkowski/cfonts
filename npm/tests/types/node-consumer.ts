import { BrowserConsoleEnv, Cfonts, Color, Font, GradientPreset, hexToRgb, NodeHost } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = NodeHost.fromOverrides({ canvasWidth: 80 });

const rendered = banner.render(host);
banner.say(host);

const artifact = banner.renderWith(BrowserConsoleEnv, {
	canvasWidth: 80,
});

const colorful = Cfonts.text("colors")
	.colors([Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }])
	.colors({ preset: GradientPreset.Pride })
	.globalColors({ start: "red", end: "#0000ff" })
	.independentGradient();

colorful.colors({ transition: ["red", { red: 0, green: 0, blue: 255 }, "#00ff00"] });
colorful.colors({ start: Color.Red, end: hexToRgb("#0000ff") });
colorful.colors({ transition: [Color.Red, Color.Gray, hexToRgb("#8899dd")] });

const channels: { red: number; green: number; blue: number } = hexToRgb("#ff8800");
console.log(channels.red);
colorful.colors({ preset: GradientPreset.Lesbian });
colorful.render(host);

Cfonts.text("global").globalColors([Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }]);

Cfonts.text("banded").background(Color.Blue);
Cfonts.text("banded").background(Color.System);
Cfonts.text("banded").background("#222");
Cfonts.text("banded").background(hexToRgb("#222222"));
Cfonts.text("banded").background({ start: Color.Red, end: "#0000ff" });
Cfonts.text("banded").background({ transition: [Color.Red, Color.WhiteBright] });
Cfonts.text("banded").background({ preset: GradientPreset.Pride });

// @ts-expect-error Candy rolls per segment and cannot fill a row
Cfonts.text("banded").background(Color.Candy);

// @ts-expect-error a bare preset would read as a Color, presets go in their object form
Cfonts.text("banded").background(GradientPreset.Pride);

// @ts-expect-error channels and a gradient are two backgrounds
Cfonts.text("banded").background({ red: 1, green: 2, blue: 3, start: Color.Red, end: Color.Blue });

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

// @ts-expect-error a gradient takes exactly one shape
colorful.colors({ preset: GradientPreset.Pride, start: Color.Red, end: Color.Blue });

// @ts-expect-error the independent flag is a builder setting, not a gradient field
colorful.colors({ start: Color.Red, end: Color.Blue, independentGradient: true });

// readonly color lists are accepted: the methods only read them
const readonlyColors = [Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }] as const;
Cfonts.text("frozen").colors(readonlyColors).globalColors(readonlyColors);

const readonlyTyped: readonly (Color | string)[] = [Color.Red, "#8899dd"];
Cfonts.text("frozen").colors(readonlyTyped);
