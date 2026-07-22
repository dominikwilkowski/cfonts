import { BrowserConsoleEnv, BrowserHost, Cfonts, Color, Font, GradientPreset } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = BrowserHost.fromOverrides({ canvasWidth: 80 });

const html = banner.render(host);
banner.say(host);

banner.colors([Color.RedBright, "#f80"]).globalGradient(GradientPreset.Bisexual);

const consoleArtifact = banner.renderWith(BrowserConsoleEnv);

const text: string = html.text;
console.log(text, consoleArtifact.text);

// @ts-expect-error NodeHost is not exported from the browser entry
import { NodeHost } from "cfonts";
