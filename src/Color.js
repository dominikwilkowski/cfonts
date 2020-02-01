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
 * Color
 *   Abstraction for coloring hex-, keyword- and background-colors
 *
 **************************************************************************************************************************************************************/

'use strict';

const { UpperCaseFirst } = require('./UpperCaseFirst.js');
const { Debugging } = require('./Debugging.js');
const { HEXTEST } = require('./constants.js');
const { Chalk } = require('./Chalk.js');


/**
 * Abstraction for coloring hex-, keyword- and background-colors
 *
 * @param  {string}  color    - The color to be used
 * @param  {boolean} bg       - Whether this is a background or not
 *
 * @typedef  {object} ReturnObject
 *   @property {string} open  - The open ansi code
 *   @property {string} close - The close ansi code
 *
 * @return {ReturnObject}     - An object with open and close ansi codes
 */
const Color = ( color, bg = false ) => {
	// bail early if we use system color
	if( color === 'system' || process.env.FORCE_COLOR == '0' ) {
		return { open: '', close: '' };
	}

	// bail if this is a chalk defined color
	if( color.includes('Bright') ) {
		if( bg ) {
			color = `bg${ UpperCaseFirst( color ) }`;
		}

		return {
			open: Chalk[ color ]._styler.open,
			close: Chalk[ color ]._styler.close,
		};
	}

	const kind = HEXTEST.test( color )
		? 'hex'
		: `${ bg ? 'bgK' : 'k' }eyword`;

	let styles;
	try {
		styles = Chalk[ kind ]( color )._styler;
	}
	catch( error ) {
		Debugging.error(`The color ${ Chalk.yellow( color ) } could not be found. Sorry about this.`);
		return { open: '', close: '' };
	}

	return {
		open: styles.open,
		close: styles.close,
	};
};


module.exports = exports = {
	Color,
};
