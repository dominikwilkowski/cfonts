```shell
     ██████╗ ███████╗  ██████╗  ███╗   ██╗ ████████╗ ███████╗
    ██╔════╝ ██╔════╝ ██╔═══██╗ ████╗  ██║ ╚══██╔══╝ ██╔════╝
    ██║      █████╗   ██║   ██║ ██╔██╗ ██║    ██║    ███████╗
    ██║      ██╔══╝   ██║   ██║ ██║╚██╗██║    ██║    ╚════██║
    ╚██████╗ ██║      ╚██████╔╝ ██║ ╚████║    ██║    ███████║
     ╚═════╝ ╚═╝       ╚═════╝  ╚═╝  ╚═══╝    ╚═╝    ╚══════╝
```

[![NPM](https://nodei.co/npm/cfonts.png?downloads=true)](https://nodei.co/npm/cfonts/)


> This is a silly little command line tool for sexy fonts in the console. **Give your cli some love.**

## Examples

Different styles:

![cfont styles](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/example1.png)

Real world example:

![api example](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/example2.png)


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
	font: 'block',              //define the font face
	align: 'left',              //define text alignment
	colors: ['system'],         //define all colors
	background: 'transparent',  //define the background color, you can also use `backgroundColor` here as key
	letterSpacing: 1,           //define letter spacing
	lineHeight: 1,              //define the line height
	space: true,                //define if the output text should have empty lines on top and on the bottom
	maxLength: '0',             //define how many character can be on one line
});
```

_All settings are optional and shown here with their default_

You can use CFonts in your project without the direct output to the console:

```js
const CFonts = require('cfonts');

const prettyFont = CFonts.render('Hello|world!', {/* same settings object as above */});

prettyFont.string //the ansi string for sexy console font
prettyFont.array //returns the array for the output
prettyFont.lines //returns the lines used
prettyFont.options //returns the options used
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

- `A`
- `B`
- `C`
- `D`
- `E`
- `F`
- `G`
- `H`
- `I`
- `J`
- `K`
- `L`
- `M`
- `N`
- `O`
- `P`
- `Q`
- `R`
- `S`
- `T`
- `U`
- `V`
- `W`
- `X`
- `Y`
- `Z`
- `0`
- `1`
- `2`
- `3`
- `4`
- `5`
- `6`
- `7`
- `8`
- `9`
- `!`
- `?`
- `.`
- `+`
- `-`
- `_`
- `=`
- `@`
- `#`
- `$`
- `%`
- `&`
- `(`
- `)`
- `/`
- `:`
- `;`
- `,`
- ` ` (space)

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
$ cfonts "text" -f "console"
```

- `console`     [colors: 1]
	![console font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/console.png)
- `block`       [colors: 2] _(default)_
	![block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/block.png)
- `simple`      [colors: 1]
	![simple font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple.png)
- `simpleBlock` [colors: 1]
	![simple-block font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple-block.png)
- `3d`          [colors: 2]
	![3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/3d.png)
- `simple3d`    [colors: 1]
	![simple-3d font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/simple-3d.png)
- `chrome`      [colors: 3]
	![chrome font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/chrome.png)
- `huge`        [colors: 2]
	![huge font style](https://raw.githubusercontent.com/dominikwilkowski/cfonts/master/img/huge.png)


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

With this setting you can set the colors for your font. Use the below color strings built in by [chalk](https://github.com/sindresorhus/chalk).
Provide colors in a comma-separated string, eg: `red,blue` _(no spaces)_
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

```shell
$ cfonts "text" --colors white,blue
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
The package comes with a testing tool for the font json format. Run the test with:

```shell
npm test
```

This tool checks:
- the existence
- all attributes of a font
- each character for:
	- existence
	- consistent width
	- consistent lines


## Release History
* 1.2.0  -  Added `transparent` and `system` as default background and color option, added `backgroundColor` as alias for `background`, upgraded deps
* 1.1.3  -  Fixed help text, removing old -t option
* 1.1.2  -  Fixed issue with older commander version #3, updated docs
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
