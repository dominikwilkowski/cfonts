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
 **************************************************************************************************************************************************************/

'use strict';


// Dependencies
const ChangeCase = require('change-case');
const WinSize = require('window-size');
const Style = require('ansi-styles');
const Chalk = require(`chalk`);
const Path = require(`path`);
const Fs = require(`fs`);


// settings
const DEBUG = false;
const DEBUGLEVEL = 2;
const COLORS = [       // All allowed font colors
	'system',
	'black',
	'red',
	'green',
	'yellow',
	'blue',
	'magenta',
	'cyan',
	'white',
	'gray',
	'redBright',
	'greenBright',
	'yellowBright',
	'blueBright',
	'magentaBright',
	'cyanBright',
	'whiteBright',
];
const BGCOLORS = [     // All allowed background colors
	'transparent',
	'black',
	'red',
	'green',
	'yellow',
	'blue',
	'magenta',
	'cyan',
	'white',
	'blackBright',
	'redBright',
	'greenBright',
	'yellowBright',
	'blueBright',
	'magentaBright',
	'cyanBright',
	'whiteBright',
];
const ALIGNMENT = [    // All allowed alignment options
	'left',
	'center',
	'right',
];
const FONTFACES = [    // All allowed fonts
	'console',
	'block',
	'simpleBlock',
	'simple',
	'3d',
	'simple3d',
	'chrome',
	'huge',
];


/**
 * Get a selected JSON font-file object
 *
 * @param  {string} font - The name of the font to be returned
 *
 * @return {object}      - The font object of that file
 */
const GetFont = ( font ) => {
	Debugging.report( `Running GetFont`, 1 );

	// try loading the font file
	try {
		let fontFile = Path.normalize( `${ __dirname }/../fonts/${ font }.json` ); // build font path
		let FONTFACE = JSON.parse( Fs.readFileSync( fontFile, 'utf8' ) ); // read font file

		Debugging.report( `GetFont: Fontface path selected: "${ fontFile }"`, 2 );

		return FONTFACE;
	}
	catch( error ) {
		Debugging.error( `Font file for "${ font }" errored out: ${ error }`, 2 );
		Log.error( `Font file for "${ font }" could not be found.\nTry reinstalling this package.` );

		process.exit( 1 ); // exit program with failure code
	}
};


/**
 * Return the max width of a character by looking at its longest line
 *
 * @param  {array}  character - The character array from the font face object
 * @param  {object} FONTFACE  - The font file object
 * @param  {object} OPTIONS   - Our options
 *
 * @return {integer}         - The length of a longest line in a character
 */
const CharLength = ( character, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running CharLength`, 1 );

	let charWidth = 0;

	for( let i = 0; i < FONTFACE.lines; i++ ) {
		let char = character[ i ].replace( /(<([^>]+)>)/ig, '' ); // get character and strip color infos

		if( char.length > charWidth ) {
			charWidth = char.length; // assign only largest
		}
	};

	if( charWidth === 0 && OPTIONS.letterSpacing > 0 ) {
		Debugging.report( `CharLength: Adding space to letter spacing`, 1 );

		charWidth = 1;
	}

	return charWidth;
};


/**
 * Add a new line to the output array
 *
 * @param  {array}  output   - The output array the line shall be appended to
 * @param  {object} FONTFACE - The font file object
 * @param  {object} OPTIONS  - Our options
 *
 * @return {array}           - The output array with new line
 */
const AddLine = ( output, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running AddLine`, 1 );

	let lineHeight = OPTIONS.lineHeight;
	if( output.length === 0 ) {
		lineHeight = 0;
	}

	let lines = FONTFACE.lines + output.length + lineHeight;
	let length = output.length;

	for(let i = length; i < lines; i++) {
		let index = i - length;

		if( index > lineHeight ) {
			output[ i ] = FONTFACE.buffer[ ( index - lineHeight ) ];
		}
		else {
			output[ i ] = '';
		}
	}

	return output;
};


/**
 * Abstraction for all ansi codes with open and close keys
 *
 * @type {object}
 */
const AnsiSytle = Object.assign( { system: { open: '', close: '' } }, Style );


/**
 * Replace placeholders with color information
 *
 * @param  {string} character - The string to be converted
 * @param  {object} FONTFACE  - The font file object
 * @param  {object} OPTIONS   - Our options
 *
 * @return {string}           - The character with color information for CLI
 */
const Colorize = ( character, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running Colorize`, 1 );

	let candyColors = [
		'red',
		'green',
		'yellow',
		'magenta',
		'cyan',
		'redBright',
		'greenBright',
		'yellowBright',
		'blueBright',
		'magentaBright',
		'cyanBright',
	]; // allowed candy colors

	if( character !== undefined ) {
		if( FONTFACE.colors > 1 ) {
			for( let i = 0; i < FONTFACE.colors; i++ ) { // convert all colors
				let open = new RegExp(`<c${ ( i + 1 ) }>`, 'g');
				let close = new RegExp(`</c${ ( i + 1 ) }>`, 'g');

				let color = OPTIONS.colors[ i ] || 'system';

				if( color === 'candy' ) {
					color = candyColors[ Math.floor( Math.random() * candyColors.length ) ];
				}

				character = character.replace( open, AnsiSytle[ color ].open );
				character = character.replace( close, AnsiSytle[ color ].close );
			}
		}

		if( FONTFACE.colors === 1 ) {
			let color = OPTIONS.colors[ 0 ] || 'system';

			if( color === 'candy' ) {
				color = candyColors[ Math.floor( Math.random() * candyColors.length ) ];
			}

			character = AnsiSytle[ color ].open + character + AnsiSytle[ color ].close;
		}
	}

	return character;
};


/**
 * Add a new character to the output array
 *
 * @param  {string} CHAR     - The character to be added
 * @param  {array}  output   - The output array the line shall be appended to
 * @param  {object} FONTFACE - The font file object
 * @param  {object} OPTIONS  - Our options
 *
 * @return {array}           - The output array with new line
 */
const AddChar = ( CHAR, output, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running AddChar with "${ CHAR }"`, 1 );

	let lines = output.length - FONTFACE.lines; // last line is FONTFACE.lines tall and is located at the bottom of the output array

	for( let i = lines; i < output.length; i++ ) { // iterate over last line
		let index = i - lines;

		output[ i ] += Colorize( FONTFACE.chars[ CHAR ][ index ], FONTFACE, OPTIONS );
	}

	return output;
};


/**
 * Add letter spacing for the next character
 *
 * @param  {array}  output   - The output array the line shall be appended to
 * @param  {object} FONTFACE - The font file object
 * @param  {object} OPTIONS  - Our options
 *
 * @return {array}           - The output array with space
 */
const AddLetterSpacing = ( output, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running AddLetterSpacing`, 1 );

	let lines = output.length - FONTFACE.lines; // last line is FONTFACE.lines tall and is located at the bottom of the output array

	for( let i = lines; i < output.length; i++ ) { // iterate over last line
		let index = i - lines;
		let space = Colorize( FONTFACE.letterspace[ index ], FONTFACE, OPTIONS );

		if( space.length === 0 && OPTIONS.letterSpacing > 0 ) {
			Debugging.report( `AddLetterSpacing: Adding space to letter spacing`, 1 );

			space = ' ';
		}

		output[ i ] += String.repeat( space, OPTIONS.letterSpacing );
	}

	return output;
};


/**
 * Abstraction for windows size
 *
 * @type {object}
 */
const Size = {
	width: WinSize.width || 80,
	height: WinSize.height || 24,
};


/**
 * Calculate the spaces to be added to the left of each line to align them either center or right
 *
 * @param  {array}   output     - The output array the line shall be appended to
 * @param  {integer} lineLength - The current line length
 * @param  {object}  FONTFACE   - The font file object
 * @param  {object}  OPTIONS    - Our options
 *
 * @return {array}              - The output array with space added on the left for alignment
 */
const AlignText = ( output, lineLength, FONTFACE, OPTIONS ) => {
	Debugging.report( `Running AlignText`, 1 );

	let space = 0;

	if( OPTIONS.align === 'center' ) { // calculate the size for center alignment
		space = Math.floor( ( Size.width - lineLength ) / 2 );

		Debugging.report( `AlignText: Center lineLength: ${ lineLength }, Size.width: ${ Size.width }, space: ${ space }`, 2 );
	}

	if( OPTIONS.align === 'right' ) { // calculate the size for right alignment
		space = Size.width - lineLength;

		Debugging.report( `AlignText: Right lineLength: ${ lineLength }, Size.width: ${ Size.width }, space: ${ space }`, 2 );
	}


	if( space > 0 ) { // only add if there is something to add
		let lines = output.length - FONTFACE.lines; // last line is FONTFACE.lines tall and is located at the bottom of the output array
		space = String.repeat(' ', space );

		for( let i = lines; i < output.length; i++ ) { // iterate over last line
			output[ i ] = space + output[ i ];
		}
	}

	return output;
};


/**
 * Main method to get the ANSI output for a string
 *
 * @param  {string}  INPUT                  - The string you want to write out
 * @param  {object}  SETTINGS               - Settings object
 * @param  {string}  SETTINGS.font          - Font face, Default 'block'
 * @param  {string}  SETTINGS.align         - Text alignment, Default: 'left'
 * @param  {array}   SETTINGS.colors        - Colors for font, Default: []
 * @param  {string}  SETTINGS.background    - Chalk color string for background, Default 'Black'
 * @param  {integer} SETTINGS.letterSpacing - Space between letters, Default: set by selected font face
 * @param  {integer} SETTINGS.lineHeight    - Space between lines, Default: 1
 * @param  {boolean} SETTINGS.space         - Output space before and after output, Default: true
 * @param  {integer} SETTINGS.maxLength     - Maximum amount of characters per line, Default width of console window
 *
 * @return {object}                         - CLI output of INPUT to be consoled out
 *                                            {string}  string  - The pure string for output with all line breaks
 *                                            {array}   array   - Each line of output in an array
 *                                            {integer} lines   - The number of lines
 *                                            {object}  options - All options used
 */
const Render = ( INPUT = '', SETTINGS = {} ) => {
	Debugging.report(`Running render`, 1);

	let write = ''; // output in a string
	let output = []; // output in an array
	let lines = 0; // count each line
	let fontface = {};

	// Options
	const OPTIONS = { // SETTINGS merged with defaults
		font: SETTINGS.font || 'block',
		align: SETTINGS.align || 'left',
		colors: SETTINGS.colors || [],
		background: SETTINGS.background || SETTINGS.backgroundColor || 'transparent',
		letterSpacing: SETTINGS.letterSpacing === undefined ? 1 : SETTINGS.letterSpacing,
		lineHeight: SETTINGS.lineHeight === undefined ? 1 : parseInt( SETTINGS.lineHeight ),
		space: SETTINGS.space === undefined ? true : SETTINGS.space,
		maxLength: SETTINGS.maxLength || 0,
	};

	// CHECKING INPUT
	if( INPUT === undefined || INPUT === '' ) {
		Log.error(`Please provide text to convert`);

		process.exit(1); // exit program with failure code
	}

	// CHECKING FONT
	if( FONTFACES.indexOf( OPTIONS.font ) === -1 ) {
		Log.error(
			`"${ Chalk.red( SETTINGS.font ) }" is not a valid font option.\n` +
			`Please use a font from the supported stack:\n${ Chalk.green(`[ ${ FONTFACES.join(' | ') } ]`) }`
		);

		process.exit(1); // exit program with failure code
	}

	// CHECKING COLORS
	for( let color in OPTIONS.colors ) { // check color usage
		if(
			COLORS.indexOf( OPTIONS.colors[ color ] ) === -1 &&
			OPTIONS.colors[ color ] !== 'candy'
		) {
			Log.error(
				`"${ Chalk.red( OPTIONS.colors[ color ] ) }" is not a valid font color option.\n` +
				`Please use a color from the supported stack:\n${ Chalk.green(`[ ${ COLORS.join(' | ') } | candy ]`) }`
			);

			process.exit(1); // exit program with failure code
		}
	}

	// CHECKING BACKGROUND COLORS
	if( BGCOLORS.indexOf( OPTIONS.background ) === -1 ) {
		Log.error(
			`"${ Chalk.red( OPTIONS.background ) }" is not a valid background option.\n` +
			`Please use a color from the supported stack:\n${ Chalk.green(`[ ${ BGCOLORS.join(' | ') } ]`) }`
		);

		process.exit(1); // exit program with failure code
	}

	// CHECKING ALIGNMENT
	if( ALIGNMENT.indexOf( OPTIONS.align ) === -1 ) {
		Log.error(
			`"${ Chalk.red( OPTIONS.align ) }" is not a valid alignment option.\n` +
			`Please use an alignment option from the supported stack:\n${ Chalk.green(`[ ${ CFonts.ALIGNMENT.join(' | ') } ]`) }`
		);

		process.exit(1); // exit program with failure code
	}


	// DEBUG
	if( DEBUG ) { // log options
		let outOption = `OPTIONS:\n  Text: ${ INPUT }`;

		for( let key in OPTIONS ) {
			outOption += `\n  Options.${ key }: ${ OPTIONS[ key ] }`;
		}

		Debugging.report( outOption, 2 );
	}


	if( OPTIONS.font === 'console' ) { // console fontface is pretty easy to process
		let outputLines = INPUT.replace('\\', '').split('|'); // remove escape characters and split into each line

		FONTFACE.colors = 1; // console defaults
		FONTFACE.lines = 1;

		for(let line in outputLines) { // each line needs to be pushed into the output array
			lines += Math.ceil( outputLines[ line ].length / Size.width ); // count each line even when they overflow

			if( OPTIONS.colors[0] === "candy" ) { // if the color is candy
				let character = '';

				for(let i = 0; i < outputLines[ line ].length; i++) { // iterate through the message
					character += Colorize( outputLines[ line ][ i ], FONTFACE, OPTIONS ); // and colorize each character individually
				}

				output.push( character ); // push each line to the output array
			}
			else {
				output.push( Colorize( outputLines[ line ], FONTFACE, OPTIONS ) ); // colorize line
			}

			output = AlignText( output, outputLines[ line ].length, FONTFACE, OPTIONS ); // calculate alignment based on lineLength
		}
	}
	else { // all other fontfaces need the font-file and some more work
		const FONTFACE = GetFont( OPTIONS.font ); // get fontface object and make it global
		fontface = FONTFACE;

		// setting the letterspacing preference from font face if there is no user overwrite
		if( SETTINGS.letterSpacing === undefined ) {
			Debugging.report(`Looking up letter spacing from font face`, 1);

			let width = 0;

			for( let i in FONTFACE.letterspace ) {
				let char = FONTFACE.letterspace[ i ].replace(/(<([^>]+)>)/ig, ''); // get character and strip color infos

				if( width < char.length ) {
					width = char.length;
				}
			}

			Debugging.report(`Letter spacing set to font face default: "${ width }"`, 2);
			OPTIONS.letterSpacing = width;
		}

		let lineLength = CharLength( FONTFACE.buffer, FONTFACE, OPTIONS ); // count each output character per line and start with the buffer
		let maxChars = 0; // count each character we print for maxLength option

		output = AddLine( [], FONTFACE, OPTIONS ); // create first lines with buffer
		lines ++;

		output = AddLetterSpacing( output, FONTFACE, OPTIONS ); // add letter spacing to the beginning
		lineLength += CharLength( FONTFACE.letterspace, FONTFACE, OPTIONS ) * OPTIONS.letterSpacing; // count the space for the letter spacing

		for( let i = 0; i < INPUT.length; i++ ) { // iterate through the message

			let CHAR = INPUT.charAt( i ).toUpperCase(); // the current character we convert, only upper case is supported at this time

			if( FONTFACE.chars[ CHAR ] === undefined && CHAR !== `|` ) { // make sure this character exists in the font
				Debugging.error(`Character not found in font: "${ CHAR }"`, 2); // fail silently
			}
			else {
				Debugging.report(`Character found in font: "${ CHAR }"`, 2);

				let lastLineLength = lineLength; // we need the lineLength for alignment before we look up if the next char fits

				if( CHAR !== `|` ) { // what will the line length be if we add the next char?
					lineLength += CharLength( FONTFACE.chars[ CHAR ], FONTFACE, OPTIONS ); // get the length of this character
					lineLength += CharLength( FONTFACE.letterspace, FONTFACE, OPTIONS ) * OPTIONS.letterSpacing; // new line, new line length
				}

				// jump to next line after OPTIONS.maxLength characters or when line break is found or the console windows would has ran out of space
				if(maxChars >= OPTIONS.maxLength && OPTIONS.maxLength != 0 || CHAR === `|` || lineLength > Size.width) {
					lines ++;

					Debugging.report(
						`NEWLINE: maxChars: ${ maxChars }, ` +
						`OPTIONS.maxLength: ${ OPTIONS.maxLength }, ` +
						`CHAR: ${ CHAR }, ` +
						`lineLength: ${ lineLength }, ` +
						`Size.width: ${ Size.width } `, 2
					);

					output = AlignText( output, lastLineLength, FONTFACE, OPTIONS ); // calculate alignment based on lineLength

					lineLength = CharLength( FONTFACE.buffer, FONTFACE, OPTIONS ); // new line: new line length
					lineLength += CharLength( FONTFACE.letterspace, FONTFACE, OPTIONS ) * OPTIONS.letterSpacing; // each new line starts with letter spacing
					if( CHAR !== `|` ) { // if this is a character
						lineLength += CharLength( FONTFACE.chars[ CHAR ], FONTFACE, OPTIONS ); // get the length of this character
						lineLength += CharLength( FONTFACE.letterspace, FONTFACE, OPTIONS ) * OPTIONS.letterSpacing; // add letter spacing at the end
					}
					maxChars = 0; // new line, new maxLength goal

					output = AddLine( output, FONTFACE, OPTIONS ); // adding new line
					output = AddLetterSpacing( output, FONTFACE, OPTIONS ); // add letter spacing to the beginning
				}

				Debugging.report(`lineLength at: "${ lineLength }"`, 2);

				if( CHAR !== `|` ) {
					maxChars++; // counting all printed characters
					output = AddChar( CHAR, output, FONTFACE, OPTIONS ); // add new character
					output = AddLetterSpacing( output, FONTFACE, OPTIONS ); // add letter spacing
				}
			}
		}

		output = AlignText( output, lineLength, FONTFACE, OPTIONS ); // alignment last line
	}

	write = output.join(`\n`);


	if( fontface.colors <= 1 ) {
		write = Colorize( write, fontface, OPTIONS );
	}


	if( OPTIONS.space ) { // add space
		write = `\n\n` + write + `\n\n`;
	}


	if( OPTIONS.background !== 'transparent' ) {
		const bgcolor = `bg${ ChangeCase.upperCaseFirst( OPTIONS.background ) }`;

		write = AnsiSytle[ bgcolor ].open + '\n' + write + AnsiSytle[ bgcolor ].close; // result in one string with background
	}


	return {
		string: write,
		array: output,
		lines: lines,
		options: OPTIONS,
	}
};


/**
 * Print to console
 *
 * @param same as render method
 */
const Say = ( INPUT = '', SETTINGS = {} ) => {
	Debugging.report(`Running say`, 1);

	let write = Render( INPUT, SETTINGS );

	console.log( write.string ); // write out
};


/**
 * Debugging prettiness
 *
 * @type {object}
 */
const Debugging = {
	/**
	 * Return a headline preferably at the beginning of your app
	 *
	 * @param  {string}  text  - The sting you want to log
	 * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
	 */
	headline: ( text, level = 99 ) => {
		if( DEBUG && level >= DEBUGLEVEL ) {
			console.log(
				Chalk.bgWhite(`\n${ Chalk.bold(' \u2611  ') } ${ text }`)
			);
		}
	},

	/**
	 * Return a message to report starting a process
	 *
	 * @param  {string}  text  - The sting you want to log
	 * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
	 */
	report: ( text, level = 99 ) => {
		if( DEBUG && level >= DEBUGLEVEL ) {
			console.log(
				Chalk.bgWhite(`\n${ Chalk.bold.green(' \u2611  ') } ${ Chalk.black(`${ text } `) }`)
			);
		}
	},

	/**
	 * Return a message to report an error
	 *
	 * @param  {string}  text  - The sting you want to log
	 * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
	 */
	error: ( text, level = 99 ) => {
		if( DEBUG && level >= DEBUGLEVEL ) {
			console.error(
				Chalk.bgWhite(`\n${ Chalk.red(' \u2612  ') } ${ Chalk.black(`${ text } `) }`)
			);
		}
	},
};


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
		text = text.replace(/(?:\r\n|\r|\n)/g, '\n       '); // indent each line

		console.error(`\n ${ Chalk.bold.red('Ouch:') } ${ text }\n`);
	},
};


// Module export
module.exports = exports = {
	render: Render,
	say: Say,
	DEBUG,
	DEBUGLEVEL,
	COLORS,
	BGCOLORS,
	ALIGNMENT,
	FONTFACES,
	Debugging,
	Log,

	__testing__: {
		GetFont,
		CharLength,
		AddLine,
		AnsiSytle,
		Colorize,
		AddChar,
		AddLetterSpacing,
		Size,
		AlignText,
	},
};