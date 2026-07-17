import { BrowserConsoleEnv, BrowserHost, Cfonts, Font } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = BrowserHost.fromOverrides({ canvasWidth: 80 });

const html = banner.render(host);
banner.say(host);

const consoleArtifact = banner.renderWith(BrowserConsoleEnv);

const text: string = html.text;
console.log(text, consoleArtifact.text);

// @ts-expect-error NodeHost is not exported from the browser entry
import { NodeHost } from "cfonts";
