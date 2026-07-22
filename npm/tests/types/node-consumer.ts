import { BrowserConsoleEnv, Cfonts, Color, Font, GradientPreset, NodeHost } from "cfonts";

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
colorful.gradient({ preset: GradientPreset.Lesbian, independentGradient: true });
colorful.render(host);

// @ts-expect-error an empty object is not a gradient
banner.gradient({});

const text: string = rendered.text;
console.log(text, artifact.text);

// @ts-expect-error BrowserHost is not exported from the Node entry
import { BrowserHost } from "cfonts";
