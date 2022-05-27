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
 * CleanInput
 *   Filter only allowed character
 *
 **************************************************************************************************************************************************************/

'use strict';

const { CHARS } = require('./constants.js');

/**
 * Filter only allowed character
 *
 * @param  {string} INPUT - The input text to be filtered
 * @param  {array}  chars - An array of all allowed characters
 *
 * @return {string}       - The filtered input text
 */
const CleanInput = (INPUT, chars = CHARS) => {
	if (typeof INPUT === 'string') {
		const clean = INPUT.replace(/(?:\r\n|\r|\n)/g, '|')
			.split('')
			.filter((char) => chars.includes(char.toUpperCase()))
			.join('');

		return clean;
	} else {
		return '';
	}
};

module.exports = exports = {
	CleanInput,
};
