import { Cfonts, Font } from "cfonts";

// say() in the browser prints directly into the devtools console
Cfonts.text("hello").font(Font.Block).say();

// render returns the banner instead, for handling it yourself
const rendered = Cfonts.text("hi there").font(Font.Tiny).renderBrowserConsole();
console.log(rendered.text);
