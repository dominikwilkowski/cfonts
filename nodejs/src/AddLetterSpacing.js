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
 * AddLetterSpacing
 *   Add letter spacing for the next character
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');
const { Colorize } = require('./Colorize.js');

/**
 * Add letter spacing for the next character
 *
 * @param  {array}   output          - The output array the line shall be appended to
 * @param  {number}  fontLines       - The number of lines this font has per character
 * @param  {array}   fontLetterspace - A space between the letters
 * @param  {number}  fontColors      - The amount of colors allowed for this font
 * @param  {array}   colors          - The user defined colors
 * @param  {number}  letterSpacing   - The user defined letter spacing
 *
 * @return {array}                   - The output array with space
 */
const AddLetterSpacing = (output, fontLines, fontLetterspace, fontColors, colors, letterSpacing) => {
	Debugging.report(`Running AddLetterSpacing`, 1);

	let lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

	for (let i = lines; i < output.length; i++) {
		// iterate over last line
		let index = i - lines;
		let space = Colorize(fontLetterspace[index], fontColors, colors);

		if (space.length === 0 && letterSpacing > 0) {
			Debugging.report(`AddLetterSpacing: Adding space to letter spacing`, 1);

			space = ' ';
		}

		output[i] += space.repeat(letterSpacing);
	}

	return output;
};

module.exports = exports = {
	AddLetterSpacing,
};
