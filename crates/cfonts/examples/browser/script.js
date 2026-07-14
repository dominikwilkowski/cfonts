import { Align, Cfonts, Font } from "cfonts";

const banner = document.querySelector("#banner");

if (!(banner instanceof HTMLElement)) {
	throw new Error("Missing the banner element");
}

const composition = Cfonts.text("hello world").font(Font.Block).align(Align.Center).spaceless();

// render HTML in the browser
banner.innerHTML = composition.renderBrowser().text;

// or use say() to speak to the devtools console
composition.say();
