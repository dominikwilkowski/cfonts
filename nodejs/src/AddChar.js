/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * AddChar
 *   Add a new character to the output array
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');
const { Colorize } = require('./Colorize.js');

/**
 * Add a new character to the output array
 *
 * @param  {string}  CHAR       - The character to be added
 * @param  {array}   output     - The output array the line shall be appended to
 * @param  {number}  fontLines  - The number of lines this font has per character
 * @param  {object}  fontChars  - An object with all character arrays
 * @param  {number}  fontColors - The amount of colors allowed for this font
 * @param  {object}  colors     - Our options
 *
 * @return {array}              - The output array with new line
 */
const AddChar = (CHAR, output, fontLines, fontChars, fontColors, colors) => {
	Debugging.report(`Running AddChar with "${CHAR}"`, 1);

	let lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

	for (let i = lines; i < output.length; i++) {
		// iterate over last line
		let index = i - lines;

		output[i] += Colorize(fontChars[CHAR][index], fontColors, colors);
	}

	return output;
};

module.exports = exports = {
	AddChar,
};
