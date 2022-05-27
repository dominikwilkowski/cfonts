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
 * AlignText
 *   Calculate the spaces to be added to the left of each line to align them either center or right
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');
const { Size } = require('./Size.js');

/**
 * Calculate the spaces to be added to the left of each line to align them either center or right
 *
 * @param  {array}   output         - The output array the line shall be appended to
 * @param  {number}  lineLength     - The current line length
 * @param  {number}  characterLines - The amount of line breaks in one character
 * @param  {string}  align          - The alignment of the text, only `center` and `right` will do anything
 * @param  {object}  size           - The size of the terminal as an object, default: Size
 * @param  {number}  size.width     - The width of the terminal
 * @param  {number}  size.height    - The height of the terminal
 *
 * @return {array}                  - The output array with space added on the left for alignment
 */
const AlignText = (output, lineLength, characterLines, align, size = Size) => {
	Debugging.report(`Running AlignText`, 1);

	let space = 0;

	if (align === 'center') {
		// calculate the size for center alignment
		space = Math.ceil((size.width - lineLength) / 2);

		Debugging.report(`AlignText: Center lineLength: ${lineLength}, size.width: ${size.width}, space: ${space}`, 2);
	}

	if (align === 'right') {
		// calculate the size for right alignment
		space = size.width - lineLength;

		Debugging.report(`AlignText: Right lineLength: ${lineLength}, size.width: ${size.width}, space: ${space}`, 2);
	}

	if (space > 0) {
		// only add if there is something to add
		let lines = output.length - characterLines; // last line is characterLines tall and is located at the bottom of the output array
		const spaces = ' '.repeat(space);

		for (let i = lines; i < output.length; i++) {
			// iterate over last line (which can be several line breaks long)
			output[i] = spaces + output[i];
		}
	}

	return output;
};

module.exports = exports = {
	AlignText,
};
