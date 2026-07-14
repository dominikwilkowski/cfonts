import { Align, Cfonts, Font } from "cfonts";

// say() renders for the terminal and prints in one step
Cfonts.text("hello").font(Font.Block).newText("world").font(Font.Tiny).say();

// one composition can render for every environment
const composition = Cfonts.text("hello world").font(Font.Tiny).align(Align.Center);

// the terminal render detects the terminal width by itself;
// the FORCE_SIZE environment variable overrides the detection
const cli = composition.renderCli();
console.log(cli.text);

// the same composition as an HTML fragment
const html = composition.renderBrowser();
console.log(html.text);
