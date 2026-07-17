import { Align, BrowserHost, Cfonts, Font } from "cfonts";

const banner = document.querySelector("#banner");

if (!(banner instanceof HTMLElement)) {
	throw new Error("The browser example requires a #banner element");
}

const host = new BrowserHost();

const composition = Cfonts.text("hello world").font(Font.Block).align(Align.Center).spaceless();

banner.innerHTML = composition.render(host).text;
