import { Align, BrowserEnv, Cfonts, Color, Font, NodeHost, Valign } from "cfonts";

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

// Colors paint through the host's resolved support level, one per font color slot
Cfonts.text("colors").colors([Color.Red, Color.Yellow]).say(host);

// A gradient ramps between two colors the long way around the color wheel, one color per column
Cfonts.text("rainbow").colors({ start: Color.Red, end: Color.Blue }).say(host);

// A transition travels straight to each color
Cfonts.text("sunset")

	.colors({ transition: [Color.Yellow, "#ff8800", Color.Magenta] })
	.say(host);

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

console.log(""); // Adding some space between outputs

// A background gradient ramps from the top row down, the long way around the color wheel like font gradients do
Cfonts.text(" Right ")
	.align(Align.Right)
	.colors([Color.Black, Color.Black])
	.background({ start: Color.Blue, end: Color.Magenta })
	.font(Font.Huge)
	.say(host);

console.log(""); // Adding some space between outputs

// A background transition travels straight through every stop, top to bottom
Cfonts.text("neon")
	.align(Align.Center)
	.font(Font.Chrome)
	.colors(["#f08", "#f08", "#f08"])
	.background({ transition: [Color.Magenta, Color.Cyan, Color.Magenta] })
	.say(host);

console.log(""); // Adding some space between outputs

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

console.log(""); // Adding some space between outputs

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
	.colors([Color.White])
	.spaceless()
	.align(Align.Center)
	.say(host);
