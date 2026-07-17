import { BrowserConsoleEnv, BrowserHost, Cfonts, Font } from "cfonts";

const host = new BrowserHost();

Cfonts.text("hello").font(Font.Block).say(host);

const artifact = Cfonts.text("hi there").font(Font.Tiny).renderWith(BrowserConsoleEnv);

// A logging library can consume the artifact without cfonts logging it
console.info(artifact.text);

// TODO(color): logger.info(artifact.text, ...artifact.styles)
