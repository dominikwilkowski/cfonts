import { Align, BrowserConsoleEnv, BrowserHost, Cfonts, Color, ColorLevel, Font, hexToRgb } from "cfonts";

const host = new BrowserHost();

// Render to HTML
const banner = document.getElementById("banner");
banner.innerHTML = Cfonts.text("hello")
	.font(Font.Block)
	.align(Align.Center)
	.spaceless()
	.globalGradient({ start: Color.Red, end: hexToRgb("#0000ff"), independentGradient: true })
	.newText(" world")
	.font(Font.Chrome)
	.colors([Color.Red, Color.Blue, Color.Candy])
	.newText("|How are you?")
	.font(Font.Huge)
	.render(host).text;

// Print to browser console
const composition = Cfonts.text("Pretty Console!")
	.font(Font.Pallet)
	.colors([Color.Cyan, Color.Red])
	.renderWith(BrowserConsoleEnv, { colorLevel: ColorLevel.TrueColor });

// A logging library can consume the artifact without cfonts logging it
console.info(composition.text, ...composition.styles);

// The browser console form
const consoleForm = document.getElementById("console_form");
const input = document.getElementById("input");

consoleForm.addEventListener("submit", (event) => {
	event.preventDefault();

	Cfonts.text(input.value).font(Font.Chrome).gradient({ start: Color.Blue, end: Color.Green }).say(host);
});
