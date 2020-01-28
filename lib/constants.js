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
 *   ALIGNMENT
 *   FONTFACES
 *   CLIOPTIONS
 *   PACKAGE
 *   HEXTEST
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Chalk.js'),
    Chalk = _require.Chalk; // global defaults


var CHARS = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "|", "!", "?", ".", "+", "-", "_", "=", "@", "#", "$", "%", "&", "(", ")", "/", ":", ";", ",", " ", "'"];
var COLORS = {
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
  whitebright: 'whiteBright'
};
var BGCOLORS = {
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
  whitebright: 'whiteBright'
};
var ALIGNMENT = ['left', 'center', 'right'];
var FONTFACES = {
  console: 'console',
  block: 'block',
  simpleblock: 'simpleBlock',
  simple: 'simple',
  '3d': '3d',
  simple3d: 'simple3d',
  chrome: 'chrome',
  huge: 'huge',
  shade: 'shade'
};
var CLIOPTIONS = {
  '--version': {
    description: 'Use to display the version of cfonts',
    example: '--version',
    "short": '-v',
    "default": false
  },
  '--help': {
    description: 'Use to display this help',
    example: '--help',
    "short": '-h',
    "default": false
  },
  '--font': {
    description: 'Use to define the font face',
    example: "--font block ".concat(Chalk.green("( ".concat(Object.keys(FONTFACES).map(function (font) {
      return FONTFACES[font];
    }).join(', '), " )"))),
    "short": '-f',
    options: Object.keys(FONTFACES).map(function (color) {
      return FONTFACES[color];
    }),
    "default": 'block'
  },
  '--colors': {
    description: 'Use to define the font color',
    example: "--colors red ".concat(Chalk.green("( ".concat(Object.keys(COLORS).map(function (color) {
      return COLORS[color];
    }).join(', '), ", #ff8800, hex-colors etc... )"))),
    "short": '-c',
    options: true,
    "default": 'system'
  },
  '--background': {
    description: 'Use to define background color',
    example: "--background blue ".concat(Chalk.green("( ".concat(Object.keys(BGCOLORS).map(function (bgcolor) {
      return BGCOLORS[bgcolor];
    }).join(', '), " )"))),
    "short": '-b',
    options: Object.keys(BGCOLORS).map(function (color) {
      return BGCOLORS[color];
    }),
    "default": 'transparent'
  },
  '--align': {
    description: 'Use to align your text output',
    example: "--align ".concat(Chalk.green("( ".concat(ALIGNMENT.join(', '), " )"))),
    "short": '-a',
    options: ALIGNMENT,
    "default": 'left'
  },
  '--letter-spacing': {
    description: 'Use to define your letter spacing',
    example: '--letter-spacing 2',
    "short": '-l',
    options: true,
    "default": 1
  },
  '--line-height': {
    description: 'Use to define your line height',
    example: '--line-height 5',
    "short": '-z',
    options: true,
    "default": 1
  },
  '--spaceless': {
    description: 'Use to disable the padding around your output',
    example: '--spaceless',
    "short": '-s',
    "default": false
  },
  '--max-length': {
    description: 'Use to define the amount of maximum characters per line',
    example: '--max-length 10',
    "short": '-m',
    options: true,
    "default": 0
  },
  '--gradient': {
    description: 'Use to define a start and end color per option for the chosen font',
    example: '--gradient red',
    "short": '-g',
    options: true,
    "default": false
  },
  '--debug': {
    description: 'Use to enable debug mode',
    example: '--debug',
    "short": '-d',
    "default": false
  },
  '--debug-level': {
    description: 'Use to define the debug level. The higher, the less debug infos',
    example: '--debug-level 2',
    "short": '-x',
    options: true,
    "default": 1
  }
};

var PACKAGE = require('../package.json');

var HEXTEST = RegExp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$');
module.exports = exports = {
  CHARS: CHARS,
  COLORS: COLORS,
  BGCOLORS: BGCOLORS,
  ALIGNMENT: ALIGNMENT,
  FONTFACES: FONTFACES,
  CLIOPTIONS: CLIOPTIONS,
  PACKAGE: PACKAGE,
  HEXTEST: HEXTEST
};