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
 * CharLength
 *   Return the max width of a character by looking at its longest line
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');

/**
 * Return the max width of a character by looking at its longest line
 *
 * @param  {array}   character     - The character array from the font face object
 * @param  {number}  fontLines     - The number of lines this font has per character
 * @param  {number}  letterSpacing - The user defined letter spacing
 *
 * @return {number}                - The length of a longest line in a character
 */
const CharLength = (character, fontLines, letterSpacing) => {
	Debugging.report(`Running CharLength`, 1);

	let charWidth = 0;

	for (let i = 0; i < fontLines; i++) {
		let char = character[i].replace(/(<([^>]+)>)/gi, ''); // get character and strip color infos

		if (char.length > charWidth) {
			charWidth = char.length; // assign only largest
		}
	}

	if (charWidth === 0 && letterSpacing > 0) {
		Debugging.report(`CharLength: Adding space to letter spacing`, 1);

		charWidth = 1;
	}

	return charWidth;
};

module.exports = exports = {
	CharLength,
};
