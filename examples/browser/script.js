import { Align, Cfonts, Env, Font } from "cfonts";

const banner = document.querySelector("#banner");

if (!(banner instanceof HTMLElement)) {
	throw new Error("Missing the banner element");
}

const rendered = Cfonts.text("hello").font(Font.Block).env(Env.Browser).align(Align.Center).spaceless().render();

banner.innerHTML = rendered.text;
