```shell
                 ██████╗ ███████╗  ██████╗  ███╗   ██╗ ████████╗ ███████╗
                ██╔════╝ ██╔════╝ ██╔═══██╗ ████╗  ██║ ╚══██╔══╝ ██╔════╝
                ██║      █████╗   ██║   ██║ ██╔██╗ ██║    ██║    ███████╗
                ██║      ██╔══╝   ██║   ██║ ██║╚██╗██║    ██║    ╚════██║
                ╚██████╗ ██║      ╚██████╔╝ ██║ ╚████║    ██║    ███████║
                 ╚═════╝ ╚═╝       ╚═════╝  ╚═╝  ╚═══╝    ╚═╝    ╚══════╝
```

[![NPM](https://nodei.co/npm/cfonts.png?downloads=true)](https://nodei.co/npm/cfonts/)


> This is a silly little command line tool for sexy fonts in the console.

## Examples

Different styles:

(https://github.com/dominikwilkowski/cfonts/blob/master/img/example1.png)

Real world example:

(https://github.com/dominikwilkowski/cfonts/blob/master/img/example2.png)


## Installing

To install the CLI app, simply NPM install it globally.

```shell
$ sudo npm install cfonts -g
```

To use it in your project:

```js
var FONTS = require('cfonts');

var fonts = new FONTS(
	text, //text to be converted
	font, //define the font face
	colors, //define all colors
	background, //define the background color
	letterSpacing, //define letter spacing
	space, //define if the output text should have empty lines on top and on the bottom
	maxLength //define how many character can be on one line
);
```

_Remember to escape the `!` character with `\` in the shell_


## Usage

Using the CLI is easy.

```
Usage: cfonts  <command> <param1>,<param2> <command> <param1> etc...
```

At any point you can run the *help* command to get a full list of commands and
how to use them.

```shell
$ cfonts --help
```


## Supported Characters

- A
- B
- C
- D
- E
- F
- G
- H
- I
- J
- K
- L
- M
- N
- O
- P
- Q
- R
- S
- T
- U
- V
- W
- X
- Y
- Z
- 0
- 1
- 2
- 3
- 4
- 5
- 6
- 7
- 8
- 9
- !
- ?
- .
- +
- -
- _
- =
- @
- #
- $
- %
- &
- (
- )
- /
- :
- ;
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


#### -t, --text
Type: `<string>`  
Default value: `""`

This is the "textinput" to be converted into a nice font

```shell
$ cfonts --text "Hello world!"
```


#### -f, --font
Type: `<string>`  
Default value: `"block"`

This is the font face you want to use. So far this plugin ships with with following font faces:

* `block`  [colors: 2]
* `simple` [colors: 1]

```shell
$ cfonts --font "block"
```


#### -c, --colors
Type: `<string list>`  
Default value: `[]`

In this setting you can set the colors for each font face. Use the below color strings build in by [chalk](https://github.com/sindresorhus/chalk).
Provide colors in a comma-separated string, eg: `red,blue`

- `black`
- `red`
- `green`
- `yellow`
- `blue`
- `magenta`
- `cyan`
- `white`
- `gray`

```shell
$ cfonts --colors white,blue
```


#### -b, --background
Type: `<string>`  
Default value: `"Black"`

In this setting you can set the background colors for the output. Use the below color strings build in by [chalk](https://github.com/sindresorhus/chalk).
Provide the background color from the below supported list, eg: 'white'

- `Black`
- `Red`
- `Green`
- `Yellow`
- `Blue`
- `Magenta`
- `Cyan`
- `White`

```shell
$ cfonts --background "Green"
```


#### -l, --letter-spacing
Type: `<integer>`  
Default value: `1`

Set this option to widen the space between characters.

```shell
$ cfonts --letter-spacing 2
```


#### -s, --space
Type: `<boolen>`  
Default value: `true`

Set this option to false if you don't want the plugin to insert two empty lines on top and on the bottom of the output.

```shell
$ cfonts --space false
```


#### -m, --max-length
Type: `<integer>`  
Default value: `10`

This option sets the maximum characters that will be printed on one line. As the shell usually doesn't give you access to its width, this is nessesary
to not end up with a scrambled text output.

```shell
$ cfonts --max-length 15
```


## Contributing
Please look at the coding style and work with it, not against it ;)


## Release History
* 0.0.4 - fixed `simple` font
* 0.0.3 - fixes, added new font
* 0.0.2 - fixed paths
* 0.0.1 - alpha test


## License
Copyright (c) 2015 Dominik Wilkowski. Licensed under the [MIT license](https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE).