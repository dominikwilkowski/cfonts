import { BrowserConsoleEnv, BrowserHost, Cfonts, Color, ColorLevel, Font } from "cfonts";

const host = new BrowserHost();

Cfonts.text("hello").font(Font.Block).colors([Color.Red, Color.Blue]).say(host);

const artifact = Cfonts.text("hi there")
	.font(Font.Tiny)
	.colors([Color.Cyan])
	.renderWith(BrowserConsoleEnv, { colorLevel: ColorLevel.TrueColor });

// A logging library can consume the artifact without cfonts logging it
console.info(artifact.text, ...artifact.styles);
