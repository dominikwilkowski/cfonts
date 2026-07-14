import { Align, Cfonts, Font } from "cfonts";

const banner = document.querySelector("#banner");

if (!(banner instanceof HTMLElement)) {
	throw new Error("Missing the banner element");
}

const rendered = Cfonts.text("hello").font(Font.Block).align(Align.Center).spaceless().renderBrowser();

banner.innerHTML = rendered.text;
