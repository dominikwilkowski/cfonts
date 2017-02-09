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
 **************************************************************************************************************************************************************/

'use strict';

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Dependencies
//--------------------------------------------------------------------------------------------------------------------------------------------------------------

var _repeat = require('babel-runtime/core-js/string/repeat');

var _repeat2 = _interopRequireDefault(_repeat);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var ChangeCase = require('change-case');
var WinSize = require('window-size');
var Chalk = require('chalk');
var Fs = require('fs');

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Constructor
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
var CFonts = function () {
	//constructor factory

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// GetFont, Get and set a selected JSON font-file object into global namespace
	//
	// @param  font  {string}  The name of the font to be returned
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var GetFont = function GetFont(font) {
		CFonts.debugging.report('Running GetFont', 1);

		//try loading the font file
		try {
			var fontFile = __dirname + '/fonts/' + font + '.json'; //build font path
			var FONTFACE = JSON.parse(Fs.readFileSync(fontFile, 'utf8')); //read font file

			CFonts.debugging.report('GetFont: Fontface path selected: "' + fontFile + '"', 2);

			CFonts.FONTFACE = FONTFACE;
		} catch (error) {
			CFonts.debugging.error('Font file for "' + font + '" errored out: ' + error, 2);

			CFonts.log.error('Font file for "' + font + '" failed to connect to us.\nTry reinstalling this package.');

			process.exit(1); //exit program with failure code
		}
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// CharLength, return the max width of a character by looking at its longest line
	//
	// @param  character  {array}    The character array from the font face object
	//
	// @return            {integer}  The length of a longest line in a charater
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var CharLength = function CharLength(character) {
		CFonts.debugging.report('Running CharLength', 1);

		var charWidth = 0;

		for (var i = 0; i < CFonts.FONTFACE.lines; i++) {
			var char = character[i].replace(/(<([^>]+)>)/ig, ''); //get character and strip color infos

			if (char.length > charWidth) {
				charWidth = char.length; //assign only largest
			}
		};

		if (charWidth === 0 && CFonts.OPTIONS.letterSpacing > 0) {
			CFonts.debugging.report('CharLength: Adding space to letter spacing', 1);

			charWidth = 1;
		}

		return charWidth;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// AddLine, Add a new line to the output array
	//
	// @param  output    {array}   The output array the line shall be appended to
	//
	// @return           {array}   The output array with new line
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var AddLine = function AddLine(output) {
		CFonts.debugging.report('Running AddLine', 1);

		var lineHeight = CFonts.OPTIONS.lineHeight;
		if (output.length === 0) {
			lineHeight = 0;
		}

		var lines = CFonts.FONTFACE.lines + output.length + lineHeight;
		var length = output.length;

		for (var i = length; i < lines; i++) {
			var index = i - length;

			if (index > lineHeight) {
				output[i] = CFonts.FONTFACE.buffer[index - lineHeight];
			} else {
				output[i] = '';
			}
		}

		return output;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// AddChar, Add a new character to the output array
	//
	// @param  CHAR      {string}  The character to be added
	// @param  output    {array}   The output array the line shall be appended to
	//
	// @return           {array}   The output array with new line
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var AddChar = function AddChar(CHAR, output) {
		CFonts.debugging.report('Running AddChar with "' + CHAR + '"', 1);

		var lines = output.length - CFonts.FONTFACE.lines; //last line is CFonts.FONTFACE.lines tall and is located at the bottom of the output array

		for (var i = lines; i < output.length; i++) {
			//iterate over last line
			var index = i - lines;

			output[i] += Colorize(CFonts.FONTFACE.chars[CHAR][index]);
		}

		return output;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// AddLetterSpacing, Add letter spacing for the next character
	//
	// @param  output    {array}   The output array the line shall be appended to
	//
	// @return           {array}   The output array with space
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var AddLetterSpacing = function AddLetterSpacing(output) {
		CFonts.debugging.report('Running AddLetterSpacing', 1);

		var lines = output.length - CFonts.FONTFACE.lines; //last line is CFonts.FONTFACE.lines tall and is located at the bottom of the output array

		for (var i = lines; i < output.length; i++) {
			//iterate over last line
			var index = i - lines;
			var space = Colorize(CFonts.FONTFACE.letterspace[index]);

			if (space.length === 0 && CFonts.OPTIONS.letterSpacing > 0) {
				CFonts.debugging.report('AddLetterSpacing: Adding space to letter spacing', 1);

				space = ' ';
			}

			output[i] += (0, _repeat2.default)(space, CFonts.OPTIONS.letterSpacing);
		}

		return output;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// Colorize, replace placeholders with color information
	//
	// @param  character  {string}  The string to be converted
	//
	// @return            {string}  The character with color information for CLI
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var Colorize = function Colorize(character) {
		CFonts.debugging.report('Running Colorize', 1);

		var candyColors = ['red', 'green', 'yellow', 'magenta', 'cyan']; //allowed candy colors

		if (character !== undefined) {
			if (CFonts.FONTFACE.colors > 1) {
				for (var i = 0; i < CFonts.FONTFACE.colors; i++) {
					//convert all colors
					var open = new RegExp('<c' + (i + 1) + '>', 'g');
					var close = new RegExp('</c' + (i + 1) + '>', 'g');

					var color = CFonts.OPTIONS.colors[i] || 'white';

					if (color === 'candy') {
						color = candyColors[Math.floor(Math.random() * candyColors.length)];
					}

					character = character.replace(open, Chalk.styles[color.toLowerCase()].open);
					character = character.replace(close, Chalk.styles[color.toLowerCase()].close);
				}
			}

			if (CFonts.FONTFACE.colors === 1) {
				var _color = CFonts.OPTIONS.colors[0] || 'white';

				if (_color === 'candy') {
					_color = candyColors[Math.floor(Math.random() * candyColors.length)];
				}

				character = Chalk.styles[_color.toLowerCase()].open + character + Chalk.styles[_color.toLowerCase()].close;
			}
		}

		return character;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	// Private function
	// AlignText, calculate the spaces to be added to the left of each line to align them either center or right
	//
	// @param  output      {array}    The output array the line shall be appended to
	// @param  lineLength  {integer}  the current line length
	//
	// @return             {array}    The output array with space added on the left for alignment
	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	var AlignText = function AlignText(output, lineLength) {
		CFonts.debugging.report('Running AlignText', 1);

		var space = 0;

		if (CFonts.OPTIONS.align === 'center') {
			//calculate the size for center alignment
			space = Math.floor((WinSize.width - lineLength) / 2);

			CFonts.debugging.report('AlignText: Center lineLength: ' + lineLength + ', WinSize.width: ' + WinSize.width + ', space: ' + space, 2);
		}

		if (CFonts.OPTIONS.align === 'right') {
			//calculate the size for right alignment
			space = WinSize.width - lineLength;

			CFonts.debugging.report('AlignText: Right lineLength: ' + lineLength + ', WinSize.width: ' + WinSize.width + ', space: ' + space, 2);
		}

		if (space > 0) {
			//only add if there is something to add
			var lines = output.length - CFonts.FONTFACE.lines; //last line is CFonts.FONTFACE.lines tall and is located at the bottom of the output array
			space = (0, _repeat2.default)(' ', space);

			for (var i = lines; i < output.length; i++) {
				//iterate over last line
				output[i] = space + output[i];
			}
		}

		return output;
	};

	//--------------------------------------------------------------------------------------------------------------------------------------------------------------
	return {
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		// settings
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		DEBUG: false, //Debug setting
		DEBUGLEVEL: 2, //Debug level setting
		COLORS: [//All allowed font colors
		'black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white', 'gray'],
		BGCOLORS: [//All allowed background colors
		'black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white'],
		ALIGNMENT: [//All allowed alignment options
		'left', 'center', 'right'],
		FONTFACES: [//All allowed fonts
		'console', 'block', 'simpleBlock', 'simple', '3d', 'simple3d', 'chrome', 'huge'],
		FONTFACE: {}, //Font face object to be filled with selected fontface
		OPTIONS: {}, //User options


		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		// Public function
		// render, main method to get the ANSI output for a string
		//
		// @param  INPUT     {string}   The string you want to write out
		// @param  SETTINGS  {object}   (optional) Settings object
		//                              font           {string}   Font face, Default 'block'
		//                              align          {string}   Text alignment, Default: 'left'
		//                              colors         {array}    Colors for font, Default: []
		//                              background     {string}   Chalk color string for background, Default 'Black'
		//                              letterSpacing  {integer}  Space between letters, Default: set by selected font face
		//                              lineHeight     {integer}  Space between lines, Default: 1
		//                              space          {boolean}  Output space before and after output, Default: true
		//                              maxLength      {integer}  Maximum amount of characters per line, Default width of console window
		//
		// @return           {string}   CLI output of INPUT to be consoled out
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		render: function render() {
			var INPUT = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : '';
			var SETTINGS = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : {};

			CFonts.debugging.report('Running render', 1);

			var write = ''; //output in a string
			var output = []; //output in an array
			var lines = 0; //count each line

			//SETTINGS
			CFonts.OPTIONS = { //SETTINGS and defaults
				font: SETTINGS.font || 'block',
				align: SETTINGS.align || 'left',
				colors: SETTINGS.colors || [],
				background: ChangeCase.upperCaseFirst(SETTINGS.background) || 'Black',
				letterSpacing: SETTINGS.letterSpacing === undefined ? 1 : SETTINGS.letterSpacing,
				lineHeight: SETTINGS.lineHeight === undefined ? 1 : parseInt(SETTINGS.lineHeight),
				space: SETTINGS.space === undefined ? true : SETTINGS.space,
				maxLength: SETTINGS.maxLength || 0
			};

			//CHECKING INPUT
			if (INPUT === undefined || INPUT === '') {
				CFonts.log.error('Please provide text to convert');

				process.exit(1); //exit program with failure code
			}

			//CHECKING FONT
			if (CFonts.FONTFACES.indexOf(CFonts.OPTIONS.font) === -1) {
				CFonts.log.error('"' + Chalk.red(SETTINGS.font) + '" is not a valid font option.\n' + ('Please use a font from the supported stack:\n' + Chalk.green('[ ' + CFonts.FONTFACES.join(' | ') + ' ]')));

				process.exit(1); //exit program with failure code
			}

			//CHECKING COLORS
			for (var color in CFonts.OPTIONS.colors) {
				//check color usage
				if (CFonts.COLORS.indexOf(CFonts.OPTIONS.colors[color]) === -1 && CFonts.OPTIONS.colors[color] !== 'candy') {
					CFonts.log.error('"' + Chalk.red(CFonts.OPTIONS.colors[color]) + '" is not a valid font color option.\n' + ('Please use a color from the supported stack:\n' + Chalk.green('[ ' + CFonts.COLORS.join(' | ') + ' | candy ]')));

					process.exit(1); //exit program with failure code
				}
			}

			//CHECKING BACKGROUND COLORS
			if (CFonts.BGCOLORS.indexOf(CFonts.OPTIONS.background.toLowerCase()) === -1) {
				CFonts.log.error('"' + Chalk.red(CFonts.OPTIONS.background) + '" is not a valid background option.\n' + ('Please use a color from the supported stack:\n' + Chalk.green('[ ' + CFonts.BGCOLORS.join(' | ') + ' ]')));

				process.exit(1); //exit program with failure code
			}

			//CHECKING ALIGNMENT
			if (CFonts.ALIGNMENT.indexOf(CFonts.OPTIONS.align) === -1) {
				CFonts.log.error('"' + Chalk.red(CFonts.OPTIONS.align) + '" is not a valid alignment option.\n' + ('Please use an alignment option from the supported stack:\n' + Chalk.green('[ ' + CFonts.ALIGNMENT.join(' | ') + ' ]')));

				process.exit(1); //exit program with failure code
			}

			//DEBUG
			if (CFonts.DEBUG) {
				//log options
				var outOption = 'OPTIONS:\n  Text: ' + INPUT;

				for (var key in CFonts.OPTIONS) {
					outOption += '\n  Options.' + key + ': ' + CFonts.OPTIONS[key];
				}

				CFonts.debugging.report(outOption, 2);
			}

			if (CFonts.OPTIONS.font === 'console') {
				//console fontface is pretty easy to process
				var outputLines = INPUT.replace('\\', '').split('|'); //remove escape characters and split into each line

				CFonts.FONTFACE.colors = 1; //console defaults
				CFonts.FONTFACE.lines = 1;

				for (var line in outputLines) {
					//each line needs to be pushed into the output array
					lines += Math.ceil(outputLines[line].length / WinSize.width); //count each line even when they overflow

					if (CFonts.OPTIONS.colors[0] === "candy") {
						//if the color is candy
						var character = '';

						for (var i = 0; i < outputLines[line].length; i++) {
							//iterate through the message
							character += Colorize(outputLines[line][i]); //and colorize each character individually
						}

						output.push(character); //push each line to the output array
					} else {
						output.push(Colorize(outputLines[line])); //colorize line
					}

					output = AlignText(output, outputLines[line].length); //calculate alignment based on lineLength
				}
			} else {
				//all other fontfaces need the font-file and some more work
				GetFont(CFonts.OPTIONS.font); //get fontface object and make it global

				//setting the letterspacing preference from font face if there is no user overwrite
				if (SETTINGS.letterSpacing === undefined) {
					CFonts.debugging.report('Looking up letter spacing from font face', 1);

					var width = 0;

					for (var _i in CFonts.FONTFACE.letterspace) {
						var char = CFonts.FONTFACE.letterspace[_i].replace(/(<([^>]+)>)/ig, ''); //get character and strip color infos

						if (width < char.length) {
							width = char.length;
						}
					}

					CFonts.debugging.report('Letter spacing set to font face default: "' + width + '"', 2);
					CFonts.OPTIONS.letterSpacing = width;
				}

				var lineLength = CharLength(CFonts.FONTFACE.buffer); //count each output character per line and start with the buffer
				var maxChars = 0; //count each character we print for maxLength option

				output = AddLine([]); //create first lines with buffer
				lines++;

				output = AddLetterSpacing(output); //add letter spacing to the beginning
				lineLength += CharLength(CFonts.FONTFACE.letterspace) * CFonts.OPTIONS.letterSpacing; //count the space for the letter spacing

				for (var _i2 = 0; _i2 < INPUT.length; _i2++) {
					//iterate through the message

					var CHAR = INPUT.charAt(_i2).toUpperCase(); //the current character we convert, only upper case is supported at this time

					if (CFonts.FONTFACE.chars[CHAR] === undefined && CHAR !== '|') {
						//make sure this character exists in the font
						CFonts.debugging.error('Character not found in font: "' + CHAR + '"', 2); //fail silently
					} else {
						CFonts.debugging.report('Character found in font: "' + CHAR + '"', 2);

						var lastLineLength = lineLength; //we need the lineLength for alignment before we look up if the next char fits

						if (CHAR !== '|') {
							//what will the line length be if we add the next char?
							lineLength += CharLength(CFonts.FONTFACE.chars[CHAR]); //get the length of this character
							lineLength += CharLength(CFonts.FONTFACE.letterspace) * CFonts.OPTIONS.letterSpacing; //new line, new line length
						}

						//jump to next line after OPTIONS.maxLength characters or when line break is found or the console windows would has ran out of space
						if (maxChars >= CFonts.OPTIONS.maxLength && CFonts.OPTIONS.maxLength != 0 || CHAR === '|' || lineLength > WinSize.width) {
							lines++;

							CFonts.debugging.report('NEWLINE: maxChars: ' + maxChars + ', ' + ('CFonts.OPTIONS.maxLength: ' + CFonts.OPTIONS.maxLength + ', ') + ('CHAR: ' + CHAR + ', ') + ('lineLength: ' + lineLength + ', ') + ('WinSize.width: ' + WinSize.width + ' '), 2);

							output = AlignText(output, lastLineLength); //calculate alignment based on lineLength

							lineLength = CharLength(CFonts.FONTFACE.buffer); //new line: new line length
							lineLength += CharLength(CFonts.FONTFACE.letterspace) * CFonts.OPTIONS.letterSpacing; //each new line starts with letter spacing
							if (CHAR !== '|') {
								//if this is a character
								lineLength += CharLength(CFonts.FONTFACE.chars[CHAR]); //get the length of this character
								lineLength += CharLength(CFonts.FONTFACE.letterspace) * CFonts.OPTIONS.letterSpacing; //add letter spacing at the end
							}
							maxChars = 0; //new line, new maxLength goal

							output = AddLine(output); //adding new line
							output = AddLetterSpacing(output); //add letter spacing to the beginning
						}

						CFonts.debugging.report('lineLength at: "' + lineLength + '"', 2);

						if (CHAR !== '|') {
							maxChars++; //counting all printed characters
							output = AddChar(CHAR, output); //add new character
							output = AddLetterSpacing(output); //add letter spacing
						}
					}
				}

				output = AlignText(output, lineLength); //alignment last line
			}

			write = output.join('\n'); //convert to a string


			if (CFonts.FONTFACE.colors <= 1) {
				//add text color if only one
				write = Colorize(write);
			}

			if (CFonts.OPTIONS.space) {
				//add space
				write = '\n\n' + write + '\n\n';
			} else {
				write = '\n' + write;
			}

			write = Chalk['bg' + CFonts.OPTIONS.background](write); //result in one string


			return {
				string: write,
				array: output,
				lines: lines,
				options: CFonts.OPTIONS
			};
		},

		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		// Public function
		// say, print to console
		//
		// @param same as render method
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		say: function say() {
			var INPUT = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : '';
			var SETTINGS = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : {};

			CFonts.debugging.report('Running say', 1);

			var write = CFonts.render(INPUT, SETTINGS);

			console.log(write.string); //write out
		},

		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		// Debugging prettiness
		//
		// debugging, Print debug message that will be logged to console.
		//
		// @method  headline                      Return a headline preferably at the beginning of your app
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//
		// @method  report                        Return a message to report starting a process
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//
		// @method  error                         Return a message to report an error
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//
		// @method  interaction                   Return a message to report an interaction
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//
		// @method  send                          Return a message to report data has been sent
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//
		// @method  received                      Return a message to report data has been received
		//          @param    [text]   {string}   The sting you want to log
		//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
		//          @return   [ansi]   {output}
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		debugging: {

			headline: function headline(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.bold(' \u2611  ') + ' ' + text));
				}
			},

			report: function report(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.bold.green(' \u2611  ') + ' ' + Chalk.black(text + ' ')));
				}
			},

			error: function error(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.red(' \u2612  ') + ' ' + Chalk.black(text + ' ')));
				}
			},

			interaction: function interaction(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.blue(' \u261C  ') + ' ' + Chalk.black(text + ' ')));
				}
			},

			send: function send(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.bold.cyan(' \u219D  ') + ' ' + Chalk.black(text + ' ')));
				}
			},

			received: function received(text) {
				var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;

				if (CFonts.DEBUG && level >= CFonts.DEBUGLEVEL) {
					console.log(Chalk.bgWhite('\n' + Chalk.bold.cyan(' \u219C  ') + ' ' + Chalk.black(text + ' ')));
				}
			}
		},

		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		// Logging prettiness
		//
		// log, Print error message to console.
		//
		// @method  error                       Log an error out to the user
		//          @param    [text]  {string}  The sting you want to log
		//          @return   [ansi]  {output}
		//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		log: {
			error: function error(text) {
				text = text.replace(/(?:\r\n|\r|\n)/g, '\n       '); //indent each line

				console.log('\n ' + Chalk.bold.red('Ouch:') + ' ' + text + '\n');
			}
		}
	};
}();

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Module export
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
module.exports = CFonts;
