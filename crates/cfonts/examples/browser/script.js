import { Align, BrowserHost, Cfonts, Color, Font } from "cfonts";

const banner = document.querySelector("#banner");

if (!(banner instanceof HTMLElement)) {
	throw new Error("The browser example requires a #banner element");
}

const host = new BrowserHost();

const composition = Cfonts.text("hello")
	.font(Font.Block)
	.align(Align.Center)
	.spaceless()
	.globalGradient({ start: "red", end: "blue", independentGradient: true })
	.newText(" world")
	.font(Font.Chrome)
	.colors([Color.Red, Color.Blue, Color.Candy])
	.newText("|How are you?")
	.font(Font.Huge);

banner.innerHTML = composition.render(host).text;
