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
 * Render
 *   Main method to get the ANSI output for a string
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./AddLetterSpacing.js'),
    AddLetterSpacing = _require.AddLetterSpacing;

var _require2 = require('./RenderConsole.js'),
    RenderConsole = _require2.RenderConsole;

var _require3 = require('./Debugging.js'),
    Debugging = _require3.Debugging,
    DEBUG = _require3.DEBUG;

var _require4 = require('./CharLength.js'),
    CharLength = _require4.CharLength;

var _require5 = require('./CheckInput.js'),
    CheckInput = _require5.CheckInput;

var _require6 = require('./CleanInput.js'),
    CleanInput = _require6.CleanInput;

var _require7 = require('./GetOptions.js'),
    GetOptions = _require7.GetOptions;

var _require8 = require('./AlignText.js'),
    AlignText = _require8.AlignText;

var _require9 = require('./Colorize.js'),
    Colorize = _require9.Colorize;

var _require10 = require('./AddLine.js'),
    AddLine = _require10.AddLine;

var _require11 = require('./AddChar.js'),
    AddChar = _require11.AddChar;

var _require12 = require('./GetFont.js'),
    GetFont = _require12.GetFont;

var _require13 = require('./constants.js'),
    CHARS = _require13.CHARS;

var _require14 = require('./Color.js'),
    Color = _require14.Color;

var _require15 = require('./Size.js'),
    Size = _require15.Size;

var _require16 = require('./Log.js'),
    Log = _require16.Log;
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
  var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG.enabled;
  var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUG.level;
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running render", 1);
  DEBUG.enabled = debug;
  DEBUG.level = debuglevel;
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


  if (DEBUG.enabled) {
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
      Log.error("Font file for the font \"".concat(OPTIONS.font, "\" could not be found.\nTry reinstalling this package."));
      return false;
    } // setting the letterspacing preference from font face if there is no user overwrite


    if (SETTINGS.letterSpacing === undefined) {
      Debugging.report("Looking up letter spacing from font face", 1);
      var width = 0;
      FONTFACE.letterspace.forEach(function (item) {
        var _char = item.replace(/(<([^>]+)>)/ig, ''); // get character and strip color infos


        if (width < _char.length) {
          width = _char.length;
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
    var _Color = Color(OPTIONS.background, true),
        openNew = _Color.open,
        closeNew = _Color.close;

    write = openNew + '\n' + write + closeNew; // result in one string with background
  }

  return {
    string: write,
    array: output,
    lines: lines,
    options: OPTIONS
  };
};

module.exports = exports = {
  Render: Render
};