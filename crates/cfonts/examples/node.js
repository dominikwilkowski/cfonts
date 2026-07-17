import { BrowserEnv, Cfonts, Font, NodeHost } from "cfonts";

const host = new NodeHost();

Cfonts.text("hello").font(Font.Block).say(host);

const composition = Cfonts.text("hello world").font(Font.Tiny);

const terminal = composition.render(host);
console.log(terminal.text);

const html = composition.renderWith(BrowserEnv);
console.log(html.text);
