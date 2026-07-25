import { BrowserEnv, Cfonts, Color, Font, GradientPreset, NodeHost } from "cfonts";

const host = new NodeHost();

Cfonts.text("hello").font(Font.Block).say(host);

const composition = Cfonts.text("hello world").font(Font.Tiny);

const terminal = composition.render(host);
console.log(terminal.text);

const html = composition.renderWith(BrowserEnv);
console.log(html.text);

// colors paint through the host's resolved support level
Cfonts.text("colors").font(Font.Block).colors([Color.Red, Color.Blue]).say(host);

// gradients ramp one color per column, presets are transitions
Cfonts.text("pride").font(Font.Block).globalGradient(GradientPreset.Pride).say(host);
