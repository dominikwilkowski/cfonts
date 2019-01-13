```shell
     ██████╗ ███████╗  ██████╗  ███╗   ██╗ ████████╗ ███████╗
    ██╔════╝ ██╔════╝ ██╔═══██╗ ████╗  ██║ ╚══██╔══╝ ██╔════╝
    ██║      █████╗   ██║   ██║ ██╔██╗ ██║    ██║    ███████╗
    ██║      ██╔══╝   ██║   ██║ ██║╚██╗██║    ██║    ╚════██║
    ╚██████╗ ██║      ╚██████╔╝ ██║ ╚████║    ██║    ███████║
     ╚═════╝ ╚═╝       ╚═════╝  ╚═╝  ╚═══╝    ╚═╝    ╚══════╝
```

![cfont styles](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/example1.png)


<p align="center"><img src="https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/example2.png" alt="api example"></p>
<p align="center"><a href="https://nodei.co/npm/cfonts/"><img src="https://nodei.co/npm/cfonts.png?downloads=true" alt="npm status"></a></p>
<p align="center"><a href="https://travis-ci.org/dominikwilkowski/cfonts"><img src="https://travis-ci.org/dominikwilkowski/cfonts.svg?branch=master" alt="build status"></a></p>

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

|     |     |     |     |             |
|-----|-----|-----|-----|-------------|
| `A` | `O` | `2` | `2` | `=`         |
| `B` | `P` | `3` | `3` | `@`         |
| `C` | `Q` | `4` | `4` | `#`         |
| `D` | `R` | `5` | `5` | `$`         |
| `E` | `S` | `6` | `6` | `%`         |
| `F` | `T` | `7` | `7` | `&`         |
| `G` | `U` | `8` | `8` | `(`         |
| `H` | `V` | `9` | `9` | `)`         |
| `I` | `W` | `!` | `!` | `/`         |
| `J` | `X` | `?` | `?` | `:`         |
| `K` | `Y` | `.` | `.` | `;`         |
| `L` | `Z` | `+` | `+` | `,`         |
| `M` | `0` | `-` | `-` | `'`         |
| `N` | `1` | `_` | `_` | ` ` (space) |


_The `|` character will be replaced with a line break_


## Options

#### -h, --help
Type: `<command>`  
Default value: `none`

This shows a list of all available options.

```shell
$ cfonts --help
```


#### -V, --version
Type: `<command>`  
Default value: `none`

This shows the installed version.

```shell
$ cfonts --version
```


#### text
Type: `<string>`  
Default value: `""`

This is the "textinput" to be converted into a nice font

```shell
$ cfonts "Hello world"
```


#### -f, --font
Type: `<string>`  
Default value: `"block"`

This is the font face you want to use. So far this plugin ships with with following font faces:

```shell
$ cfonts "text" -f "chrome"
```

- `block`       [colors: 2] _(default)_
	![block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/block.png)
- `shade`       [colors: 2]
	![shade font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/shade.png)
- `chrome`      [colors: 3]
	![chrome font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/chrome.png)
- `simple`      [colors: 1]
	![simple font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple.png)
- `simpleBlock` [colors: 1]
	![simple-block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple-block.png)
- `3d`          [colors: 2]
	![3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/3d.png)
- `simple3d`    [colors: 1]
	![simple-3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple-3d.png)
- `huge`        [colors: 2]
	![huge font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/huge.png)
- `console`     [colors: 1]
	![console font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/console.png)


#### -a, --align
Type: `<string>`  
Default value: `"left"`

You can align your text in the terminal with this option. Use the keywords below:

- `left` _(default)_
- `center`
- `right`

```shell
$ cfonts "text" -a "center"
```


#### -c, --colors
Type: `<string list>`  
Default value: `['system']`

With this setting you can set the colors for your font.
Use the below color strings built in by [chalk](https://github.com/sindresorhus/chalk) or a hex color.  
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


#### -b, --background
Type: `<string>`  
Default value: `"transparent"`

With this setting you can set the background colors for the output. Use the below color strings built in by [chalk](https://github.com/sindresorhus/chalk).
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


#### -l, --letter-spacing
Type: `<integer>`  
Default value: `1`

Set this option to widen the space between characters.

```shell
$ cfonts "text" --letter-spacing 2
```


#### -z, --line-height
Type: `<integer>`  
Default value: `1`

Set this option to widen the space between lines.

```shell
$ cfonts "text" --line-height 2
```


#### -s, --spaceless
Type: `<boolen>`  
Default value: `true`

Set this option to false if you don't want the plugin to insert two empty lines on top and on the bottom of the output.

```shell
$ cfonts "text" --spaceless
```


#### -m, --max-length
Type: `<integer>`  
Default value: `0`

This option sets the maximum characters that will be printed on one line. 0 means no max width and the text will break at the edge of the terminal window.

```shell
$ cfonts "text" --max-length 15
```


## Contributing
To build the repo install dependencies via:

```shell
yarn
```

and run the watch to continuously transpile the code.

```shell
npm run watch
```

Please look at the coding style and work with it, not against it ;)


## Test
The package comes with a bunch of [unit tests](https://github.com/dominikwilkowski/cfonts/tree/master/test/unit) and a
[test suite](https://github.com/dominikwilkowski/cfonts/blob/master/test/fonttest.js) for font files.

Run the unit tests via:

```shell
npm test:unit
```

Run the font test suite via:

```shell
npm test:fonts
```

This tool checks:
- the existence of the font
- all attributes of a font
- each character for:
	- existence
	- consistent width
	- consistent lines

Or run all tests via:

```shell
npm run test
```


## Release History
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
Copyright (c) 2018 Dominik Wilkowski. Licensed under the [GNU GPLv2](https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE).
