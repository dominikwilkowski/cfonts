import { BrowserConsoleEnv, Cfonts, Font, NodeHost } from "cfonts";

const banner = Cfonts.text("hello").font(Font.Block);
const host = NodeHost.fromOverrides({ canvasWidth: 80 });

const rendered = banner.render(host);
banner.say(host);

const artifact = banner.renderWith(BrowserConsoleEnv, {
	canvasWidth: 80,
});

const text: string = rendered.text;
console.log(text, artifact.text);

// @ts-expect-error BrowserHost is not exported from the Node entry
import { BrowserHost } from "cfonts";
