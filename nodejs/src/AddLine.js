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
 * AddLine
 *   Add a new line to the output array
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');

/**
 * Add a new line to the output array
 *
 * @param  {array}   output      - The output array the line shall be appended to
 * @param  {number}  fontLines   - The number of lines this font has per character
 * @param  {array}   FontBuffer  - An array of the space we add at the beginning of each line
 * @param  {number}  lineHeight  - The user defined line height
 *
 * @return {array}               - The output array with new line
 */
const AddLine = (output, fontLines, FontBuffer, lineHeight) => {
	Debugging.report(`Running AddLine`, 1);

	if (output.length === 0) {
		lineHeight = 0;
	}

	let lines = fontLines + output.length + lineHeight;
	let length = output.length;

	for (let i = length; i < lines; i++) {
		let index = i - length;

		if (index > lineHeight) {
			output[i] = FontBuffer[index - lineHeight];
		} else {
			output[i] = '';
		}
	}

	return output;
};

module.exports = exports = {
	AddLine,
};
