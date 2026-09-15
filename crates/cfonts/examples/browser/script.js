import { BrowserHost, Cfonts, Font, GradientPreset } from "cfonts";

// The page is an eighty column terminal, so long text wraps the way it would there
const host = BrowserHost.fromOverrides({ canvasWidth: 80 });

// The header, a red to blue gradient that moves one column further every iteration
const logo = document.getElementById("logo");
// One color per column of the logo, the red to blue gradient cfonts draws and the way back, so the ring rotates without a seam
const gradient = [
	"#ff0000",
	"#ff2400",
	"#ff4800",
	"#ff6d00",
	"#ff9100",
	"#ffb600",
	"#ffda00",
	"#ffff00",
	"#daff00",
	"#b6ff00",
	"#91ff00",
	"#6dff00",
	"#48ff00",
	"#24ff00",
	"#00ff00",
	"#00ff24",
	"#00ff48",
	"#00ff6d",
	"#00ff91",
	"#00ffb6",
	"#00ffda",
	"#00ffff",
	"#00daff",
	"#00b6ff",
	"#0091ff",
	"#006dff",
	"#0048ff",
	"#0024ff",
	"#0000ff",
	"#0023ff",
	"#0046ff",
	"#0069ff",
	"#008cff",
	"#00afff",
	"#00d3ff",
	"#00f6ff",
	"#00ffe4",
	"#00ffc1",
	"#00ff9e",
	"#00ff7b",
	"#00ff57",
	"#00ff34",
	"#00ff11",
	"#11ff00",
	"#34ff00",
	"#57ff00",
	"#7bff00",
	"#9eff00",
	"#c1ff00",
	"#e4ff00",
	"#fff600",
	"#ffd300",
	"#ffaf00",
	"#ff8c00",
	"#ff6900",
	"#ff4600",
	"#ff2300",
];

function paintLogo() {
	logo.innerHTML = Cfonts.text("cfonts")
		.font(Font.Block)
		.spaceless()
		.colors({ transition: gradient })
		.render(host).text;
}
paintLogo();

if (!matchMedia("(prefers-reduced-motion: reduce)").matches) {
	setInterval(() => {
		gradient.unshift(gradient.pop());
		paintLogo();
	}, 100);
}

// Render to HTML
const canvas = document.getElementById("canvas");
const browserCode = document.getElementById("browser_code");
const browserFont = document.getElementById("browser_font");
const consoleFont = document.getElementById("console_font");

for (const select of [browserFont, consoleFont]) {
	for (const name of Object.keys(Font).filter((key) => Number.isNaN(Number(key)))) {
		select.add(new Option(name, name));
	}
	select.value = "Neat";
}

const colorChoices = {
	redblue: { start: "red", end: "blue" },
	cyan: ["cyan"],
	candy: ["candy"],
	magentacyan: ["magenta", "cyan"],
	agender: { preset: GradientPreset.Agender },
};

const bgChoices = {
	system: "System",
	red: "red",
	redblue: { start: "red", end: "blue" },
	pride: { preset: GradientPreset.Pride },
};

// The cli spelling of a choice, for the code shown under each form
function source(value) {
	if (Array.isArray(value)) {
		return value.join(",");
	}

	if (typeof value === "object") {
		if (value.preset) {
			return GradientPreset[value.preset];
		} else {
			return [value.start, value.end].join("-");
		}
	}

	return value;
}

// The builder chain one form describes, and the same chain spelled out for the reader
function pick(input, font, colors, background) {
	const builder = Cfonts.text(input.value)
		.font(Font[font.value])
		.colors(colorChoices[colors.value])
		.background(bgChoices[background.value])
		.wordWrap();
	const code = `cfonts "${source(input.value)}" --font ${font.value} --colors ${source(colorChoices[colors.value])} --background ${source(bgChoices[background.value])} --word-wrap`;

	return { builder, code };
}

// The browser form
const browserForm = document.getElementById("browser_form");
const browserInput = document.getElementById("browser_input");
const browserColors = document.getElementById("browser_colors");
const browserBackground = document.getElementById("browser_background");

function paintBrowser() {
	const { builder, code } = pick(browserInput, browserFont, browserColors, browserBackground);
	canvas.innerHTML = builder.render(host).text;
	canvas.setAttribute("aria-label", browserInput.value);
	browserCode.textContent = code;
}
paintBrowser();

// The canvas follows every keystroke and every pick, enter renders it again instead of reloading the page
browserForm.addEventListener("input", paintBrowser);
browserForm.addEventListener("submit", (event) => {
	event.preventDefault();
	paintBrowser();
});

// The console form
const consoleForm = document.getElementById("console_form");
const consoleInput = document.getElementById("console_input");
const consoleColors = document.getElementById("console_colors");
const consoleBackground = document.getElementById("console_background");
const consoleCode = document.getElementById("console_code");

function paintConsole() {
	const { builder, code } = pick(consoleInput, consoleFont, consoleColors, consoleBackground);
	builder.say(host);
	consoleCode.textContent = code;
}
paintConsole();

// Every pick prints a banner and enter prints it again, a typed text waits for enter so it prints once
for (const select of [consoleFont, consoleColors, consoleBackground]) {
	select.addEventListener("change", paintConsole);
}
consoleForm.addEventListener("submit", (event) => {
	event.preventDefault();
	paintConsole();
});
