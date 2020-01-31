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
 * Log
 *   Logging prettiness
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Chalk } = require('./Chalk.js');


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
	error: ( text ) => {
		text = text.replace( /(?:\r\n|\r|\n)/g, '\n       ' ); // indent each line

		console.error(`\n ${ Chalk.bold.red('Ouch:') } ${ text }\n`);
	},
};


module.exports = exports = {
	Log,
};
