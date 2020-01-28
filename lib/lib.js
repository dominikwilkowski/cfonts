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
 * Constants:
 *   DEBUG
 *   DEBUGLEVEL
 *   CHARS
 *   COLORS
 *   BGCOLORS
 *   ALIGNMENT
 *   FONTFACES
 *   CLIOPTIONS
 *   PACKAGE
 *   HEXTEST
 *
 * Methods:
 *   GetFont          - Get a selected JSON font-file object
 *   CharLength       - Return the max width of a character by looking at its longest line
 *   AddLine          - Add a new line to the output array
 *   Color            - Abstraction for coloring hex-, keyword- and background-colors
 *   Colorize         - Replace placeholders with color information
 *   UpperCaseFirst   - Upper case the first character of an input string
 *   AddChar          - Add a new character to the output array
 *   AddLetterSpacing - Add letter spacing for the next character
 *   Size             - Abstraction for windows size
 *   AlignText        - Calculate the spaces to be added to the left of each line to align them either center or right
 *   CheckInput       - Check input for human errors
 *   RenderConsole    - Render our input with the console font
 *   CleanInput       - Filter only allowed character
 *   GetOptions       - Merge user settings with default options
 *   Render           - Main method to get the ANSI output for a string
 *   Say              - Print to console
 *   AddShortcuts     - Flatten the shortcuts in our cli options object
 *   ParseArgs        - Parse cli arguments into a nice object
 *   DisplayHelp      - Display the help generated from our CLIOPTIONS
 *   DisplayVersion   - Display the version of this package
 *   Cli              - Run cli commands
 *   Debugging        - Debugging prettiness
 *   Log              - Logging prettiness
 *
 **************************************************************************************************************************************************************/
'use strict'; // Dependencies

function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { _defineProperty(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

function _defineProperty(obj, key, value) { if (key in obj) { Object.defineProperty(obj, key, { value: value, enumerable: true, configurable: true, writable: true }); } else { obj[key] = value; } return obj; }

var chalkOriginal = require("chalk");

var WinSize = require('window-size');

var Path = require("path");

var Fs = require("fs"); // We pass on the FORCE_COLOR env var to chalk so we can force it in ci


var Chalk = new chalkOriginal.Instance(_objectSpread({}, process.env.FORCE_COLOR ? {
  level: parseInt(process.env.FORCE_COLOR)
} : null)); // global defaults

var DEBUG = false;
var DEBUGLEVEL = 2;
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
/**
 * Get a selected JSON font-file object
 *
 * @param  {string} font - The name of the font to be returned
 *
 * @return {object}      - The font object of that file
 */

var GetFont = function GetFont(font) {
  Debugging.report("Running GetFont", 1); // try loading the font file

  try {
    var FONTFACE = require("../fonts/".concat(font, ".json")); // read font file


    Debugging.report("GetFont: Fontface path selected: \"".concat(font, ".json\""), 2);
    return FONTFACE;
  } catch (error) {
    Debugging.error("Font file for \"".concat(font, "\" errored out: ").concat(error), 2);
    return false;
  }
};
/**
 * Return the max width of a character by looking at its longest line
 *
 * @param  {array}   character     - The character array from the font face object
 * @param  {integer} fontLines     - The number of lines this font has per character
 * @param  {integer} letterSpacing - The user defined letter spacing
 *
 * @return {integer}               - The length of a longest line in a character
 */


var CharLength = function CharLength(character, fontLines, letterSpacing) {
  Debugging.report("Running CharLength", 1);
  var charWidth = 0;

  for (var i = 0; i < fontLines; i++) {
    var _char = character[i].replace(/(<([^>]+)>)/ig, ''); // get character and strip color infos


    if (_char.length > charWidth) {
      charWidth = _char.length; // assign only largest
    }
  }

  ;

  if (charWidth === 0 && letterSpacing > 0) {
    Debugging.report("CharLength: Adding space to letter spacing", 1);
    charWidth = 1;
  }

  return charWidth;
};
/**
 * Add a new line to the output array
 *
 * @param  {array}   output      - The output array the line shall be appended to
 * @param  {integer} fontLines   - The number of lines this font has per character
 * @param  {array}   FontBuffer  - An array of the space we add at the beginning of each line
 * @param  {integer} lineHeight  - The user defined line height
 *
 * @return {array}               - The output array with new line
 */


var AddLine = function AddLine(output, fontLines, FontBuffer, lineHeight) {
  Debugging.report("Running AddLine", 1);

  if (output.length === 0) {
    lineHeight = 0;
  }

  var lines = fontLines + output.length + lineHeight;
  var length = output.length;

  for (var i = length; i < lines; i++) {
    var index = i - length;

    if (index > lineHeight) {
      output[i] = FontBuffer[index - lineHeight];
    } else {
      output[i] = '';
    }
  }

  return output;
};
/**
 * Abstraction for coloring hex-, keyword- and background-colors
 *
 * @param  {string}  color    - The color to be used
 * @param  {boolean} bg       - Whether this is a background or not
 *
 * @typedef  {object} ReturnObject
 *   @property {string} open  - The open ansi code
 *   @property {string} close - The close ansi code
 *
 * @return {ReturnObject}     - An object with open and close ansi codes
 */


var Color = function Color(color) {
  var bg = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : false;

  // bail early if we use system color
  if (color === 'system' || process.env.FORCE_COLOR == 0) {
    return {
      open: '',
      close: ''
    };
  } // bail if this is a chalk defined color


  if (color.includes('Bright')) {
    if (bg) {
      color = "bg".concat(UpperCaseFirst(color));
    }

    return {
      open: Chalk[color]._styler.open,
      close: Chalk[color]._styler.close
    };
  }

  var kind = HEXTEST.test(color) ? 'hex' : "".concat(bg ? 'bgK' : 'k', "eyword");
  var styles;

  try {
    styles = Chalk[kind](color)._styler;
  } catch (error) {
    Debugging.error("The color ".concat(Chalk.yellow(color), " could not be found. Sorry about this."));
    return {
      open: '',
      close: ''
    };
  }

  return {
    open: styles.open,
    close: styles.close
  };
};
/**
 * Replace placeholders with color information
 *
 * @param  {string}  character    - The string to be converted
 * @param  {integer} fontColors   - The number of allowed colors for this font
 * @param  {array}   optionColors - An array of user defined colors
 *
 * @return {string}               - The character with color ansi escape sequences for CLI
 */


var Colorize = function Colorize(character, fontColors, optionColors) {
  Debugging.report("Running Colorize", 1);
  var candyColors = [// allowed candy colors
  'red', 'green', 'yellow', 'magenta', 'cyan', 'redBright', 'greenBright', 'yellowBright', 'blueBright', 'magentaBright', 'cyanBright'];

  if (character !== undefined) {
    if (fontColors > 1) {
      // we have to replace all color placeholder with ansi escape sequences
      for (var i = 0; i < fontColors; i++) {
        var color = optionColors[i] === 'candy' ? candyColors[Math.floor(Math.random() * candyColors.length)] : optionColors[i] || 'system';

        var _Color = Color(color),
            openNew = _Color.open,
            closeNew = _Color.close;

        var open = new RegExp("<c".concat(i + 1, ">"), 'g');
        var close = new RegExp("</c".concat(i + 1, ">"), 'g');
        character = character.replace(open, openNew);
        character = character.replace(close, closeNew);
      }
    } // if only one color is allowed there won't be any color placeholders in the characters


    if (fontColors === 1) {
      var _color = optionColors[0] === 'candy' ? candyColors[Math.floor(Math.random() * candyColors.length)] : optionColors[0] || 'system';

      var _Color2 = Color(_color),
          _openNew = _Color2.open,
          _closeNew = _Color2.close;

      character = _openNew + character + _closeNew;
    }
  }

  return character;
};
/**
 * Upper case the first character of an input string.
 *
 * @author https://github.com/blakeembrey/change-case/tree/master/packages/upper-case-first
 *
 * @param  {string} input - A string to be converted
 *
 * @return {string}       - A string with the first letter in upper case
 */


var UpperCaseFirst = function UpperCaseFirst(input) {
  return typeof input === 'string' ? input.charAt(0).toUpperCase() + input.substr(1) : input;
};
/**
 * Add a new character to the output array
 *
 * @param  {string}  CHAR       - The character to be added
 * @param  {array}   output     - The output array the line shall be appended to
 * @param  {integer} fontLines  - The number of lines this font has per character
 * @param  {object}  fontChars  - An object with all character arrays
 * @param  {integer} fontColors - The amount of colors allowed for this font
 * @param  {object}  colors     - Our options
 *
 * @return {array}              - The output array with new line
 */


var AddChar = function AddChar(CHAR, output, fontLines, fontChars, fontColors, colors) {
  Debugging.report("Running AddChar with \"".concat(CHAR, "\""), 1);
  var lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

  for (var i = lines; i < output.length; i++) {
    // iterate over last line
    var index = i - lines;
    output[i] += Colorize(fontChars[CHAR][index], fontColors, colors);
  }

  return output;
};
/**
 * Add letter spacing for the next character
 *
 * @param  {array}   output          - The output array the line shall be appended to
 * @param  {integer} fontLines       - The number of lines this font has per character
 * @param  {array}   fontLetterspace - A space between the letters
 * @param  {integer} fontColors      - The amount of colors allowed for this font
 * @param  {array}   colors          - The user defined colors
 * @param  {integer} letterSpacing   - The user defined letter spacing
 *
 * @return {array}                   - The output array with space
 */


var AddLetterSpacing = function AddLetterSpacing(output, fontLines, fontLetterspace, fontColors, colors, letterSpacing) {
  Debugging.report("Running AddLetterSpacing", 1);
  var lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

  for (var i = lines; i < output.length; i++) {
    // iterate over last line
    var index = i - lines;
    var space = Colorize(fontLetterspace[index], fontColors, colors);

    if (space.length === 0 && letterSpacing > 0) {
      Debugging.report("AddLetterSpacing: Adding space to letter spacing", 1);
      space = ' ';
    }

    output[i] += space.repeat(letterSpacing);
  }

  return output;
};
/**
 * Abstraction for windows size
 *
 * @type {object}
 */


var Size = {
  width: WinSize ? WinSize.width > 0 ? WinSize.width : 80 : 80,
  height: WinSize ? WinSize.height > 0 ? WinSize.height : 24 : 24
};
/**
 * Calculate the spaces to be added to the left of each line to align them either center or right
 *
 * @param  {array}   output         - The output array the line shall be appended to
 * @param  {integer} lineLength     - The current line length
 * @param  {integer} characterLines - The amount of line breaks in one character
 * @param  {string}  align          - The alignment of the text, only `center` and `right` will do anything
 * @param  {object}  size           - The size of the terminal as an object, default: Size
 * @param  {integer} size.width     - The width of the terminal
 * @param  {integer} size.height    - The height of the terminal
 *
 * @return {array}                  - The output array with space added on the left for alignment
 */

var AlignText = function AlignText(output, lineLength, characterLines, align) {
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running AlignText", 1);
  var space = 0;

  if (align === 'center') {
    // calculate the size for center alignment
    space = Math.floor((size.width - lineLength) / 2);
    Debugging.report("AlignText: Center lineLength: ".concat(lineLength, ", size.width: ").concat(size.width, ", space: ").concat(space), 2);
  }

  if (align === 'right') {
    // calculate the size for right alignment
    space = size.width - lineLength;
    Debugging.report("AlignText: Right lineLength: ".concat(lineLength, ", size.width: ").concat(size.width, ", space: ").concat(space), 2);
  }

  if (space > 0) {
    // only add if there is something to add
    var lines = output.length - characterLines; // last line is characterLines tall and is located at the bottom of the output array

    space = ' '.repeat(space);

    for (var i = lines; i < output.length; i++) {
      // iterate over last line (which can be several line breaks long)
      output[i] = space + output[i];
    }
  }

  return output;
};
/**
 * Check input for human errors
 *
 * @param  {string} INPUT          - The string you want to write out
 * @param  {string} userFont       - The user specified font
 * @param  {array}  userColors     - The user specified colors
 * @param  {string} userBackground - The user specified background color
 * @param  {string} userAlign      - The user specified alignment option
 * @param  {object} fontfaces      - All allowed fontfaces
 * @param  {object} colors         - All allowed font colors
 * @param  {object} bgcolors       - All allowed background colors
 * @param  {array}  alignment      - All allowed alignments
 *
 * @typedef  {object} ReturnObject
 *   @property {boolean} pass      - Whether the input is valid
 *   @property {string}  message   - Possible error messages
 *
 * @return {ReturnObject}          - An object with error messages and a pass key
 */


var CheckInput = function CheckInput(INPUT, userFont, userColors, userBackground, userAlign, userGradient) {
  var fontfaces = arguments.length > 6 && arguments[6] !== undefined ? arguments[6] : FONTFACES;
  var colors = arguments.length > 7 && arguments[7] !== undefined ? arguments[7] : COLORS;
  var bgcolors = arguments.length > 8 && arguments[8] !== undefined ? arguments[8] : BGCOLORS;
  var alignment = arguments.length > 9 && arguments[9] !== undefined ? arguments[9] : ALIGNMENT;
  var result = {
    message: '',
    pass: true
  }; // checking input

  if (INPUT === undefined || INPUT === '') {
    return {
      message: 'Please provide text to convert',
      pass: false
    };
  } // checking font


  if (Object.keys(fontfaces).indexOf(userFont.toLowerCase()) === -1) {
    return {
      message: "\"".concat(Chalk.red(userFont), "\" is not a valid font option.\n") + "Please use a font from the supported stack:\n".concat(Chalk.green(Object.keys(fontfaces).map(function (font) {
        return fontfaces[font];
      }).join(', '))),
      pass: false
    };
  } // checking colors


  userColors.forEach(function (color) {
    // check color usage
    if (Object.keys(colors).indexOf(color.toLowerCase()) === -1 && color !== 'candy' && !HEXTEST.test(color)) {
      result = {
        message: "\"".concat(Chalk.red(color), "\" is not a valid font color option.\n") + "Please use a color from the supported stack or any valid hex color:\n".concat(Chalk.green("".concat(Object.keys(colors).map(function (color) {
          return colors[color];
        }).join(', '), ", candy, \"#3456ff\", \"#f80\", etc..."))),
        pass: false
      };
    }
  }); // checking background colors

  if (Object.keys(bgcolors).indexOf(userBackground.toLowerCase()) === -1) {
    return {
      message: "\"".concat(Chalk.red(userBackground), "\" is not a valid background option.\n") + "Please use a color from the supported stack:\n".concat(Chalk.green(Object.keys(bgcolors).map(function (bgcolor) {
        return bgcolors[bgcolor];
      }).join(', '))),
      pass: false
    };
  } // CHECKING ALIGNMENT


  if (alignment.indexOf(userAlign.toLowerCase()) === -1) {
    return {
      message: "\"".concat(Chalk.red(userAlign), "\" is not a valid alignment option.\n") + "Please use an alignment option from the supported stack:\n".concat(Chalk.green(alignment.join(' | '))),
      pass: false
    };
  } // CHECKING GRADIENT


  if (userGradient) {
    var gradientColors = userGradient.split(',');

    if (gradientColors.length !== 2) {
      return {
        message: "\"".concat(Chalk.red(userGradient), "\" is not a valid gradient option.\n") + "Please pass in at least two colors.",
        pass: false
      };
    } // check validity of colors


    gradientColors.forEach(function (color) {
      if (Object.keys(colors).indexOf(color.toLowerCase()) === -1 && !HEXTEST.test(color)) {
        result = {
          message: "\"".concat(Chalk.red(color), "\" is not a valid gradient color option.\n") + "Please use a color from the supported stack or any valid hex color:\n".concat(Chalk.green("".concat(Object.keys(colors).map(function (color) {
            return colors[color];
          }).join(', '), ", \"#3456ff\", \"#f80\", etc..."))),
          pass: false
        };
      }
    });
  }

  return result;
};
/**
 * Render our input with the console font
 *
 * @param  {string}  INPUT       - The string you want to write out
 * @param  {object}  OPTIONS     - All user options
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {integer} size.width  - The width of the terminal
 * @param  {integer} size.height - The height of the terminal
 *
 * @typedef  {object} ReturnObject
 *   @property {array}   output  - An array of each line of the output
 *   @property {integer} lines   - The count of line breaks
 *
 * @return {ReturnObject}        - An object with the output and the line breaks
 */


var RenderConsole = function RenderConsole(INPUT, OPTIONS) {
  var size = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : Size;
  var output = [];
  var i = 0; // the defaults need to cramp a little so console doesn't look silly

  OPTIONS.letterSpacing = OPTIONS.letterSpacing <= 1 ? 0 : OPTIONS.letterSpacing - 1;
  OPTIONS.lineHeight = OPTIONS.lineHeight <= 1 ? 0 : OPTIONS.lineHeight - 1;
  var space = '';

  if (OPTIONS.letterSpacing > 0) {
    space = ' '.repeat(OPTIONS.letterSpacing);
  } // we have to add our letter spacing first


  var outputLines = INPUT.replace(/(?:\r\n|\r|\n)/g, '|').split('|').map(function (line) {
    return line.trim().split('').join(space);
  }); // now we check each line for it's length and split them if too long

  while (i < outputLines.length) {
    var line = outputLines[i];

    if (line.length > size.width) {
      outputLines[i] = line.slice(0, size.width).trim();
      outputLines.splice(i + 1, 0, line.slice(size.width).trim());
      line = outputLines[i];
    }

    if (OPTIONS.colors[0] === 'candy') {
      output.push(line.split('').map(function (character) {
        return Colorize(character, 1, OPTIONS.colors);
      }).join(''));
    } else {
      output.push(line);
    }

    output = AlignText(output, line.length, 1, OPTIONS.align, size);
    output = AddLine(output, 0, [''], OPTIONS.lineHeight);
    i++;
  }

  return {
    output: output,
    lines: output.length
  };
};
/**
 * Filter only allowed character
 *
 * @param  {string} INPUT - The input text to be filtered
 * @param  {array}  chars - An array of all allowed characters
 *
 * @return {string}       - The filtered input text
 */


var CleanInput = function CleanInput(INPUT) {
  var chars = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : CHARS;

  if (typeof INPUT === 'string') {
    var clean = INPUT.replace(/(?:\r\n|\r|\n)/g, '|').split('').filter(function (_char2) {
      return chars.includes(_char2.toUpperCase());
    }).join('');
    return clean;
  } else {
    return '';
  }
};
/**
 * Merge user settings with default options
 *
 * @param  {SETTINGS} SETTINGS       - Some or all of the allowed settings
 * @param  {array}     allowedColors - All allowed font colors
 * @param  {array}     allowedBG     - All allowed background colors
 * @param  {array}     allowedFont   - All allowed fontfaces
 *
 * @typedef  {object} SETTINGS
 *   @param  {string}  font          - Font face, Default 'block'
 *   @param  {string}  align         - Text alignment, Default: 'left'
 *   @param  {array}   colors        - Colors for font, Default: []
 *   @param  {string}  background    - Chalk color string for background, Default 'Black'
 *   @param  {integer} letterSpacing - Space between letters, Default: set by selected font face
 *   @param  {integer} lineHeight    - Space between lines, Default: 1
 *   @param  {boolean} space         - Output space before and after output, Default: true
 *   @param  {integer} maxLength     - Maximum amount of characters per line, Default width of console window
 *   @param  {integer} gradient      - Gradient color pair, Default: false
 *
 * @return {object}                  - Our merged options
 */


var GetOptions = function GetOptions(_ref) {
  var font = _ref.font,
      align = _ref.align,
      colors = _ref.colors,
      background = _ref.background,
      backgroundColor = _ref.backgroundColor,
      letterSpacing = _ref.letterSpacing,
      lineHeight = _ref.lineHeight,
      space = _ref.space,
      maxLength = _ref.maxLength,
      gradient = _ref.gradient;
  var allowedColors = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : COLORS;
  var allowedBG = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : BGCOLORS;
  var allowedFont = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : FONTFACES;
  return {
    font: font === undefined ? 'block' : allowedFont[font.toLowerCase()] || font,
    align: align === undefined ? 'left' : align.toLowerCase(),
    colors: Array.isArray(colors) ? colors.map(function (color) {
      return allowedColors[color.toLowerCase()] || color;
    }) : [],
    background: background === undefined && backgroundColor === undefined ? 'transparent' : background === undefined ? allowedBG[backgroundColor.toLowerCase()] || backgroundColor : allowedBG[background.toLowerCase()] || background,
    letterSpacing: typeof parseInt(letterSpacing) === 'number' && letterSpacing > 0 ? letterSpacing : 1,
    lineHeight: lineHeight === undefined ? 1 : parseInt(lineHeight),
    space: typeof space === 'boolean' ? space : true,
    maxLength: maxLength || 0,
    gradient: gradient || false
  };
};
/**
 * Main method to get the ANSI output for a string
 *
 * @param  {string}  input       - The string you want to write out
 * @param  {object}  SETTINGS    - Settings object
 * @param  {boolean} debug       - A flag to enable debug mode
 * @param  {integer} debuglevel  - The debug level we want to show
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {integer} size.width  - The width of the terminal
 * @param  {integer} size.height - The height of the terminal
 *
 * @typedef  {object} ReturnObject
 *   @property {string}  string  - The pure string for output with all line breaks
 *   @property {array}   array   - Each line of output in an array
 *   @property {integer} lines   - The number of lines
 *   @property {object}  options - All options used
 *
 * @return {ReturnObject}        - CLI output of INPUT to be consoled out
 */


var Render = function Render(input) {
  var SETTINGS = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : {};
  var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG;
  var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUGLEVEL;
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running render", 1);
  DEBUG = debug;
  DEBUGLEVEL = debuglevel;
  var INPUT = CleanInput(input, CHARS);
  var OPTIONS = GetOptions(SETTINGS);
  var output = []; // for output where each line is an output line

  var lines = 0; // for counting each line

  var FONTFACE = {}; // scoping the fontface object higher for fonts with just one color

  var _isGoodHuman = CheckInput(INPUT, OPTIONS.font, OPTIONS.colors, OPTIONS.background, OPTIONS.align, OPTIONS.gradient);

  if (!_isGoodHuman.pass) {
    Log.error(_isGoodHuman.message);
    return false;
  } // display an overview of options if debug flag is enabled


  if (DEBUG) {
    var outOption = "OPTIONS:\n  Text: ".concat(INPUT);

    for (var key in OPTIONS) {
      outOption += "\n  Options.".concat(key, ": ").concat(OPTIONS[key]);
    }

    Debugging.report(outOption, 3);
  }

  if (OPTIONS.font === 'console') {
    // console fontface is pretty easy to process
    FONTFACE = {
      colors: 1,
      lines: 1
    };
    var consoleOutput = RenderConsole(INPUT, OPTIONS, size);
    output = consoleOutput.output;
    lines = consoleOutput.lines;
  } else {
    // all other fontfaces need the font-file and some more work
    FONTFACE = GetFont(OPTIONS.font);

    if (!FONTFACE) {
      Log.error("Font file for the font \"".concat(font, "\" could not be found.\nTry reinstalling this package."));
      return false;
    } // setting the letterspacing preference from font face if there is no user overwrite


    if (SETTINGS.letterSpacing === undefined) {
      Debugging.report("Looking up letter spacing from font face", 1);
      var width = 0;
      FONTFACE.letterspace.forEach(function (item) {
        var _char3 = item.replace(/(<([^>]+)>)/ig, ''); // get character and strip color infos


        if (width < _char3.length) {
          width = _char3.length;
        }
      });
      Debugging.report("Letter spacing set to font face default: \"".concat(width, "\""), 2);
      OPTIONS.letterSpacing = width;
    }

    var lineLength = CharLength(FONTFACE.buffer, FONTFACE.lines, OPTIONS); // count each output character per line and start with the buffer

    var maxChars = 0; // count each character we print for maxLength option

    output = AddLine([], FONTFACE.lines, FONTFACE.buffer, OPTIONS.lineHeight); // create first lines with buffer

    lines++;
    output = AddLetterSpacing(output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing); // add letter spacing to the beginning

    lineLength += CharLength(FONTFACE.letterspace, FONTFACE.lines, OPTIONS) * OPTIONS.letterSpacing; // count the space for the letter spacing

    for (var i = 0; i < INPUT.length; i++) {
      // iterate through the message
      var CHAR = INPUT.charAt(i).toUpperCase(); // the current character we convert, only upper case is supported at this time

      var lastLineLength = lineLength; // we need the lineLength for alignment before we look up if the next char fits

      Debugging.report("Character found in font: \"".concat(CHAR, "\""), 2);

      if (CHAR !== "|") {
        // what will the line length be if we add the next char?
        lineLength += CharLength(FONTFACE.chars[CHAR], FONTFACE.lines, OPTIONS); // get the length of this character

        lineLength += CharLength(FONTFACE.letterspace, FONTFACE.lines, OPTIONS) * OPTIONS.letterSpacing; // new line, new line length
      } // jump to next line after OPTIONS.maxLength characters or when line break is found or the console windows would have ran out of space


      if (maxChars >= OPTIONS.maxLength && OPTIONS.maxLength != 0 || CHAR === "|" || lineLength > size.width) {
        lines++;
        Debugging.report("NEWLINE: maxChars: ".concat(maxChars, ", ") + "OPTIONS.maxLength: ".concat(OPTIONS.maxLength, ", ") + "CHAR: ".concat(CHAR, ", ") + "lineLength: ".concat(lineLength, ", ") + "Size.width: ".concat(size.width, " "), 2);
        output = AlignText(output, lastLineLength, FONTFACE.lines, OPTIONS.align, size); // calculate alignment based on lineLength

        lineLength = CharLength(FONTFACE.buffer, FONTFACE.lines, OPTIONS); // new line: new line length

        lineLength += CharLength(FONTFACE.letterspace, FONTFACE.lines, OPTIONS) * OPTIONS.letterSpacing; // each new line starts with letter spacing

        if (CHAR !== "|") {
          // if this is a character and not a line break
          lineLength += CharLength(FONTFACE.chars[CHAR], FONTFACE.lines, OPTIONS); // get the length of this character

          lineLength += CharLength(FONTFACE.letterspace, FONTFACE.lines, OPTIONS) * OPTIONS.letterSpacing; // add letter spacing at the end
        }

        maxChars = 0; // new line, new maxLength goal

        output = AddLine(output, FONTFACE.lines, FONTFACE.buffer, OPTIONS.lineHeight); // adding new line
        // add letter spacing to the beginning

        output = AddLetterSpacing(output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing);
      }

      Debugging.report("lineLength at: \"".concat(lineLength, "\""), 2);

      if (CHAR !== "|") {
        maxChars++; // counting all printed characters

        output = AddChar(CHAR, output, FONTFACE.lines, FONTFACE.chars, FONTFACE.colors, OPTIONS.colors); // add new character

        output = AddLetterSpacing(output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing);
      }
    }

    output = AlignText(output, lineLength, FONTFACE.lines, OPTIONS.align, size); // alignment last line
  }

  var write = output.join("\n");

  if (FONTFACE.colors <= 1) {
    write = Colorize(write, FONTFACE.colors, OPTIONS.colors);
  }

  if (OPTIONS.space) {
    // add space
    write = "\n\n".concat(write, "\n\n");
  }

  if (OPTIONS.background !== 'transparent') {
    var _Color3 = Color(OPTIONS.background, true),
        openNew = _Color3.open,
        closeNew = _Color3.close;

    write = openNew + '\n' + write + closeNew; // result in one string with background
  }

  return {
    string: write,
    array: output,
    lines: lines,
    options: OPTIONS
  };
};
/**
 * Print to console
 *
 * @param same as render method
 */


var Say = function Say(INPUT) {
  var SETTINGS = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : {};
  var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG;
  var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUGLEVEL;
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running say", 1);
  DEBUG = debug;
  DEBUGLEVEL = debuglevel;
  var write = Render(INPUT, SETTINGS, debug, debuglevel, size);

  if (write) {
    console.log(write.string); // write out
  }
};
/**
 * Flatten the shortcuts in our cli options object
 *
 * @param  {object} options - An object objects with a short key
 *
 * @return {object}         - All short keys flattened into first level
 */


var AddShortcuts = function AddShortcuts(options) {
  var flatOptions = Object.assign({}, options);
  Object.keys(flatOptions).forEach(function (option) {
    flatOptions[option]._name = option;
    flatOptions[flatOptions[option]["short"]] = flatOptions[option];
  });
  return flatOptions;
};
/**
 * Parse cli arguments into a nice object
 *
 * @param  {array} inputOptions - All possible options registered for this app
 * @param  {array} inputArgs    - The arguments given to us in our cli, default: process.argv
 *
 * @return {object}             - An object of all options with at least their default values
 */


var ParseArgs = function ParseArgs() {
  var inputOptions = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : CLIOPTIONS;
  var inputArgs = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : process.argv;
  var parsedArgs = {
    text: inputArgs[2]
  }; // create defaults

  Object.keys(inputOptions).forEach(function (option) {
    var name = option.replace('--', '');
    parsedArgs[name] = inputOptions[option]["default"];
  });

  if (inputArgs[2] === '--help' || inputArgs[2] === '-h') {
    parsedArgs.help = true;
  }

  if (inputArgs[2] === '--version' || inputArgs[2] === '-v') {
    parsedArgs.version = true;
  }

  var args = inputArgs.splice(3); // the first two are node specific, the third is our text

  var options = AddShortcuts(inputOptions);

  for (var index = 0; args.length > index; index++) {
    var option = options[args[index]];

    if (option) {
      var name = option._name.replace('--', '');

      if (option.options !== undefined) {
        index++;
        var value = args[index];
        parsedArgs[name] = value;
      } else {
        parsedArgs[name] = true;
      }
    } else {
      Debugging.report("The cli argument ".concat(args[index], " was not found and ignored"), 2);
    }
  }

  ;
  return parsedArgs;
};
/**
 * Display the help generated from our CLIOPTIONS
 */


var DisplayHelp = function DisplayHelp() {
  console.log(" ".concat(Render('cfonts', {
    align: 'left',
    colors: ['redBright', 'greenBright']
  }).string) + "This is a tool for sexy fonts in the console. Give your cli some love.\n\n" + "Usage: cfonts \"<value>\" [option1] <input1> [option2] <input1>,<input2> [option3]\n" + "Example: ".concat(Chalk.bold('$ cfonts "sexy font" -f chrome -a center -c red,green,gray'), "\n\n") + "Options:\n");
  var command = [];
  var largestSize = 0;
  Object.keys(CLIOPTIONS).forEach(function (option) {
    console.log(Chalk.bold("".concat(option, ", ").concat(CLIOPTIONS[option]["short"])));
    console.log(CLIOPTIONS[option].description);
    console.log("".concat(Chalk.bold('$'), " cfonts ").concat(CLIOPTIONS[option].example, "\n"));
  });
};
/**
 * Display the version of this package
 */


var DisplayVersion = function DisplayVersion() {
  console.log(PACKAGE.version);
};
/**
 * Run cli commands
 *
 * @param  {array} inputOptions - All possible options registered for this app
 * @param  {array} inputArgs    - The arguments given to us in our cli, default: process.argv
 */


var Cli = function Cli() {
  var inputOptions = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : CLIOPTIONS;
  var inputArgs = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : process.argv;
  var args = ParseArgs(inputOptions, inputArgs);
  Debugging.report("OPTIONS:\n" + "  CFonts.say(\"".concat(args.text, "\", {\n") + "    font: \"".concat(args.font, "\",\n") + "    align: \"".concat(args.align, "\",\n") + "    colors: ".concat(args.colors ? JSON.stringify(args.colors.split(',')) : [], ",\n") + "    background: \"".concat(args.background, "\",\n") + "    letterSpacing: ".concat(args['letter-spacing'], ",\n") + "    lineHeight: ".concat(args['line-height'], ",\n") + "    space: ".concat(!args.spaceless, ",\n") + "    maxLength: ".concat(args['max-length'], "\n") + "  }, ".concat(args.debug, ", ").concat(args.debugLevel, " );"), 3, args.debug, args.debugLevel);

  if (args.help) {
    DisplayHelp();
    return;
  }

  if (args.version) {
    DisplayVersion();
    return;
  }

  if (!args.text) {
    Log.error("Please provide text to convert with ".concat(Chalk.green("cfonts \"Text\""), "\n") + "Run ".concat(Chalk.green("cfonts --help"), " for more infos"));
    return;
  }

  Say(args.text, {
    font: args.font,
    align: args.align,
    colors: args.colors ? args.colors.split(',') : [],
    background: args.background,
    letterSpacing: args['letter-spacing'],
    lineHeight: args['line-height'],
    space: !args.spaceless,
    maxLength: args['max-length'],
    gradient: args.gradient
  }, args.debug, args.debugLevel);
};
/**
 * Debugging prettiness
 *
 * @type {object}
 */


var Debugging = {
  /**
   * Return a headline preferably at the beginning of your app
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  headline: function headline(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUGLEVEL;

    if (debug && level >= debuglevel) {
      console.log(Chalk.bgWhite("\n".concat(Chalk.bold(" \u2611  "), " ").concat(text)));
    }
  },

  /**
   * Return a message to report starting a process
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  report: function report(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUGLEVEL;

    if (debug && level >= debuglevel) {
      console.log(Chalk.bgWhite("\n".concat(Chalk.bold.green(" \u2611  "), " ").concat(Chalk.black("".concat(text, " ")))));
    }
  },

  /**
   * Return a message to report an error
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  error: function error(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUGLEVEL;

    if (debug && level >= debuglevel) {
      console.error(Chalk.bgWhite("\n".concat(Chalk.red(" \u2612  "), " ").concat(Chalk.black("".concat(text, " ")))));
    }
  }
};
/**
 * Logging prettiness
 *
 * @type {object}
 */

var Log = {
  /**
   * Print error message to console.
   *
   * @param  {string} text - The sting you want to log
   */
  error: function error(text) {
    text = text.replace(/(?:\r\n|\r|\n)/g, '\n       '); // indent each line

    console.error("\n ".concat(Chalk.bold.red('Ouch:'), " ").concat(text, "\n"));
  }
}; // Export for API use and unit tests

module.exports = exports = {
  render: Render,
  say: Say,
  __test__: {
    DEBUG: DEBUG,
    DEBUGLEVEL: DEBUGLEVEL,
    CHARS: CHARS,
    COLORS: COLORS,
    BGCOLORS: BGCOLORS,
    ALIGNMENT: ALIGNMENT,
    FONTFACES: FONTFACES,
    CLIOPTIONS: CLIOPTIONS,
    PACKAGE: PACKAGE,
    AddShortcuts: AddShortcuts,
    GetFont: GetFont,
    CharLength: CharLength,
    AddLine: AddLine,
    Color: Color,
    Colorize: Colorize,
    UpperCaseFirst: UpperCaseFirst,
    AddChar: AddChar,
    AddLetterSpacing: AddLetterSpacing,
    Size: Size,
    AlignText: AlignText,
    CheckInput: CheckInput,
    RenderConsole: RenderConsole,
    CleanInput: CleanInput,
    GetOptions: GetOptions,
    ParseArgs: ParseArgs,
    DisplayHelp: DisplayHelp,
    DisplayVersion: DisplayVersion,
    Cli: Cli,
    Debugging: Debugging,
    Log: Log
  }
};