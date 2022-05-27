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
 * Colorize
 *   Replace placeholders with color information
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');
const { Color } = require('./Color.js');

/**
 * Replace placeholders with color information
 *
 * @param  {string}  character    - The string to be converted
 * @param  {number}  fontColors   - The number of allowed colors for this font
 * @param  {array}   optionColors - An array of user defined colors
 *
 * @return {string}               - The character with color ansi escape sequences for CLI
 */
const Colorize = (character, fontColors, optionColors) => {
	Debugging.report(`Running Colorize`, 1);

	if (character !== undefined) {
		if (fontColors > 1) {
			// we have to replace all color placeholder with ansi escape sequences
			for (let i = 0; i < fontColors; i++) {
				const color = optionColors[i] || 'system';

				const { open: openNew, close: closeNew } = Color(color);

				const open = new RegExp(`<c${i + 1}>`, 'g');
				const close = new RegExp(`</c${i + 1}>`, 'g');

				character = character.replace(open, openNew);
				character = character.replace(close, closeNew);
			}
		}

		// if only one color is allowed there won't be any color placeholders in the characters
		if (fontColors === 1) {
			const color = optionColors[0] || 'system';

			const { open: openNew, close: closeNew } = Color(color);

			character = openNew + character + closeNew;
		}
	}

	return character;
};

module.exports = exports = {
	Colorize,
};
