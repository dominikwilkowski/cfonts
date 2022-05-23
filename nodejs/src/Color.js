/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPLv2
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
const { Options } = require('./Options.js');
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

	const OPTIONS = Options.get;

	if( OPTIONS.env === 'node' ) {

		let open;
		let close = bg ? "\x1b[49m" : "\x1b[39m";

		switch (color.toLowerCase()) {
			case "system":
				open = "\x1b[39m"
				break;
			case "transparent":
				open = "\x1b[49m"
				break;
			case "black":
				open = bg ? "\x1b[40m" : "\x1b[30m"
				break;
			case "red":
				open = bg ? "\x1b[41m" : "\x1b[31m"
				break;
			case "green":
				open = bg ? "\x1b[42m" : "\x1b[32m"
				break;
			case "yellow":
				open = bg ? "\x1b[43m" : "\x1b[33m"
				break;
			case "blue":
				open = bg ? "\x1b[44m" : "\x1b[34m"
				break;
			case "magenta":
				open = bg ? "\x1b[45m" : "\x1b[35m"
				break;
			case "cyan":
				open = bg ? "\x1b[46m" : "\x1b[36m"
				break;
			case "white":
				open = bg ? "\x1b[47m" : "\x1b[37m"
				break;
			case "gray":
				open = bg ? "\x1b[100m" : "\x1b[90m"
				break;
			case "redbright":
				open = bg ? "\x1b[101m" : "\x1b[91m"
				break;
			case "greenbright":
				open = bg ? "\x1b[102m" : "\x1b[92m"
				break;
			case "yellowbright":
				open = bg ? "\x1b[103m" : "\x1b[93m"
				break;
			case "bluebright":
				open = bg ? "\x1b[104m" : "\x1b[94m"
				break;
			case "magentabright":
				open = bg ? "\x1b[105m" : "\x1b[95m"
				break;
			case "cyanbright":
				open = bg ? "\x1b[106m" : "\x1b[96m"
				break;
			case "whitebright":
				open = bg ? "\x1b[107m" : "\x1b[97m"
				break;
			default: {
				const kind = HEXTEST.test( color )
					? bg ? 'bgHex' : 'hex'
					: `${ bg ? 'bgK' : 'k' }eyword`;
				open = Chalk[ kind ]( color )._styler.open;
			}
		}
		return { open, close };

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
	}
	else {
		const COLORS = {
			black: '#000',
			red: '#ea3223',
			green: '#377d22',
			yellow: '#fffd54',
			blue: '#0020f5',
			magenta: '#ea3df7',
			cyan: '#74fbfd',
			white: '#fff',
			gray: '#808080',
			redbright: '#ee776d',
			greenbright: '#8cf57b',
			yellowbright: '#fffb7f',
			bluebright: '#6974f6',
			magentabright: '#ee82f8',
			cyanbright: '#8dfafd',
			whitebright: '#fff',
		};

		if( !HEXTEST.test( color ) ) {
			color = COLORS[ color.toLowerCase() ];
		}

		if( bg ) {
			return {
				open: color,
				close: '',
			};
		}

		return {
			open: `<span style="color:${ color }">`,
			close: '</span>',
		}
	}
};


module.exports = exports = {
	Color,
};
