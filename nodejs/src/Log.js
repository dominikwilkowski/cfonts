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
 * Log
 *   Logging prettiness
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Color } = require('./Color.js');

/**
 * Logging prettiness
 *
 * @type {object}
 */
const Log = {
	/**
	 * Print error message to console.
	 *
	 * @param  {string} text - The sting you want to log
	 */
	error: (text) => {
		text = text.replace(/(?:\r\n|\r|\n)/g, '\n       '); // indent each line
		const { open, close } = Color('red');
		console.error(`\n \u001b[1m${open}Ouch:${close}\u001b[22m ${text}\n`);
	},
};

module.exports = exports = {
	Log,
};
