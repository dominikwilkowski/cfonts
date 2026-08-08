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
	.gradient(GradientPreset.Pride)
	.globalGradient({ start: "red", end: "#0000ff", independentGradient: true });

colorful.gradient({ transition: ["red", { red: 0, green: 0, blue: 255 }, "#00ff00"] });
colorful.gradient({ start: Color.Red, end: hexToRgb("#0000ff") });
colorful.gradient({ transition: [Color.Red, Color.Gray, hexToRgb("#8899dd")] });

const channels: { red: number; green: number; blue: number } = hexToRgb("#ff8800");
console.log(channels.red);
colorful.gradient({ preset: GradientPreset.Lesbian, independentGradient: true });
colorful.render(host);

Cfonts.text("global").globalColors([Color.Red, "#ff8800", { red: 1, green: 2, blue: 3 }]);

// @ts-expect-error an empty object is not a gradient
banner.gradient({});

const text: string = rendered.text;
const styles: string[] = artifact.styles;
console.log(text, artifact.text, styles.length);

// @ts-expect-error BrowserHost is not exported from the Node entry
import { BrowserHost } from "cfonts";

// @ts-expect-error the Node entry must not inject DOM globals
document;
