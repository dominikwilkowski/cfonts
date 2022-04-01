```shell
     ██████╗ ███████╗  ██████╗  ███╗   ██╗ ████████╗ ███████╗
    ██╔════╝ ██╔════╝ ██╔═══██╗ ████╗  ██║ ╚══██╔══╝ ██╔════╝
    ██║      █████╗   ██║   ██║ ██╔██╗ ██║    ██║    ███████╗
    ██║      ██╔══╝   ██║   ██║ ██║╚██╗██║    ██║    ╚════██║
    ╚██████╗ ██║      ╚██████╔╝ ██║ ╚████║    ██║    ███████║
     ╚═════╝ ╚═╝       ╚═════╝  ╚═╝  ╚═══╝    ╚═╝    ╚══════╝
```

![cfont styles](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/example1.png)


<p align="center"><img src="https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/example2.png" alt="api example"></p>
<p align="center"><a href="https://nodei.co/npm/cfonts/"><img src="https://nodei.co/npm/cfonts.png?downloads=true" alt="npm status"></a></p>
<p align="center">
	<a href="https://travis-ci.org/dominikwilkowski/cfonts"><img src="https://travis-ci.org/dominikwilkowski/cfonts.svg?branch=released" alt="build status"></a>
	<a href='https://coveralls.io/github/dominikwilkowski/cfonts?branch=released'><img src='https://coveralls.io/repos/github/dominikwilkowski/cfonts/badge.svg?branch=released' alt='CFont Coverage Status' /></a>
</p>

<p align="center">This is a silly little command line tool for sexy fonts in the console. <strong>Give your cli some love.</strong></p>

## Installing

To install the CLI app, simply NPM install it globally.

```shell
$ npm install cfonts -g
```

To use it in your shell:

```shell
$ cfonts "Hello|World\!"
```

_Remember to escape the `!` character with `\` in the shell_

Or use it in your project:

```js
const CFonts = require('cfonts');

CFonts.say('Hello|world!', {
	font: 'block',              // define the font face
	align: 'left',              // define text alignment
	colors: ['system'],         // define all colors
	background: 'transparent',  // define the background color, you can also use `backgroundColor` here as key
	letterSpacing: 1,           // define letter spacing
	lineHeight: 1,              // define the line height
	space: true,                // define if the output text should have empty lines on top and on the bottom
	maxLength: '0',             // define how many character can be on one line
	gradient: false,            // define your two gradient colors
	independentGradient: false, // define if you want to recalculate the gradient for each new line
	transitionGradient: false,  // define if this is a transition between colors directly
	env: 'node'                 // define the environment CFonts is being executed in
});
```

_All settings are optional and shown here with their default_

You can use CFonts in your project without the direct output to the console:

```js
const CFonts = require('cfonts');

const prettyFont = CFonts.render('Hello|world!', {/* same settings object as above */});

prettyFont.string  // the ansi string for sexy console font
prettyFont.array   // returns the array for the output
prettyFont.lines   // returns the lines used
prettyFont.options // returns the options used
```


## Usage

Using the CLI is easy.

```
Usage: cfonts  "<value>" [option1] <input1> [option2] <input1>,<input2> [option3] etc...
```

At any point you can run the *help* command to get a full list of commands and
how to use them.

```shell
$ cfonts --help
```


## Supported Characters

|     |     |     |             |
|-----|-----|-----|-------------|
| `A` | `P` | `4` | `$`         |
| `B` | `Q` | `5` | `%`         |
| `C` | `R` | `6` | `&`         |
| `D` | `S` | `7` | `(`         |
| `E` | `T` | `8` | `)`         |
| `F` | `U` | `9` | `/`         |
| `G` | `V` | `!` | `:`         |
| `H` | `W` | `?` | `;`         |
| `I` | `X` | `.` | `,`         |
| `J` | `Y` | `+` | `'`         |
| `K` | `Z` | `-` | `"`         |
| `L` | `0` | `_` | ` ` (space) |
| `M` | `1` | `=` |             |
| `N` | `2` | `@` |             |
| `O` | `3` | `#` |             |

_The `|` character will be replaced with a line break_


## Options

#### -h, --help
Type: `<command>`  
Default value: `none`

This shows a list of all available options.

```shell
$ cfonts --help
```

![Help command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/help.png)


#### -V, --version
Type: `<command>`  
Default value: `none`

This shows the installed version.

```shell
$ cfonts --version
```

![Version command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/version.png)


#### text
Type: `<string>`  
Default value: `""`

This is the "text input" to be converted into a nice font.  
The `|` character will be replaced with a line break.

```shell
$ cfonts "Hello world"
```

![Text command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/text.png)


#### -f, --font
Type: `<string>`  
Default value: `"block"`

This is the font face you want to use. So far this plugin ships with with following font faces:

```shell
$ cfonts "text" --font "chrome"
```

![Font command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/font.png)

- `block`       [colors: 2] _(default)_
	![block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/block.png)
- `slick`       [colors: 2]
	![slick font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/slick.png)
- `tiny`        [colors: 1]
	![tiny font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/tiny.png)
- `grid`        [colors: 2]
	![grid font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/grid.png)
- `pallet`      [colors: 2]
	![pallet font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/pallet.png)
- `shade`       [colors: 2]
	![shade font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/shade.png)
- `chrome`      [colors: 3]
	![chrome font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/chrome.png)
- `simple`      [colors: 1]
	![simple font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple.png)
- `simpleBlock` [colors: 1]
	![simple-block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple-block.png)
- `3d`          [colors: 2]
	![3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/3d.png)
- `simple3d`    [colors: 1]
	![simple-3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/simple-3d.png)
- `huge`        [colors: 2]
	![huge font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/huge.png)
- `console`     [colors: 1]
	![console font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/console.png)


#### -a, --align
Type: `<string>`  
Default value: `"left"`

You can align your text in the terminal with this option. Use the keywords below:

- `left` _(default)_
- `center`
- `right`
- `top` _(Will be ignored if used with the spaceless option)_
- `bottom` _(Will be ignored if used with the spaceless option)_

```shell
$ cfonts "text" --align "center"
```

![Align command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/align.png)


#### -c, --colors
Type: `<string list>`  
Default value: `['system']`

With this setting you can set the colors for your font.
Use the below color strings built in by [chalk](https://github.com/chalk/chalk) or a hex color.  
Provide colors in a comma-separated string, eg: `red,blue`. _(no spaces)_  
If you use a hex color make sure you include the `#` prefix. _(In the terminal wrap the hex in quotes)_  
The `system` color falls back to the system color of your terminal.

- `system` _(default)_
- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `gray`
- `redBright`
- `greenBright`
- `yellowBright`
- `blueBright`
- `magentaBright`
- `cyanBright`
- `whiteBright`
- `#ff8800` _(any valid hex color)_
- `#f80` _(short form is supported as well)_

```shell
$ cfonts "text" --colors white,"#f80"
```

![Colors command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/colors.png)


#### -g, --gradient
Type: `<string list>`  
Default value: `false`

With this setting you can set a gradient over your output.  
This setting supersedes the color open.  
The gradient requires two colors, a start color and an end color from left to right.  
_(If you want to set your own colors for the gradient, use the [transition](#-t---transition-gradient) option.)_  
CFonts will then generate a gradient through as many colors as it can find to make the output most impressive.  
Provide two colors in a comma-separated string, eg: `red,blue`. _(no spaces)_  
If you use a hex color make sure you include the `#` prefix. _(In the terminal wrap the hex in quotes)_  

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `gray`
- `grey`
- `#ff8800` _(any valid hex color)_
- `#f80` _(short form is supported as well)_

```shell
$ cfonts "text" --gradient red,"#f80"
```

![Gradient command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/gradient.png)


#### -i, --independent-gradient
Type: `<boolean>`  
Default value: `false`

Set this option to re-calculate the gradient colors for each new line.  
Only works in combination with the [gradient](#-g---gradient) option.

```shell
$ cfonts "text|next line" --gradient red,"#f80" --independent-gradient
```

![Independent gradient command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/independent-gradient.png)


#### -t, --transition-gradient
Type: `<boolean>`  
Default value: `false`

Set this option to generate your own gradients.
Each color set in the gradient option will then be transitioned to directly.
This option allows you to specify more than just two colors for your gradient.  
Only works in combination with the [gradient](#-g---gradient) option.

```shell
$ cfonts "text" --gradient red,"#f80",green,blue --transition-gradient
```

![Independent gradient command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/transition-gradient.png)


#### -b, --background
Type: `<string>`  
Default value: `"transparent"`

With this setting you can set the background colors for the output. Use the below color strings built in by [chalk](https://github.com/chalk/chalk).
Provide the background color from the below supported list, eg: 'white'

- `transparent` _(default)_
- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `blackBright`
- `redBright`
- `greenBright`
- `yellowBright`
- `blueBright`
- `magentaBright`
- `cyanBright`
- `whiteBright`

```shell
$ cfonts "text" --background "Green"
```

![Background command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/background.png)


#### -l, --letter-spacing
Type: `<integer>`  
Default value: `1`

Set this option to widen the space between characters.

```shell
$ cfonts "text" --letter-spacing 2
```

![Letter spacing command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/letter-spacing.png)


#### -z, --line-height
Type: `<integer>`  
Default value: `1`

Set this option to widen the space between lines.

```shell
$ cfonts "text" --line-height 2
```

![Line height command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/line-height.png)


#### -s, --spaceless
Type: `<boolean>`  
Default value: `false`

Set this option to false if you don't want the plugin to insert two empty lines on top and on the bottom of the output.

```shell
$ cfonts "text" --spaceless
```

![Spaceless command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/spaceless.png)


#### -m, --max-length
Type: `<integer>`  
Default value: `0`

This option sets the maximum characters that will be printed on one line.  
CFonts detects the size of your terminal but you can opt out and determine your own max width.  
`0` means no max width and the text will break at the edge of the terminal window.

```shell
$ cfonts "text" --max-length 15
```

![Max length command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/max-length.png)


#### -e, --env
Type: `<string>`  
Default value: `node`

This option lets you use CFonts to generate HTML instead of ANSI code.  
Note that `max-length` won't be automatically detected anymore and you will have to supply it if you want the text to wrap.
Best used in a node script.

```js
const CFonts = require('cfonts');
const path = require('path');
const fs = require('fs');

const output = CFonts.render('My text', {
	colors: ['white'],
	gradient: ['cyan', 'red'],
	background: 'black',
	space: false,
	env: 'browser',
});

fs.writeFileSync(
	path.normalize(`${ __dirname }/test.html`),
	output.string,
	{
		encoding: 'utf8',
	}
);
```

![Max length command](https://raw.githubusercontent.com/dominikwilkowski/cfonts/released/img/env.png)


## Consistency
[Chalk](https://github.com/chalk/chalk) detects what colors are supported on your platform.
It sets a [level of support](https://github.com/chalk/chalk#256-and-truecolor-color-support) automatically.
In CFonts you can override this by passing in the `FORCE_COLOR` environment variable.

```shell
FORCE_COLOR=3 cfonts "hello world" -c "#0088ff"
```

## Contributing
To build the repo install dependencies via:  
_(Since we ship a `yarn.lock` file please use [`yarn`](https://yarnpkg.com/) for development.)_

```shell
yarn
```

and run the watch to continuously transpile the code.

```shell
yarn watch
```

Please look at the coding style and work with it, not against it ;)


## Tests
This package is tested on the below platform and node combinations as part of our [CI](https://github.com/dominikwilkowski/cfonts/tree/released/.travis.yml).

| Platform | Node   |
|----------|--------|
| Linux    | v10    |
| Linux    | v12    |
| Linux    | latest |
| OSX      | v10    |
| OSX      | v12    |
| OSX      | latest |
| Windows  | v10    |
| Windows  | v12    |
| Windows  | latest |

### Unit tests
The package comes with a bunch of [unit tests](https://github.com/dominikwilkowski/cfonts/tree/released/test/unit) that aim to cover 100% of the code base.
For more details about the code coverage check out [coveralls](https://coveralls.io/github/dominikwilkowski/cfonts?branch=released).

```shell
npm run test:unit
```

### Type tests
Since the code base uses [JSDocs](https://jsdoc.app/) we use [typescript](https://www.typescriptlang.org/) to test the inferred types from those comments.
Typescript [supports JSDocs](https://www.typescriptlang.org/docs/handbook/type-checking-javascript-files.html#supported-jsdoc) and we use it in our
[test](https://github.com/dominikwilkowski/cfonts/blob/released/package.json#L38).

```shell
npm run test:types
```

### Font file test
There is also a [test suite](https://github.com/dominikwilkowski/cfonts/blob/released/test/fonttest.js) for font files.

```shell
npm run test:fonts
```

This tool checks:
- the existence of the font
- all attributes of a font
- each character for:
	- existence
	- consistent width
	- consistent lines

### All tests
Run all tests via:

```shell
npm run test
```


## Release History
* 2.10.1 -  bumped dependencies
* 2.10.0 -  bumped dependencies, added typescript definitions into npm bundle
* 2.9.3  -  bumped dependencies
* 2.9.2  -  bumped dependencies
* 2.9.1  -  bumped dependencies
* 2.9.0  -  added `top` and `bottom` align options
* 2.8.6  -  bumped dependencies
* 2.8.5  -  renamed branches
* 2.8.4  -  fixed block double quote
* 2.8.3  -  bumped dependencies
* 2.8.2  -  bumped dependencies, added linting, fixed #22 (again)
* 2.8.1  -  bumped dependencies
* 2.8.0  -  added environment support, added font `tiny`
* 2.7.0  -  added font `slick`, `grid` and `pallet`, added double quote to all fonts
* 2.6.1  -  fixed console `maxLength`, `gradient` and `lineHeight`, added more end-to-end tests
* 2.6.0  -  added transition gradients and sets
* 2.5.2  -  fixed jsDocs, added typescript type test
* 2.5.1  -  fixed array output to include everything including colors
* 2.5.0  -  added gradient option, separated code into files, added 100% unit testing coverage
* 2.4.8  -  removed `ansi-styles` from direct dependencies
* 2.4.7  -  fixed bug from adopting chalk v3 and hex colors
* 2.4.6  -  bumped dependencies, removed `change-case` dependency, added `UpperCaseFirst` with tests
* 2.4.5  -  bumped dependencies, moved to relative links for fonts for webpack support (#22)
* 2.4.4  -  bumped dependencies
* 2.4.3  -  bumped dependencies
* 2.4.2  -  bumped dependencies
* 2.4.1  -  updated to babel 7, removed runtime from dependencies
* 2.4.0  -  added font `shade`, added hex color support
* 2.3.1  -  added tests, fixed options, updated dependencies
* 2.3.0  -  added apostrophe support in all fonts
* 2.2.3  -  bumped dependencies
* 2.2.2  -  bumped dependencies
* 2.2.1  -  bumped dependencies
* 2.2.0  -  inside the API you can use line breaks as well as the pipe
* 2.1.3  -  refactored some loops
* 2.1.2  -  made WinSize more robust
* 2.1.1  -  fixed size detection in non-tty environments
* 2.1.0  -  rebuilt cfonts with pure functions, made colors case-insensitive
* 2.0.1  -  fixed terminal width detection
* 2.0.0  -  added tests, split into more pure functions
* 1.2.0  -  added `transparent` and `system` as default background and color option, added `backgroundColor` as alias for `background`, upgraded deps
* 1.1.3  -  fixed help text, removing old -t option
* 1.1.2  -  fixed issue with older commander version #3, updated docs
* 1.1.1  -  moved from `babel-polyfill` to `babel-plugin-transform-runtime`, added files to package.json, added images to docs, fixed dependencies
* 1.1.0  -  transpiled code to support node 0.12.15 and up
* 1.0.2  -  fixed background in `console` font, added comma, added font `huge`, added render method, added candy color
* 1.0.1  -  added `chrome` font, fonttest
* 1.0.0  -  refactor, added alignment and line height option, new cli commands, added `simpleBlock`
* 0.0.13 -  fixed `simple3d`
* 0.0.12 -  fixed `simple3d` and added to grunt test
* 0.0.11 -  added `simple3d` font
* 0.0.10 -  added npmignore, added to docs
* 0.0.9  -  added `console` font
* 0.0.8  -  fixed bugs, docs
* 0.0.7  -  changed to settings object
* 0.0.6  -  added `3d` font
* 0.0.5  -  added grunt test
* 0.0.4  -  fixed `simple` font
* 0.0.3  -  fixes, added `simple` font
* 0.0.2  -  fixed paths
* 0.0.1  -  alpha test


## License
Copyright (c) 2018 Dominik Wilkowski. Licensed under the [GNU GPLv2](https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE).
