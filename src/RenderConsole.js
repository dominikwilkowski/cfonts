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
const { Debugging } = require('./Debugging.js');
const { Colorize } = require('./Colorize.js');
const { AddLine } = require('./AddLine.js');
const { Size } = require('./Size.js');


/**
 * Render our input with the console font
 *
 * @param  {string}  INPUT       - The string you want to write out
 * @param  {object}  OPTIONS     - All user options
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {integer} size.width  - The width of the terminal
 * @param  {integer} size.height - The height of the terminal
 *
 * @typedef  {object} ReturnObject
 *   @property {array}   output  - An array of each line of the output
 *   @property {integer} lines   - The count of line breaks
 *
 * @return {ReturnObject}        - An object with the output and the line breaks
 */
const RenderConsole = ( INPUT, OPTIONS, size = Size ) => {
	let output = [];
	let i = 0;

	// the defaults need to cramp a little so console doesn't look silly
	OPTIONS.letterSpacing = OPTIONS.letterSpacing <= 1 ? 0 : OPTIONS.letterSpacing - 1;
	OPTIONS.lineHeight = OPTIONS.lineHeight <= 1 ? 0 : OPTIONS.lineHeight - 1;

	let space = '';
	if( OPTIONS.letterSpacing > 0 ) {
		space = ' '.repeat( OPTIONS.letterSpacing );
	}

	// we have to add our letter spacing first
	let outputLines = INPUT
		.replace( /(?:\r\n|\r|\n)/g, '|' )
		.split( '|' )
		.map( line =>
			line
				.trim()
				.split('')
				.join( space )
	);

	// now we check each line for it's length and split them if too long
	while( i < outputLines.length ) {
		let line = outputLines[ i ];

		if( line.length > size.width ) {
			outputLines[ i ] = line.slice( 0, size.width ).trim();
			outputLines.splice( i + 1, 0, line.slice( size.width ).trim() );
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
			output.push( line );
		}

		output = AlignText( output, line.length, 1, OPTIONS.align, size );
		output = AddLine( output, 0, [''], OPTIONS.lineHeight );

		i++;
	}

	return {
		output,
		lines: output.length,
	};
};


module.exports = exports = {
	RenderConsole,
};
