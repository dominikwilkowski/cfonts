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
 * RenderConsole
 *   Render our input with the console font
 *
 **************************************************************************************************************************************************************/

'use strict';

const { AlignText } = require('./AlignText.js');
const { Colorize } = require('./Colorize.js');
const { Size } = require('./Size.js');


/**
 * Render our input with the console font
 *
 * @param  {string}  INPUT       - The string you want to write out
 * @param  {object}  OPTIONS     - All user options
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {number}  size.width  - The width of the terminal
 * @param  {number}  size.height - The height of the terminal
 *
 * @typedef  {object}  ReturnObject
 *   @property {array}   output  - An array of each line of the output
 *   @property {number}  lines   - The count of line breaks
 *
 * @return {ReturnObject}        - An object with the output and the line breaks
 */
const RenderConsole = ( INPUT, OPTIONS, size = Size ) => {
	const width = OPTIONS.maxLength < size.width && OPTIONS.maxLength !== 0
		? OPTIONS.maxLength
		: size.width;
	let lines = 0;
	let output = [];
	let i = 0;

	let space = '';
	if( OPTIONS.letterSpacing > 0 ) {
		space = ' '.repeat( OPTIONS.letterSpacing );
	}

	// we have to add our letter spacing first
	const outputLines = INPUT
		.replace( /(?:\r\n|\r|\n)/g, '|' )
		.split( '|' )
		.map( line =>
			line
				.split('')
				.join( space )
	);

	// now we check each line for it's length and split them if too long
	while( i < outputLines.length ) {
		let line = outputLines[ i ];

		if( line.length > width ) {
			outputLines[ i ] = line.slice( 0, width );
			outputLines.splice( i + 1, 0, line.slice( width ) );
			line = outputLines[ i ];
		}

		if( OPTIONS.colors[ 0 ] === 'candy' ) {
			output.push( line
				.split('')
				.map( character => Colorize( character, 1, OPTIONS.colors ) )
				.join('')
			);
		}
		else {
			output.push( Colorize( line, 1, OPTIONS.colors ) );
		}

		if( OPTIONS.env !== 'browser' ) {
			output = AlignText( output, line.length, 1, OPTIONS.align, size );
		}
		if( i !== outputLines.length - 1 ) {
			output = [ ...output, ...Array( OPTIONS.lineHeight ).fill('') ];
		}

		lines++;
		i++;
	}

	return {
		output,
		lines,
	};
};


module.exports = exports = {
	RenderConsole,
};
