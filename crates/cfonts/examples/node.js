import {
	Align,
	BrowserConsoleEnv,
	BrowserEnv,
	Cfonts,
	CliEnv,
	Color,
	ColorLevel,
	Font,
	GradientPreset,
	hexToRgb,
	NodeHost,
	Valign,
} from "cfonts";

const host = new NodeHost();

// A quick print to stdout with the default settings
Cfonts.text("hello cfonts").say(host);

// Get the instance and do something with it later
const composition = Cfonts.text("hello world").font(Font.Tiny);

// Render manually with your own pipe to stdout
const terminal = composition.render(host);
console.log(terminal.text);

// Use the same instance to render to another env manually
const html = composition.renderWith(BrowserEnv);
console.log(html.text);

// A file is not a terminal: width zero lifts the wrap and false paints no escape codes
const fileHost = NodeHost.fromOverrides({ canvasWidth: 0, color: false });
const notes = Cfonts.text("release notes").font(Font.Simple).render(fileHost);
console.log(notes.text); // ready for writeFileSync("NOTES.txt", notes.text)

// A serial console knows sixteen colors, so a hex gradient snaps to the closest ones
const serialHost = NodeHost.fromOverrides({ color: ColorLevel.Basic });
Cfonts.text("serial").colors({ start: "#f80", end: "#80f" }).say(serialHost);

// Candy rolls a fresh color per painted segment, a seed makes the roll repeatable
const seededHost = NodeHost.fromOverrides({ seed: 42 });
Cfonts.text("same").font(Font.Chrome).colors([Color.Candy, Color.Candy]).spaceless().say(seededHost);
Cfonts.text("same").font(Font.Chrome).colors([Color.Candy, Color.Candy]).spaceless().say(seededHost); // the same picks again

// renderWith detects nothing, so colors need a level in the context
// A static HTML report embeds the banner in true color, a preset is a transition through the flag's stops
const report = Cfonts.text("report")
	.font(Font.Chrome)
	.colors({ preset: GradientPreset.Pride })
	.renderWith(BrowserEnv, { colorLevel: ColorLevel.TrueColor });
console.log(report.text); // ready for writeFileSync("report.html", report.text)

// The browser console form pairs every %c marker in the text with one entry of styles
const devtools = Cfonts.text("devtools")
	.font(Font.Tiny)
	.colors({ start: Color.Cyan, end: Color.Blue })
	.renderWith(BrowserConsoleEnv, { colorLevel: ColorLevel.TrueColor });
console.log(devtools.text, ...devtools.styles); // node skips the styles, a browser console paints them

// The host resolves the terminal width but this can be overwritten
const fixedHost = NodeHost.fromOverrides({ canvasWidth: 40 });
Cfonts.text("hello fixed world").font(Font.Edge).align(Align.Center).say(fixedHost);

// Or you can use the renderWith method
const fixedRendered = Cfonts.text("hello small world")
	.font(Font.Edge)
	.align(Align.Right)
	.renderWith(CliEnv, { canvasWidth: 44 });
console.log(fixedRendered.text);

// Colors paint through the host's resolved support level, one per font color slot
Cfonts.text("colors").colors([Color.Red, Color.Yellow]).say(host);

// A gradient ramps between two colors the long way around the color wheel, one color per column
Cfonts.text("rainbow").colors({ start: Color.Red, end: Color.Blue }).say(host);

// A transition travels straight to each color
Cfonts.text("sunset")
	.colors({ transition: [Color.Yellow, "#ff8800", Color.Magenta] })
	.say(host);

// Design tokens come as channels, hexToRgb turns a hex value into that shape
const brand = hexToRgb("#f08");
Cfonts.text("brand")
	.colors([brand, { red: 255, green: 255, blue: 255 }]) // colors can also be set as objects
	.say(host);

// System keeps the terminal's own text color, here for the fill while only the frame is painted
// So the first color works on dark mode and light mode and any other terminal themes
Cfonts.text(" theme ").font(Font.Shade).colors([Color.System, Color.Yellow]).say(host);

// Blocks share one line, each with its own font and colors
Cfonts.text("say ")
	.font(Font.Tiny)
	.colors({ start: Color.Green, end: Color.Magenta })
	.next("fire")
	.font(Font.Tiny)
	.colors([Color.YellowBright])
	.say(host);

// Global colors cover every block and can be set anywhere while color setters have to be set within the current block
// Once a global color is set, a block with its own colors keeps them can overwrite them
// but blocks not having set a color inherit the global color
Cfonts.text("one ")
	.font(Font.Tiny)
	.next("two ")
	.font(Font.Tiny)
	.colors(["#fff"])
	.globalColors([Color.Yellow]) // this could also be a gradient just like any `.colors()` setter
	.next("three")
	.font(Font.Tiny)
	// no color set in this block
	.say(host);

// A global gradient ramps across every block as one, and a preset works here too
Cfonts.text("two")
	.next(" fonts")
	.font(Font.Tiny)
	.valign(Valign.Middle)
	.globalColors({ preset: GradientPreset.Transgender })
	.say(host);

// Setting the independentGradient means each line will use its real length,
// without it a gradient uses the longest line to calculate the gradient colors
Cfonts.text("All you need is|Love")
	.font(Font.Braille)
	.align(Align.Center)
	.colors({ start: Color.Red, end: Color.Blue })
	.say(host);
Cfonts.text("All you need is|Love")
	.font(Font.Braille)
	.align(Align.Center)
	.colors({ start: Color.Red, end: Color.Blue })
	.independentGradient()
	.say(host);

// You can set a static background for your output which will include the padding (which can be disabled with `.spaceless()`)
Cfonts.text(" Banner ").colors([Color.White, Color.Yellow]).background(Color.Blue).say(host);

console.log(""); // Adding some space between examples

// A background gradient ramps from the top row down, the long way around the color wheel like font gradients do
Cfonts.text(" Right ")
	.align(Align.Right)
	.colors([Color.Black, Color.Black])
	.background({ start: Color.Blue, end: Color.Magenta })
	.font(Font.Huge)
	.say(host);

console.log(""); // Adding some space between examples

// A background transition travels straight through every stop, top to bottom
Cfonts.text("neon")
	.align(Align.Center)
	.font(Font.Chrome)
	.colors(["#f08", "#f08", "#f08"])
	.background({ transition: [Color.Magenta, Color.Cyan, Color.Magenta] })
	.say(host);

console.log(""); // Adding some space between examples

// A background is global and spans every block, blocks of different heights meet at the row valign picks
// Align within the width of the terminal is global and effects all blocks
Cfonts.text("cfonts")
	.font(Font.Dense)
	.colors(["f08", "f08", "f08"])
	.next(" v4")
	.font(Font.Console)
	.colors(["#fff"])
	.valign(Valign.Bottom)
	.background(Color.Gray)
	.align(Align.Center)
	.say(host);

console.log(""); // Adding some space between examples

// Spaceless drops the padding (two empty lines above and below) for tight stacks
Cfonts.text("Neat").font(Font.Neat).colors(["#fff"]).spaceless().background("f00").say(host);

// Max length breaks a line after this many glyphs (it means max characters)
// word wrap moves whole words to the next line instead of breaking them mid way
Cfonts.text("wrap whole words").font(Font.Retro).maxLength(8).colors({ start: "f08", end: "f08" }).say(host);
Cfonts.text("wrap whole words").font(Font.Retro).maxLength(8).colors({ start: "f08", end: "f08" }).wordWrap().say(host);

// Letter spacing widens the gap between letters, line height sets the rows between lines
Cfonts.text("wide|normal").font(Font.Thin).letterSpacing(3).say(host);
Cfonts.text("tight|close").font(Font.Thin).lineHeight(0).say(host);

// Put together: a startup banner with a logo and a status line
Cfonts.text("Bronzies")
	.colors([Color.Red, "#ff0"])
	.next("|Bronzies-RESTful-API listening at http://0.0.0.0:5555")
	.font(Font.Console)
	.colors({ start: "#f00", end: "#fff" })
	.spaceless()
	.align(Align.Center)
	.say(host);

console.log(""); // Adding some space between examples

// A host is any object with render and say methods
// This custom host prefixes every line for a build log
const buildLog = {
	render: (composition) => composition.renderWith(CliEnv, { canvasWidth: 60, colorLevel: ColorLevel.Ansi256 }),
	say(composition) {
		for (const line of this.render(composition).text.split("\n")) {
			process.stdout.write(`[build] ${line}\n`);
		}
	},
};
Cfonts.text("step 3").font(Font.Tiny).colors([Color.Green]).spaceless().say(buildLog);
Cfonts.text("failed")
	.font(Font.Tiny)
	.colors([Color.White])
	.background(Color.Red)
	.spaceless()
	.align(Align.Center)
	.say(buildLog);

console.log(""); // Adding some space between examples
