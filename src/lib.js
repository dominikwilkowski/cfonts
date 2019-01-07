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


// global defaults
let DEBUG = false;
let DEBUGLEVEL = 2;
const CHARS = [
	"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
	"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "|",
	"!", "?", ".", "+", "-", "_", "=", "@", "#", "$", "%", "&", "(", ")", "/", ":", ";", ",", " ", "'"
];
const COLORS = {
	system: 'system',
	black: 'black',
	red: 'red',
	green: 'green',
	yellow: 'yellow',
	blue: 'blue',
	magenta: 'magenta',
	cyan: 'cyan',
	white: 'white',
	gray: 'gray',
	redbright: 'redBright',
	greenbright: 'greenBright',
	yellowbright: 'yellowBright',
	bluebright: 'blueBright',
	magentabright: 'magentaBright',
	cyanbright: 'cyanBright',
	whitebright: 'whiteBright',
};
const BGCOLORS = {
	transparent: 'transparent',
	black: 'black',
	red: 'red',
	green: 'green',
	yellow: 'yellow',
	blue: 'blue',
	magenta: 'magenta',
	cyan: 'cyan',
	white: 'white',
	blackbright: 'blackBright',
	redbright: 'redBright',
	greenbright: 'greenBright',
	yellowbright: 'yellowBright',
	bluebright: 'blueBright',
	magentabright: 'magentaBright',
	cyanbright: 'cyanBright',
	whitebright: 'whiteBright',
};
const ALIGNMENT = [
	'left',
	'center',
	'right',
];
const FONTFACES = {
	console: 'console',
	block: 'block',
	simpleblock: 'simpleBlock',
	simple: 'simple',
	'3d': '3d',
	simple3d: 'simple3d',
	chrome: 'chrome',
	huge: 'huge',
};
const CLIOPTIONS = {
	'--version': {
		description: 'Use to display the version of cfonts',
		example: '--version',
		short: '-v',
		default: false,
	},
	'--help': {
		description: 'Use to display this help',
		example: '--help',
		short: '-h',
		default: false,
	},
	'--font': {
		description: 'Use to define the font face',
		example: '--font block',
		short: '-f',
		options: Object.keys( FONTFACES ).map( color => FONTFACES[ color ] ),
		default: 'block',
	},
	'--colors': {
		description: 'Use to define the font color',
		example: '--colors block',
		short: '-c',
		options: true, // Object.keys( COLORS ).map( color => COLORS[ color ] ),
		default: 'system',
	},
	'--background': {
		description: 'Use to define background color',
		example: '--background block',
		short: '-b',
		options: Object.keys( BGCOLORS ).map( color => BGCOLORS[ color ] ),
		default: 'transparent',
	},
	'--align': {
		description: 'Use to align your text output',
		example: '--align block',
		short: '-a',
		options: ALIGNMENT,
		default: 'left',
	},
	'--letter-spacing': {
		description: 'Use to define your letter spacing',
		example: '--letter-spacing 2',
		short: '-l',
		options: true,
		default: 1,
	},
	'--line-height': {
		description: 'Use to define your line height',
		example: '--line-height 5',
		short: '-z',
		options: true,
		default: 1,
	},
	'--spaceless': {
		description: 'Use to disable the padding around your output',
		example: '--spaceless',
		short: '-s',
		default: false,
	},
	'--max-length': {
		description: 'Use to define the amount of maximum characters per line',
		example: '--max-length 10',
		short: '-m',
		options: true,
		default: 0,
	},
	'--debug': {
		description: 'Use to enable debug mode',
		example: '--debug',
		short: '-d',
		default: false,
	},
	'--debug-level': {
		description: 'Use to define the debug level. The higher, the less debug infos',
		example: '--debug-level 2',
		short: '-x',
		options: true,
		default: 1,
	},
};
const PACKAGE = JSON.parse( Fs.readFileSync( Path.normalize(`${ __dirname }/../package.json`), 'utf8' ) );


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

		return false;
	}
};


/**
 * Return the max width of a character by looking at its longest line
 *
 * @param  {array}   character     - The character array from the font face object
 * @param  {integer} fontLines     - The number of lines this font has per character
 * @param  {integer} letterSpacing - The user defined letter spacing
 *
 * @return {integer}               - The length of a longest line in a character
 */
const CharLength = ( character, fontLines, letterSpacing ) => {
	Debugging.report( `Running CharLength`, 1 );

	let charWidth = 0;

	for( let i = 0; i < fontLines; i++ ) {
		let char = character[ i ].replace( /(<([^>]+)>)/ig, '' ); // get character and strip color infos

		if( char.length > charWidth ) {
			charWidth = char.length; // assign only largest
		}
	};

	if( charWidth === 0 && letterSpacing > 0 ) {
		Debugging.report( `CharLength: Adding space to letter spacing`, 1 );

		charWidth = 1;
	}

	return charWidth;
};


/**
 * Add a new line to the output array
 *
 * @param  {array}   output      - The output array the line shall be appended to
 * @param  {integer} fontLines   - The number of lines this font has per character
 * @param  {array}   FontBuffer  - An array of the space we add at the beginning of each line
 * @param  {integer} lineHeight  - The user defined line height
 *
 * @return {array}               - The output array with new line
 */
const AddLine = ( output, fontLines, FontBuffer, lineHeight ) => {
	Debugging.report( `Running AddLine`, 1 );

	if( output.length === 0 ) {
		lineHeight = 0;
	}

	let lines = fontLines + output.length + lineHeight;
	let length = output.length;

	for( let i = length; i < lines; i++ ) {
		let index = i - length;

		if( index > lineHeight ) {
			output[ i ] = FontBuffer[ ( index - lineHeight ) ];
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
 * @param  {string}  character    - The string to be converted
 * @param  {integer} fontColors   - The number of allowed colors for this font
 * @param  {array}   optionColors - An array of user defined colors
 *
 * @return {string}               - The character with color ansi escape sequences for CLI
 */
const Colorize = ( character, fontColors, optionColors ) => {
	Debugging.report( `Running Colorize`, 1 );

	let candyColors = [ // allowed candy colors
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
	];

	if( character !== undefined ) {
		if( fontColors > 1 ) {
			for( let i = 0; i < fontColors; i++ ) { // we have to replace all color placeholder with ansi escape sequences
				let open = new RegExp(`<c${ ( i + 1 ) }>`, 'g');
				let close = new RegExp(`</c${ ( i + 1 ) }>`, 'g');

				let color = optionColors[ i ] || 'system';

				if( color === 'candy' ) {
					color = candyColors[ Math.floor( Math.random() * candyColors.length ) ];
				}

				character = character.replace( open, AnsiSytle[ color ].open );
				character = character.replace( close, AnsiSytle[ color ].close );
			}
		}

		if( fontColors === 1 ) { // if only one color is allowed there won't be any color placeholders in the characters
			let color = optionColors[ 0 ] || 'system';

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
 * @param  {string}  CHAR       - The character to be added
 * @param  {array}   output     - The output array the line shall be appended to
 * @param  {integer} fontLines  - The number of lines this font has per character
 * @param  {object}  fontChars  - An object with all character arrays
 * @param  {integer} fontColors - The amount of colors allowed for this font
 * @param  {object}  colors     - Our options
 *
 * @return {array}              - The output array with new line
 */
const AddChar = ( CHAR, output, fontLines, fontChars, fontColors, colors ) => {
	Debugging.report( `Running AddChar with "${ CHAR }"`, 1 );

	let lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

	for( let i = lines; i < output.length; i++ ) { // iterate over last line
		let index = i - lines;

		output[ i ] += Colorize( fontChars[ CHAR ][ index ], fontColors, colors );
	}

	return output;
};


/**
 * Add letter spacing for the next character
 *
 * @param  {array}   output          - The output array the line shall be appended to
 * @param  {integer} fontLines       - The number of lines this font has per character
 * @param  {array}   fontLetterspace - A space between the letters
 * @param  {integer} fontColors      - The amount of colors allowed for this font
 * @param  {array}   colors          - The user defined colors
 * @param  {integer} letterSpacing   - The user defined letter spacing
 *
 * @return {array}                   - The output array with space
 */
const AddLetterSpacing = ( output, fontLines, fontLetterspace, fontColors, colors, letterSpacing ) => {
	Debugging.report( `Running AddLetterSpacing`, 1 );

	let lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

	for( let i = lines; i < output.length; i++ ) { // iterate over last line
		let index = i - lines;
		let space = Colorize( fontLetterspace[ index ], fontColors, colors );

		if( space.length === 0 && letterSpacing > 0 ) {
			Debugging.report( `AddLetterSpacing: Adding space to letter spacing`, 1 );

			space = ' ';
		}

		output[ i ] += String.repeat( space, letterSpacing );
	}

	return output;
};


/**
 * Abstraction for windows size
 *
 * @type {object}
 */
const Size = {
	width: WinSize
		? WinSize.width > 0
			? WinSize.width
			: 80
		: 80,
	height: WinSize
		? WinSize.height > 0
			? WinSize.height
			: 24
		: 24,
};


/**
 * Calculate the spaces to be added to the left of each line to align them either center or right
 *
 * @param  {array}   output         - The output array the line shall be appended to
 * @param  {integer} lineLength     - The current line length
 * @param  {integer} characterLines - The amount of line breaks in one character
 * @param  {string}  align          - The alignment of the text, only `center` and `right` will do anything
 * @param  {object}  size           - The size of the terminal as an object, default: Size
 * @param  {integer} size.width     - The width of the terminal
 * @param  {integer} size.height    - The height of the terminal
 *
 * @return {array}                  - The output array with space added on the left for alignment
 */
const AlignText = ( output, lineLength, characterLines, align, size = Size ) => {
	Debugging.report( `Running AlignText`, 1 );

	let space = 0;

	if( align === 'center' ) { // calculate the size for center alignment
		space = Math.floor( ( size.width - lineLength ) / 2 );

		Debugging.report( `AlignText: Center lineLength: ${ lineLength }, size.width: ${ size.width }, space: ${ space }`, 2 );
	}

	if( align === 'right' ) { // calculate the size for right alignment
		space = size.width - lineLength;

		Debugging.report( `AlignText: Right lineLength: ${ lineLength }, size.width: ${ size.width }, space: ${ space }`, 2 );
	}


	if( space > 0 ) { // only add if there is something to add
		let lines = output.length - characterLines; // last line is characterLines tall and is located at the bottom of the output array
		space = String.repeat(' ', space );

		for( let i = lines; i < output.length; i++ ) { // iterate over last line (which can be several line breaks long)
			output[ i ] = space + output[ i ];
		}
	}

	return output;
};


/**
 * Check input for human errors
 *
 * @param  {string} INPUT          - The string you want to write out
 * @param  {string} userFont       - The user specified font
 * @param  {array}  userColors     - The user specified colors
 * @param  {string} userBackground - The user specified background color
 * @param  {string} userAlign      - The user specified alignment option
 * @param  {object} fontfaces      - All allowed fontfaces
 * @param  {object} colors         - All allowed font colors
 * @param  {object} bgcolors       - All allowed background colors
 * @param  {array}  alignment      - All allowed alignments
 *
 * @typedef  {object} ReturnObject
 *   @property {boolean} pass      - Whether the input is valid
 *   @property {string}  message   - Possible error messages
 *
 * @return {ReturnObject}          - An object with error messages and a pass key
 */
const CheckInput = (
	INPUT,
	userFont,
	userColors,
	userBackground,
	userAlign,
	fontfaces = FONTFACES,
	colors = COLORS,
	bgcolors = BGCOLORS,
	alignment = ALIGNMENT
) => {
	let result = {
		message: '',
		pass: true,
	};

	// checking input
	if( INPUT === undefined || INPUT === '' ) {
		return {
			message: 'Please provide text to convert',
			pass: false,
		};
	}

	// checking font
	if( Object.keys( fontfaces ).indexOf( userFont.toLowerCase() ) === -1 ) {
		return {
			message: `"${ Chalk.red( userFont ) }" is not a valid font option.\n` +
				`Please use a font from the supported stack:\n${
					Chalk.green( Object.keys( fontfaces ).map( font => fontfaces[ font ] ).join(', ') )
				}`,
			pass: false,
		};
	}

	// checking colors
	userColors.forEach( color => { // check color usage
		if(
			Object.keys( colors ).indexOf( color.toLowerCase() ) === -1
			&& color !== 'candy'
		) {
			result = {
				message: `"${ Chalk.red( color ) }" is not a valid font color option.\n` +
					`Please use a color from the supported stack:\n${ Chalk.green(`${
						Object.keys( colors ).map( color => colors[ color ] ).join(', ')
					}, candy`) }`,
				pass: false,
			};
		}
	});

	// checking background colors
	if( Object.keys( bgcolors ).indexOf( userBackground.toLowerCase() ) === -1 ) {
		return {
			message: `"${ Chalk.red( userBackground ) }" is not a valid background option.\n` +
				`Please use a color from the supported stack:\n${
					Chalk.green( Object.keys( bgcolors ).map( bgcolor => bgcolors[ bgcolor ] ).join(', ') )
				}`,
			pass: false,
		};
	}

	// CHECKING ALIGNMENT
	if( alignment.indexOf( userAlign.toLowerCase() ) === -1 ) {
		return {
			message: `"${ Chalk.red( userAlign ) }" is not a valid alignment option.\n` +
				`Please use an alignment option from the supported stack:\n${
					Chalk.green( alignment.join(' | ') )
				}`,
			pass: false,
		};
	}

	return result;
};


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
		space = String.repeat( ' ', OPTIONS.letterSpacing );
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


/**
 * Filter only allowed character
 *
 * @param  {string} INPUT - The input text to be filtered
 * @param  {array}  chars - An array of all allowed characters
 *
 * @return {string}       - The filtered input text
 */
const CleanInput = ( INPUT, chars = CHARS ) => {
	if( typeof INPUT === 'string' ) {
		const clean = INPUT
			.replace(/(?:\r\n|\r|\n)/g, '|')
			.split('')
			.filter( char => chars.includes( char.toUpperCase() ) )
			.join('');

		return clean;
	}
	else {
		return '';
	}
};


/**
 * Merge user settings with default options
 *
 * @param  {SETTINGS} SETTINGS       - Some or all of the allowed settings
 * @param  {array}     allowedColors - All allowed font colors
 * @param  {array}     allowedBG     - All allowed background colors
 * @param  {array}     allowedFont   - All allowed fontfaces
 *
 * @typedef  {object} SETTINGS
 *   @param  {string}  font          - Font face, Default 'block'
 *   @param  {string}  align         - Text alignment, Default: 'left'
 *   @param  {array}   colors        - Colors for font, Default: []
 *   @param  {string}  background    - Chalk color string for background, Default 'Black'
 *   @param  {integer} letterSpacing - Space between letters, Default: set by selected font face
 *   @param  {integer} lineHeight    - Space between lines, Default: 1
 *   @param  {boolean} space         - Output space before and after output, Default: true
 *   @param  {integer} maxLength     - Maximum amount of characters per line, Default width of console window
 *
 * @return {object}                  - Our merged options
 */
const GetOptions = (
	{ font, align, colors, background, backgroundColor, letterSpacing, lineHeight, space, maxLength },
	allowedColors = COLORS,
	allowedBG = BGCOLORS,
	allowedFont = FONTFACES
) => ({
	font: font === undefined
		? 'block'
		: allowedFont[ font.toLowerCase() ] || font,
	align: align === undefined
		? 'left'
		: align.toLowerCase(),
	colors: Array.isArray( colors )
		? colors.map( color => allowedColors[ color.toLowerCase() ] || color )
		: [],
	background: background === undefined && backgroundColor === undefined
		? 'transparent'
		: background === undefined
			? allowedBG[ backgroundColor.toLowerCase() ] || backgroundColor
			: allowedBG[ background.toLowerCase() ] || background,
	letterSpacing: typeof parseInt( letterSpacing ) === 'number' && letterSpacing > 0
		? letterSpacing
		: 1,
	lineHeight: lineHeight === undefined
		? 1
		: parseInt( lineHeight ),
	space: typeof space === 'boolean'
		? space
		: true,
	maxLength: maxLength || 0,
});


/**
 * Main method to get the ANSI output for a string
 *
 * @param  {string}  input       - The string you want to write out
 * @param  {object}  SETTINGS    - Settings object
 * @param  {boolean} debug       - A flag to enable debug mode
 * @param  {integer} debuglevel  - The debug level we want to show
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {integer} size.width  - The width of the terminal
 * @param  {integer} size.height - The height of the terminal
 *
 * @typedef  {object} ReturnObject
 *   @property {string}  string  - The pure string for output with all line breaks
 *   @property {array}   array   - Each line of output in an array
 *   @property {integer} lines   - The number of lines
 *   @property {object}  options - All options used
 *
 * @return {ReturnObject}        - CLI output of INPUT to be consoled out
 */
const Render = ( input, SETTINGS = {}, debug = DEBUG, debuglevel = DEBUGLEVEL, size = Size ) => {
	Debugging.report(`Running render`, 1);

	DEBUG = debug;
	DEBUGLEVEL = debuglevel;

	const INPUT = CleanInput( input, CHARS );
	const OPTIONS = GetOptions( SETTINGS );

	let output = [];   // for output where each line is an output line
	let lines = 0;     // for counting each line
	let FONTFACE = {}; // scoping the fontface object higher for fonts with just one color

	const _isGoodHuman = CheckInput( INPUT, OPTIONS.font, OPTIONS.colors, OPTIONS.background, OPTIONS.align );
	if( !_isGoodHuman.pass ) {
		Log.error( _isGoodHuman.message );

		return false;
	}


	// display an overview of options if debug flag is enabled
	if( DEBUG ) {
		let outOption = `OPTIONS:\n  Text: ${ INPUT }`;

		for( let key in OPTIONS ) {
			outOption += `\n  Options.${ key }: ${ OPTIONS[ key ] }`;
		}

		Debugging.report( outOption, 3 );
	}


	if( OPTIONS.font === 'console' ) { // console fontface is pretty easy to process
		FONTFACE = {
			colors: 1,
			lines: 1,
		};

		const consoleOutput = RenderConsole( INPUT, OPTIONS, size );

		output = consoleOutput.output;
		lines = consoleOutput.lines;
	}
	else { // all other fontfaces need the font-file and some more work
		FONTFACE = GetFont( OPTIONS.font );

		if( !FONTFACE ) {
			Log.error( `Font file for the font "${ font }" could not be found.\nTry reinstalling this package.` );

			return false;
		}

		// setting the letterspacing preference from font face if there is no user overwrite
		if( SETTINGS.letterSpacing === undefined ) {
			Debugging.report( `Looking up letter spacing from font face`, 1 );

			let width = 0;

			FONTFACE.letterspace.forEach( item => {
				let char = item.replace( /(<([^>]+)>)/ig, '' ); // get character and strip color infos

				if( width < char.length ) {
					width = char.length;
				}
			});

			Debugging.report(`Letter spacing set to font face default: "${ width }"`, 2);
			OPTIONS.letterSpacing = width;
		}

		let lineLength = CharLength( FONTFACE.buffer, FONTFACE.lines, OPTIONS ); // count each output character per line and start with the buffer
		let maxChars = 0; // count each character we print for maxLength option

		output = AddLine( [], FONTFACE.lines, FONTFACE.buffer, OPTIONS.lineHeight ); // create first lines with buffer
		lines ++;

		output = AddLetterSpacing( output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing ); // add letter spacing to the beginning
		lineLength += CharLength( FONTFACE.letterspace, FONTFACE.lines, OPTIONS ) * OPTIONS.letterSpacing; // count the space for the letter spacing

		for( let i = 0; i < INPUT.length; i++ ) { // iterate through the message
			let CHAR = INPUT.charAt( i ).toUpperCase(); // the current character we convert, only upper case is supported at this time
			let lastLineLength = lineLength; // we need the lineLength for alignment before we look up if the next char fits

			Debugging.report(`Character found in font: "${ CHAR }"`, 2);

			if( CHAR !== `|` ) { // what will the line length be if we add the next char?
				lineLength += CharLength( FONTFACE.chars[ CHAR ], FONTFACE.lines, OPTIONS ); // get the length of this character
				lineLength += CharLength( FONTFACE.letterspace, FONTFACE.lines, OPTIONS ) * OPTIONS.letterSpacing; // new line, new line length
			}

			// jump to next line after OPTIONS.maxLength characters or when line break is found or the console windows would have ran out of space
			if( maxChars >= OPTIONS.maxLength && OPTIONS.maxLength != 0 || CHAR === `|` || lineLength > size.width ) {
				lines ++;

				Debugging.report(
					`NEWLINE: maxChars: ${ maxChars }, ` +
					`OPTIONS.maxLength: ${ OPTIONS.maxLength }, ` +
					`CHAR: ${ CHAR }, ` +
					`lineLength: ${ lineLength }, ` +
					`Size.width: ${ size.width } `, 2
				);

				output = AlignText( output, lastLineLength, FONTFACE.lines, OPTIONS.align, size ); // calculate alignment based on lineLength

				lineLength = CharLength( FONTFACE.buffer, FONTFACE.lines, OPTIONS ); // new line: new line length
				lineLength += CharLength( FONTFACE.letterspace, FONTFACE.lines, OPTIONS ) * OPTIONS.letterSpacing; // each new line starts with letter spacing

				if( CHAR !== `|` ) { // if this is a character and not a line break
					lineLength += CharLength( FONTFACE.chars[ CHAR ], FONTFACE.lines, OPTIONS ); // get the length of this character
					lineLength += CharLength( FONTFACE.letterspace, FONTFACE.lines, OPTIONS ) * OPTIONS.letterSpacing; // add letter spacing at the end
				}

				maxChars = 0; // new line, new maxLength goal

				output = AddLine( output, FONTFACE.lines, FONTFACE.buffer, OPTIONS.lineHeight ); // adding new line
				// add letter spacing to the beginning
				output = AddLetterSpacing( output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing );
			}

			Debugging.report(`lineLength at: "${ lineLength }"`, 2);

			if( CHAR !== `|` ) {
				maxChars++; // counting all printed characters
				output = AddChar( CHAR, output, FONTFACE.lines, FONTFACE.chars, FONTFACE.colors, OPTIONS.colors ); // add new character
				output = AddLetterSpacing( output, FONTFACE.lines, FONTFACE.letterspace, FONTFACE.colors, OPTIONS.colors, OPTIONS.letterSpacing );
			}
		}

		output = AlignText( output, lineLength, FONTFACE.lines, OPTIONS.align, size ); // alignment last line
	}

	let write = output.join(`\n`);


	if( FONTFACE.colors <= 1 ) {
		write = Colorize( write, FONTFACE.colors, OPTIONS.colors );
	}


	if( OPTIONS.space ) { // add space
		write = `\n\n${ write }\n\n`;
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
const Say = ( INPUT, SETTINGS = {}, debug = DEBUG, debuglevel = DEBUGLEVEL, size = Size ) => {
	Debugging.report(`Running say`, 1);

	DEBUG = debug;
	DEBUGLEVEL = debuglevel;

	let write = Render( INPUT, SETTINGS, debug, debuglevel, size );

	if( write ) {
		console.log( write.string ); // write out
	}
};


/**
 * Flatten the shortcuts in our cli options object
 *
 * @param  {object} options - An object objects with a short key
 *
 * @return {object}         - All short keys flattened into first level
 */
const AddShortcuts = ( options ) => {
	const flatOptions = Object.assign( {}, options );

	Object.keys( flatOptions ).forEach( option => {
		flatOptions[ option ]._name = option;
		flatOptions[ flatOptions[ option ].short ] = flatOptions[ option ];
	});

	return flatOptions;
};


/**
 * Parse cli arguments into a nice object
 *
 * @param  {array} inputOptions - All possible options registered for this app
 * @param  {array} inputArgs    - The arguments given to us in our cli, default: process.argv
 *
 * @return {object}             - An object of all options with at least their default values
 */
const ParseArgs = ( inputOptions = CLIOPTIONS, inputArgs = process.argv ) => {
	const parsedArgs = {
		text: inputArgs[ 2 ],
	};

	// create defaults
	Object.keys( inputOptions ).forEach( option => {
		const name = option.replace( '--', '' );

		parsedArgs[ name ] = inputOptions[ option ].default;
	});

	if( inputArgs[ 2 ] === '--help' || inputArgs[ 2 ] === '-h' ) {
		parsedArgs.help = true;
	}

	if( inputArgs[ 2 ] === '--version' || inputArgs[ 2 ] === '-v' ) {
		parsedArgs.version = true;
	}

	const args = inputArgs.splice( 3 ); // the first two are node specific, the third is our text

	const options = AddShortcuts( inputOptions );

	for( let index = 0; args.length > index; index ++ ) {
		const option = options[ args[ index ] ];

		if( option ) {
			const name = option._name.replace( '--', '' );

			if( option.options !== undefined ) {
				index ++;
				const value = args[ index ];

				if( option.options === true ) {
					parsedArgs[ name ] = value;
				}
				else {
					if( option.options.includes( value ) ) {
						parsedArgs[ name ] = value;
					}
					else {
						parsedArgs[ name ] = option.default;
					}
				}
			}
			else {
				parsedArgs[ name ] = true;
			}
		}
		else {
			Debugging.report( `The cli argument ${ args[ index ] } was not found and ignored`, 2 );
		}
	};

	return parsedArgs;
};


/**
 * Display the help generated from our CLIOPTIONS
 */
const DisplayHelp = () => {
	console.log(
		` ${ Render( 'cfonts', { align: 'center', colors: ['redBright','greenBright'] } ).string }` +
		`This is a tool for sexy fonts in the console. Give your cli some love.\n\n` +
		`Usage: cfonts "<value>" [option1] <input1> [option2] <input1>,<input2> [option3]\n` +
		`Example: ${ Chalk.bold('$ cfonts "sexy font" -f chrome -a center -c red,green,gray') }\n\n` +
		`Options:\n`
	);

	let command = [];
	let largestSize = 0;

	Object.keys( CLIOPTIONS ).forEach( option => {
		console.log( Chalk.bold(`${ option }, ${ CLIOPTIONS[ option ].short }`) );
		console.log( CLIOPTIONS[ option ].description );
		console.log(`${ Chalk.bold('$') } cfonts ${ option }${
			typeof CLIOPTIONS[ option ].options !== 'boolean' && typeof CLIOPTIONS[ option ].options !== 'undefined'
				? Chalk.green(` ( ${ CLIOPTIONS[ option ].options.join(', ') } )`)
				: ''
		}\n`);
	});
};


/**
 * Display the version of this package
 */
const DisplayVersion = () => {
	console.log( PACKAGE.version );
};


/**
 * Run cli commands
 *
 * @param  {array} inputOptions - All possible options registered for this app
 * @param  {array} inputArgs    - The arguments given to us in our cli, default: process.argv
 */
const Cli = ( inputOptions = CLIOPTIONS, inputArgs = process.argv ) => {
	const args = ParseArgs( inputOptions, inputArgs );

	Debugging.report(
		`OPTIONS:\n` +
		`  CFonts.say("${ args.text }", {\n` +
		`    font: "${ args.font }",\n` +
		`    align: "${ args.align }",\n` +
		`    colors: ${ args.colors ? JSON.stringify( args.colors.split(',') ) : [] },\n` +
		`    background: "${ args.background }",\n` +
		`    letterSpacing: ${ args['letter-spacing'] },\n` +
		`    lineHeight: ${ args['line-height'] },\n` +
		`    space: ${ !args.spaceless },\n` +
		`    maxLength: ${ args['max-length'] }\n` +
		`  }, ${ args.debug }, ${ args.debugLevel } );`,
		3,
		args.debug,
		args.debugLevel
	);

	if( args.help ) {
		DisplayHelp();
		return;
	}

	if( args.version ) {
		DisplayVersion();
		return;
	}

	if( !args.text ) {
		Log.error(
			`Please provide text to convert with ${ Chalk.green(`cfonts "Text"`) }\n` +
			`Run ${ Chalk.green(`cfonts --help`) } for more infos`
		);
		return;
	}

	Say( args.text, {
		font: args.font,
		align: args.align,
		colors: args.colors ? args.colors.split(',') : [],
		background: args.background,
		letterSpacing: args['letter-spacing'],
		lineHeight: args['line-height'],
		space: !args.spaceless,
		maxLength: args['max-length'],
	}, args.debug, args.debugLevel );
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
	headline: ( text, level = 99, debug = DEBUG, debuglevel = DEBUGLEVEL ) => {
		if( debug && level >= debuglevel ) {
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
	report: ( text, level = 99, debug = DEBUG, debuglevel = DEBUGLEVEL ) => {
		if( debug && level >= debuglevel ) {
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
	error: ( text, level = 99, debug = DEBUG, debuglevel = DEBUGLEVEL ) => {
		if( debug && level >= debuglevel ) {
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
		text = text.replace( /(?:\r\n|\r|\n)/g, '\n       ' ); // indent each line

		console.error(`\n ${ Chalk.bold.red('Ouch:') } ${ text }\n`);
	},
};


// Export for API use and unit tests
module.exports = exports = {
	render: Render,
	say: Say,

	__test__: {
		DEBUG,
		DEBUGLEVEL,
		CHARS,
		COLORS,
		BGCOLORS,
		ALIGNMENT,
		FONTFACES,
		CLIOPTIONS,
		PACKAGE,
		AddShortcuts,
		GetFont,
		CharLength,
		AddLine,
		AnsiSytle,
		Colorize,
		AddChar,
		AddLetterSpacing,
		Size,
		AlignText,
		CheckInput,
		RenderConsole,
		CleanInput,
		GetOptions,
		ParseArgs,
		DisplayHelp,
		DisplayVersion,
		Cli,
		Debugging,
		Log,
	},
};
