import { BrowserConsoleEnv, BrowserEnv, BrowserHost, Cfonts, fontNames } from "cfonts";

// The page is an eighty column terminal, so long text wraps the way it would there
const host = BrowserHost.fromOverrides({ canvasWidth: 80 });

// The header, a red to blue gradient that moves one column further every iteration
const logo = document.getElementById("logo");

// One color per column of the logo, the red to blue gradient cfonts draws, so the ring rotates without a seam
const gradient = [
	"#ff0000",
	"#ff001a",
	"#ff0035",
	"#ff0050",
	"#ff006b",
	"#ff0086",
	"#ff00a1",
	"#ff00bb",
	"#ff00d6",
	"#ff00f1",
	"#f100ff",
	"#d600ff",
	"#bb00ff",
	"#a100ff",
	"#8600ff",
	"#6b00ff",
	"#5000ff",
	"#3500ff",
	"#1a00ff",
	"#0000ff",
	"#001aff",
	"#0035ff",
	"#0050ff",
	"#006bff",
	"#0086ff",
	"#00a1ff",
	"#00bbff",
	"#00d6ff",
	"#00f1ff",
	"#00fff1",
	"#00ffd6",
	"#00ffbb",
	"#00ffa1",
	"#00ff86",
	"#00ff6b",
	"#00ff50",
	"#00ff35",
	"#00ff1a",
	"#00ff00",
	"#1aff00",
	"#35ff00",
	"#50ff00",
	"#6bff00",
	"#86ff00",
	"#a1ff00",
	"#bbff00",
	"#d6ff00",
	"#f1ff00",
	"#fff100",
	"#ffd600",
	"#ffbb00",
	"#ffa100",
	"#ff8600",
	"#ff6b00",
	"#ff5000",
	"#ff3500",
	"#ff1a00",
];
let offset = 0;

function paintLogo() {
	logo.innerHTML = Cfonts.text("cfonts")
		.spaceless()
		.colors({ start: gradient[(offset + gradient.length - 1) % gradient.length], end: gradient[offset] })
		.render(host, BrowserEnv).text;
}
paintLogo();

if (!matchMedia("(prefers-reduced-motion: reduce)").matches) {
	setInterval(() => {
		offset = (offset + 1) % gradient.length;
		paintLogo();
	}, 100);
}

// The configurator, one form whose fields are the options of the command line
const form = document.getElementById("configurator");
const command = document.getElementById("command");
const canvas = document.getElementById("canvas");

// The font pickers list the command line names of the core, the pick goes back to cfonts by name
for (const [select, chosen] of [
	[form.elements.font, "neat"],
	[form.elements["next-font"], "tiny"],
]) {
	for (const name of fontNames()) {
		select.add(new Option(name, name));
	}
	select.value = chosen;
}

// A word of the command line in quotes, the way a text is passed
function quoted(value) {
	return `"${value.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
}

// A word of the command line, quoted when the shell would need it
function shellWord(value) {
	return /^[\w,:-]+$/.test(value) ? value : quoted(value);
}

// One whole number, or the value that is none, the sentence the framework pages print for it
//
// The framework pages count in usize, 32 bits on wasm32, so a larger number is none on every page
function wholeNumber(value) {
	if (!/^\d+$/.test(value) || Number(value) > 4294967295) {
		throw new Error(`"${value}" is not a whole number`);
	}

	return Number(value);
}

/*
 * The options in the order the command line takes them, each with the flag it prints, the value
 * the command line assumes when it is left out, and the builder call it makes
 * The colors of the first block color every block, as they do on the command line
 * A flag option prints its flag alone, a next-font applies only once a next block exists
 */
const options = [
	{ name: "font", flag: "--font", unset: "block", apply: (cfonts, value) => cfonts.font(value) },
	{
		name: "letter-spacing",
		flag: "--letter-spacing",
		unset: "1",
		apply: (cfonts, value) => cfonts.letterSpacing(wholeNumber(value)),
	},
	{
		name: "line-height",
		flag: "--line-height",
		unset: "",
		apply: (cfonts, value) => cfonts.lineHeight(wholeNumber(value)),
	},
	{ name: "word-wrap", flag: "--word-wrap", unset: "", apply: (cfonts) => cfonts.wordWrap() },
	{ name: "colors", flag: "--colors", unset: "system", apply: (cfonts, value) => cfonts.globalColors(value) },
	{ name: "background", flag: "--background", unset: "system", apply: (cfonts, value) => cfonts.background(value) },
	{
		name: "independent-gradient",
		flag: "--independent-gradient",
		unset: "",
		apply: (cfonts) => cfonts.independentGradient(),
	},
	{ name: "align", flag: "--align", unset: "left", apply: (cfonts, value) => cfonts.align(value) },
	{ name: "valign", flag: "--valign", unset: "middle", apply: (cfonts, value) => cfonts.valign(value) },
	{
		name: "max-length",
		flag: "--max-length",
		unset: "0",
		apply: (cfonts, value) => cfonts.maxLength(wholeNumber(value)),
	},
	{ name: "spaceless", flag: "--spaceless", unset: "", apply: (cfonts) => cfonts.spaceless() },
	{ name: "next", flag: "--next", unset: "", quoted: true, apply: (cfonts, value) => cfonts.next(value) },
	{ name: "next-font", flag: "--font", unset: "block", apply: (cfonts, value) => cfonts.font(value) },
];

// The composition the form describes and the command line that describes it, or the option the command line would refuse
function compose(data) {
	const text = data.get("text");
	const cfonts = Cfonts.text(text);
	const words = ["cfonts", quoted(text)];

	for (const option of options) {
		const value = data.get(option.name) ?? "";
		const skipped = value === "" || value === option.unset || (option.name === "next-font" && data.get("next") === "");

		if (skipped) {
			continue;
		}

		try {
			option.apply(cfonts, value);
		} catch (error) {
			return { command: words.join(" "), error: { name: option.name, message: error.message } };
		}

		words.push(option.flag);
		if (value !== "on") {
			words.push(option.quoted ? quoted(value) : shellWord(value));
		}
	}

	return { cfonts, command: words.join(" ") };
}

// Runs the command line the form spells: in the page or in the devtools console, an error where the output would go
function run() {
	const data = new FormData(form);
	const { cfonts, command: line, error } = compose(data);

	for (const control of form.querySelectorAll("input, select")) {
		control.setCustomValidity("");
	}
	command.textContent = line;

	if (error) {
		form.elements[error.name].setCustomValidity(error.message);
	}

	if (data.get("env") === "console") {
		if (error) {
			console.error(`ERROR ${error.message}`);
		} else {
			cfonts.say(host, BrowserConsoleEnv);
		}
		return;
	}

	if (error) {
		const output = document.createElement("span");
		const badge = document.createElement("span");
		badge.className = "error";
		badge.textContent = "ERROR";
		output.append(badge, ` ${error.message}`);
		canvas.replaceChildren(output);
		canvas.setAttribute("aria-label", error.message);
	} else {
		canvas.innerHTML = cfonts.render(host, BrowserEnv).text;
		canvas.setAttribute("aria-label", data.get("text"));
	}
}

run();

// The page follows every keystroke and pick, the console prints on every pick and a typed value prints on enter
form.addEventListener("input", () => {
	if (form.elements.env.value === "browser") {
		run();
	}
});
form.addEventListener("change", (event) => {
	if (form.elements.env.value === "console" && !event.target.matches("input:is([type='text'], [type='number'])")) {
		run();
	}
});
form.addEventListener("submit", (event) => {
	event.preventDefault();
	run();
});
