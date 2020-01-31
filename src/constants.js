/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE  GNU GPLv2
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * Constants
 *   CHARS
 *   COLORS
 *   BGCOLORS
 *   GRADIENTCOLORS
 *   ALIGNMENT
 *   FONTFACES
 *   CLIOPTIONS
 *   PACKAGE
 *   HEXTEST
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Chalk } = require('./Chalk.js');


// global defaults
const CHARS = [
	"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "|",
	"!", "?", ".", "+", "-", "_", "=", "@", "#", "$", "%", "&", "(", ")", "/", ":", ";", ",", " ", "'"
];

const COLORS = {
	system: 'system',
	black: 'black',
	red: 'red',
	green: 'green',
	yellow: 'yellow',
	blue: 'blue',
	magenta: 'magenta',
	cyan: 'cyan',
	white: 'white',
	gray: 'gray',
	redbright: 'redBright',
	greenbright: 'greenBright',
	yellowbright: 'yellowBright',
	bluebright: 'blueBright',
	magentabright: 'magentaBright',
	cyanbright: 'cyanBright',
	whitebright: 'whiteBright',
};

const BGCOLORS = {
	transparent: 'transparent',
	black: 'black',
	red: 'red',
	green: 'green',
	yellow: 'yellow',
	blue: 'blue',
	magenta: 'magenta',
	cyan: 'cyan',
	white: 'white',
	blackbright: 'blackBright',
	redbright: 'redBright',
	greenbright: 'greenBright',
	yellowbright: 'yellowBright',
	bluebright: 'blueBright',
	magentabright: 'magentaBright',
	cyanbright: 'cyanBright',
	whitebright: 'whiteBright',
};

const GRADIENTCOLORS = {
	transparent: 'transparent',
	black: 'black',
	red: 'red',
	green: 'green',
	yellow: 'yellow',
	blue: 'blue',
	magenta: 'magenta',
	cyan: 'cyan',
	white: 'white',
};

const ALIGNMENT = [
	'left',
	'center',
	'right',
];

const FONTFACES = {
	console: 'console',
	block: 'block',
	simpleblock: 'simpleBlock',
	simple: 'simple',
	'3d': '3d',
	simple3d: 'simple3d',
	chrome: 'chrome',
	huge: 'huge',
	shade: 'shade',
};

const CLIOPTIONS = {
	'--version': {
		description: 'Use to display the version of cfonts',
		example: '--version',
		short: '-v',
		default: false,
	},
	'--help': {
		description: 'Use to display this help',
		example: '--help',
		short: '-h',
		default: false,
	},
	'--font': {
		description: 'Use to define the font face',
		example: `--font block ${ Chalk.green(`( ${ Object.keys( FONTFACES ).map( font => FONTFACES[ font ] ).join(', ') } )`) }`,
		short: '-f',
		options: Object.keys( FONTFACES ).map( color => FONTFACES[ color ] ),
		default: 'block',
	},
	'--colors': {
		description: 'Use to define the font color',
		example: `--colors red ${ Chalk.green(`( ${ Object.keys( COLORS ).map( color => COLORS[ color ] ).join(', ') }, #ff8800, hex-colors etc... )`) }`,
		short: '-c',
		options: true,
		default: 'system',
	},
	'--background': {
		description: 'Use to define background color',
		example: `--background blue ${ Chalk.green(`( ${ Object.keys( BGCOLORS ).map( bgcolor => BGCOLORS[ bgcolor ] ).join(', ') } )`) }`,
		short: '-b',
		options: Object.keys( BGCOLORS ).map( color => BGCOLORS[ color ] ),
		default: 'transparent',
	},
	'--align': {
		description: 'Use to align your text output',
		example: `--align ${ Chalk.green(`( ${ ALIGNMENT.join(', ') } )`) }`,
		short: '-a',
		options: ALIGNMENT,
		default: 'left',
	},
	'--letter-spacing': {
		description: 'Use to define your letter spacing',
		example: '--letter-spacing 2',
		short: '-l',
		options: true,
		default: 1,
	},
	'--line-height': {
		description: 'Use to define your line height',
		example: '--line-height 5',
		short: '-z',
		options: true,
		default: 1,
	},
	'--spaceless': {
		description: 'Use to disable the padding around your output',
		example: '--spaceless',
		short: '-s',
		default: false,
	},
	'--max-length': {
		description: 'Use to define the amount of maximum characters per line',
		example: '--max-length 10',
		short: '-m',
		options: true,
		default: 0,
	},
	'--gradient': {
		description: 'Use to define a start and end color of a gradient',
		example: '--gradient red,blue',
		short: '-g',
		options: true,
		default: false,
	},
	'--independentGradient': {
		description: 'Use to define if a gradient is applied independently for each line',
		example: '--gradient red,blue --independentGradient',
		short: '-i',
		default: false,
	},
	'--debug': {
		description: 'Use to enable debug mode',
		example: '--debug',
		short: '-d',
		default: false,
	},
	'--debug-level': {
		description: 'Use to define the debug level. The higher, the less debug infos',
		example: '--debug-level 2',
		short: '-x',
		options: true,
		default: 1,
	},
};

const PACKAGE = require('../package.json');

const HEXTEST = RegExp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$');


module.exports = exports = {
	CHARS,
	COLORS,
	BGCOLORS,
	GRADIENTCOLORS,
	ALIGNMENT,
	FONTFACES,
	CLIOPTIONS,
	PACKAGE,
	HEXTEST,
};
