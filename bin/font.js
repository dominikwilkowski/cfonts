#!/usr/bin/env node

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

var _stringify = require('babel-runtime/core-js/json/stringify');

var _stringify2 = _interopRequireDefault(_stringify);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var Fs = require('fs');
var Chalk = require('chalk');
var Program = require('commander');
var CFonts = require('./../index.js');

var Package = JSON.parse(Fs.readFileSync(__dirname + '/../package.json', 'utf8'));
var Version = Package.version;

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Setting up command line tool
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
Program.usage('"<value>" [option1] <input1> [option2] <input1>,<input2> [option3]\n' + ('  Example: $ ' + Chalk.bold('cfonts "sexy font" -f chrome -a center -c red,green,gray'))).description('This is a tool for sexy fonts in the console. Give your cli some love.').version('v' + Version).option('-f, --font            <keyword>', 'define "font face"', 'block').option('-a, --align           <keyword>', 'define "alignment" for the text', 'left').option('-c, --colors          <keyword>,<keyword>...', 'provide colors for text; comma separated (no space)', 'white').option('-b, --background      <keyword>', 'provide background color', 'Black').option('-l, --letter-spacing  <n>', 'define letter spacing {integer}').option('-z, --line-height     <n>', 'define line height {integer}', 1).option('-s, --spaceless', 'surpress space on top and on the bottom').option('-m, --max-length     <keyword>', 'define how many character can be on one line').action(function (text) {
	Program.text = text; //add flagless option for text
}).on('--help', function () {
	//adding options for each keyword section
	console.log(Chalk.bold('  Font face options:'));
	console.log('  [ ' + CFonts.FONTFACES.join(', ') + ' ]\n');

	console.log(Chalk.bold('  Alignment options:'));
	console.log('  [ ' + CFonts.ALIGNMENT.join(', ') + ' ]\n');

	console.log(Chalk.bold('  Color options:'));
	console.log('  [ ' + CFonts.COLORS.join(', ') + ' ]\n');

	console.log(Chalk.bold('  background color options:'));
	console.log('  [ ' + CFonts.BGCOLORS.join(', ') + ' ]\n');
}).parse(process.argv);

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Execute program
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
if (Program.text !== undefined) {
	//log OPTIONS for debugging
	if (CFonts.DEBUG) {
		CFonts.debugging.report('OPTIONS:\n' + ('  CFonts.say("' + Program.text + '", {\n') + ('\t\'font\': "' + Program.font + '",\n') + ('\t\'align\': "' + Program.align + '",\n') + ('\t\'colors\': ' + (Program.colors ? (0, _stringify2.default)(Program.colors.split(',')) : []) + ',\n') + ('\t\'background\': "' + Program.background + '",\n') + ('\t\'letterSpacing\': ' + Program.letterSpacing + ',\n') + ('\t\'lineHeight\': ' + Program.lineHeight + ',\n') + ('\t\'space\': ' + (Program.spaceless ? false : true) + ',\n') + ('\t\'maxLength\': ' + Program.maxLength + '\n') + '  });', 3);
	}

	//execute cfonts
	CFonts.say(Program.text, {
		'font': Program.font,
		'align': Program.align,
		'colors': Program.colors ? Program.colors.split(',') : [],
		'background': Program.background,
		'letterSpacing': Program.letterSpacing,
		'lineHeight': Program.lineHeight,
		'space': Program.spaceless ? false : true,
		'maxLength': Program.maxLength
	});
} else {
	//we do need text to convert
	CFonts.log.error('Please provide text to convert with ' + Chalk.green('cfonts "Text"') + '\n' + ('Run ' + Chalk.green('cfonts --help') + ' for more infos'));
}
