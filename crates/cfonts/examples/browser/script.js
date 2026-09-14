import {
	Align,
	BrowserConsoleEnv,
	BrowserHost,
	Cfonts,
	Color,
	ColorLevel,
	Font,
	GradientPreset,
	hexToRgb,
} from "cfonts";

const host = new BrowserHost();

// Render to HTML
const canvas = document.getElementById("canvas");
canvas.innerHTML = Cfonts.text("hello")
	.font(Font.Block)
	.align(Align.Center)
	.globalColors({ start: Color.Red, end: hexToRgb("#0000ff") })
	.independentGradient()
	.next(" world|")
	.font(Font.Chrome)
	.colors([Color.Red, Color.Blue, Color.Candy])
	.next("How are you?")
	.font(Font.Huge)
	.render(host).text;

// Print to browser console
const composition = Cfonts.text("Pretty Console!")
	.font(Font.Pallet)
	.colors([Color.Cyan, Color.Red])
	.renderWith(BrowserConsoleEnv, { colorLevel: ColorLevel.TrueColor });

// A logging library can consume the artifact without cfonts logging it
console.info(composition.text, ...composition.styles);

// The font selectors
const browserFont = document.getElementById("browser_font");
const consoleFont = document.getElementById("console_font");

for (const select of [browserFont, consoleFont]) {
	for (const name of Object.keys(Font).filter((key) => Number.isNaN(Number(key)))) {
		select.add(new Option(name, name));
	}
	select.value = "Huge";
}

const colorChoices = {
	blue: ["blue"],
	candy: ["candy"],
	cyanmagenta: ["cyan", "magenta"],
	redblue: { start: "red", end: "blue" },
	agender: { preset: GradientPreset.Agender },
};

const bgChoices = {
	system: "System",
	red: "red",
	redblue: { start: "red", end: "blue" },
	pride: { preset: GradientPreset.Pride },
};

// The browser form
const browserForm = document.getElementById("browser_form");
const browserInput = document.getElementById("browser_input");
const browserColors = document.getElementById("browser_colors");
const browserBackground = document.getElementById("browser_background");

browserForm.addEventListener("submit", (event) => {
	event.preventDefault();

	canvas.innerHTML = Cfonts.text("hello")
		.font(Font.Block)
		.align(Align.Center)
		.globalColors({ start: Color.Red, end: hexToRgb("#0000ff") })
		.independentGradient()
		.next(" world|")
		.font(Font.Chrome)
		.colors([Color.Red, Color.Blue, Color.Candy])
		.next(browserInput.value)
		.font(Font.Huge)
		.colors(colorChoices[browserColors.value])
		.font(Font[browserFont.value])
		.background(bgChoices[browserBackground.value])
		.render(host).text;
});

// The browser console form
const consoleForm = document.getElementById("console_form");
const consoleInput = document.getElementById("console_input");
const consoleColors = document.getElementById("console_colors");
const consoleBackground = document.getElementById("console_background");

consoleForm.addEventListener("submit", (event) => {
	event.preventDefault();

	Cfonts.text(consoleInput.value)
		.font(Font[consoleFont.value])
		.colors(colorChoices[consoleColors.value])
		.background(bgChoices[consoleBackground.value])
		.say(host);
});
