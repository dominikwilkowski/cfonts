/***************************************************************************************************************************************************************
 *
 * cfonts, Sexy fonts for the console. (CLI output)
 *
 * Testing the each font file:
 * - Font file has all font attributes?
 * - All characters included?
 * - All characters have the correct width?
 * - All characters have consistent lines?
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE  GNU GPLv2
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 **************************************************************************************************************************************************************/

'use strict';


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Dependencies
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
const CFonts = require('../lib/index.js');
const WinSize = require('window-size');
const Readline = require('readline');
const Chalk = require(`chalk`);
const Path = require('path');
const Fs = require(`fs`);


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Constructor
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
const FontTest = (() => { //constructor factory

	return {
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// settings
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		DEBUG: true,
		DEBUGLEVEL: 2,
		CHARS: CFonts.CHARS.filter( font => font !== '|' ), // we won't have the pip in the char-set
		FONTFACES: Object.keys( CFonts.FONTFACES ).map( font => CFonts.FONTFACES[ font ] ).filter( font => font !== 'console' ), // console is a font but not a font-file


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Public function
// fonts, test each font is there
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		fonts: () => {
			FontTest.debugging.report(`Running fonts`, 1);

			let font = '';
			let fontFile = '';
			let FONTFACE = {};

			for( let i in FontTest.FONTFACES ) { //iterate over all installed fonts
				font = FontTest.FONTFACES[ i ];
				fontFile = Path.normalize( `${__dirname}/../fonts/${font}.json` ); //build font path

				FontTest.log.headline(`${font}`);

				FontTest.log.check(`Checking: "${font}" existence`);

				try {
					FONTFACE = JSON.parse( Fs.readFileSync(fontFile, 'utf8') ); //read font file

					FontTest.log.subdone(`FOUND!`);
				}
				catch( e ) {
					FontTest.log.suberror(`NOT FOUND: "${fontFile}"`);
				}

				FontTest.attributes( FONTFACE );  //for this font run attributes test
				FontTest.chars( FONTFACE );       //for this font run char test
				FontTest.width( FONTFACE );       //for this font run width test
				FontTest.lines( FONTFACE );       //for this font run line test
			}

			console.log(`\n`);
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Public function
// attributes, test the font has all attributes it needs
//
// @param  FONTFACE  {object}  The font object to be checked
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		attributes: ( FONTFACE ) => {
			FontTest.debugging.report(`Running attributes`, 1);
			FontTest.log.check(`Checking font attributes of "${FONTFACE.name}"`);
			let fails = [];

			if( FONTFACE.name === undefined ) {
				fails.push(`name`);
			}

			if( FONTFACE.version === undefined ) {
				fails.push(`version`);
			}

			if( FONTFACE.homepage === undefined ) {
				fails.push(`homepage`);
			}

			if( FONTFACE.colors === undefined ) {
				fails.push(`colors`);
			}

			if( FONTFACE.lines === undefined ) {
				fails.push(`lines`);
			}

			if( FONTFACE.buffer === undefined ) {
				fails.push(`buffer`);
			}

			if( FONTFACE.letterspace === undefined ) {
				fails.push(`letterspace`);
			}

			if( FONTFACE.chars === undefined ) {
				fails.push(`chars`);
			}

			if( fails.length > 0 ) {
				FontTest.log.suberror(`Font has missing attributes: "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
			}
			else {
				FontTest.log.subdone(`All THERE!`);
			}
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Public function
// chars, test the font to include all characters
//
// @param  FONTFACE  {object}  The font object to be checked
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		chars: ( FONTFACE ) => {
			FontTest.debugging.report(`Running chars`, 1);
			FontTest.log.check(`Checking all characters in "${FONTFACE.name}"`);
			let fails = [];

			for( let i in FontTest.CHARS ) { //each character
				let char = FontTest.CHARS[ i ];

				if( FONTFACE.chars[ char ] === undefined ) {
					fails.push( char );
				}
			}

			if( fails.length > 0 ) {
				FontTest.log.suberror(`Character could not be found: "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
			}
			else {
				FontTest.log.subdone(`All THERE!`);
			}
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Public function
// lines, test each character has the right amount of lines
//
// @param  FONTFACE  {object}  The font object to be checked
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		lines: ( FONTFACE ) => {
			FontTest.debugging.report(`Running lines`, 1);
			FontTest.log.check(`Checking all character lines in "${FONTFACE.name}"`);
			let fails = [];

			if( FONTFACE.buffer.length !== FONTFACE.lines ) {
				fails.push(`buffer`);
			}

			if( FONTFACE.letterspace.length !== FONTFACE.lines ) {
				fails.push(`letterspace`);
			}

			for( let i in FontTest.CHARS ) { //each character
				let char = FontTest.CHARS[ i ];

				if( FONTFACE.chars[ char ] !== undefined ) {

					if( FONTFACE.chars[ char ].length !== FONTFACE.lines ) {
						fails.push( char );
					}
				}
			}

			if( fails.length > 0 ) {
				FontTest.log.suberror(`Character lines are not correct in "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
			}
			else {
				FontTest.log.subdone(`All CORRECT!`);
			}
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Public function
// width, test each character to have the same width
//
// @param  FONTFACE  {object}  The font object to be checked
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		width: ( FONTFACE ) => {
			FontTest.debugging.report(`Running width`, 1);
			FontTest.log.check(`Checking all character widths in "${FONTFACE.name}"`);
			let fails = [];

			for( let i in FontTest.CHARS ) { //each character
				let char = FontTest.CHARS[ i ];

				if( FONTFACE.chars[ char ] !== undefined ) {
					let character = FONTFACE.chars[ char ];
					let width = 0;

					for(let i = 0; i < character.length; i++) { //each line of each character
						character[ i ] = character[ i ].replace(/(<([^>]+)>)/ig, ''); //get character and strip color infos

						if( i === 0 ) {
							width = character[ i ].length;
						}

						if( width !== character[ i ].length ) {
							fails.push(`${char}((${width}) ${i}=${character[ i ].length})`);
							// break;
						}
					};
				}
			}

			if( fails.length > 0 ) {
				FontTest.log.suberror(`Character width is not consistent in "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
			}
			else {
				FontTest.log.subdone(`All CONSISTENT!`);
			}
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Debugging prettiness
//
// debugging, Print debug message that will be logged to console.
//
// @method  headline                      Return a headline preferably at the beginning of your app
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//
// @method  report                        Return a message to report starting a process
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//
// @method  error                         Return a message to report an error
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//
// @method  interaction                   Return a message to report an interaction
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//
// @method  send                          Return a message to report data has been sent
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//
// @method  received                      Return a message to report data has been received
//          @param    [text]   {string}   The sting you want to log
//          @param    [level]  {integer}  (optional) The debug level. Show equal and greater levels. Default: 99
//          @return   [ansi]   {output}
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		debugging: {

			headline: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.bold(' \u2611  ')} ${text}`)
					);
				}
			},

			report: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.bold.green(' \u2611  ')} ${Chalk.black(`${text} `)}`)
					);
				}
			},

			error: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.red(' \u2612  ')} ${Chalk.black(`${text} `)}`)
					);
				}
			},

			interaction: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.blue(' \u261C  ')} ${Chalk.black(`${text} `)}`)
					);
				}
			},

			send: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.bold.cyan(' \u219D  ')} ${Chalk.black(`${text} `)}`)
					);
				}
			},

			received: ( text, level = 99 ) => {
				if( FontTest.DEBUG && level >= FontTest.DEBUGLEVEL ) {
					console.log(
						Chalk.bgWhite(`\n${Chalk.bold.cyan(' \u219C  ')} ${Chalk.black(`${text} `)}`)
					);
				}
			}
		},


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Logging prettiness
//
// log, Print error message to console.
//
// @method  headline                    Start a test category
//          @param    [text]  {string}  The name of the test
//          @return   [ansi]  {output}
//
// @method  check                       Log a check and keep space for the result
//          @param    [text]  {string}  The sting you want to log
//          @return   [ansi]  {output}
//
// @method  subdone                     Note positive outcome for previously run check method
//          @param    [text]  {string}  Success message (not logged)
//          @return   [ansi]  {output}
//
// @method  suberror                    Note negative outcome for previously run check method
//          @param    [text]  {string}  Error message
//          @return   [ansi]  {output}
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
		log: {
			headline: ( text ) => {
				let space = Math.floor( (WinSize.width - text.length - 6) / 2 );
				if( space < 0 ) {
					space = 1;
				}

				console.log(`\n${Chalk.bgWhite.black(`\n${' '.repeat( space )}══ ${text} ══`)}`);
			},

			check: ( text ) => {
				let prefix = `${Chalk.bold.green(' \u231B     ')} ${Chalk.bold.black('Testing:')} `;

				text = text.replace(/(?:\r\n|\r|\n)/g, `\n${' '.repeat( prefix.length )}`); //indent each line
				process.stdout.write(`\n${Chalk.bgWhite(`${prefix}${Chalk.black(text)}`)}`);
			},

			subdone: ( text ) => {
				let prefix = ` ${Chalk.bold.black('OK')} `;
				text = text.replace(/(?:\r\n|\r|\n)/g, `\n   ${' '.repeat( prefix.length )}`); //indent each line

				Readline.cursorTo( process.stdout, 0 );
				console.log(`${Chalk.bgGreen(` ${prefix}`)}`);
				// console.log(`${Chalk.bgGreen(`   ${Chalk.black(text)}`)}`); //not outputting message will keep things cleaner
			},

			suberror: ( text ) => {
				let prefix = ` ${Chalk.bold.black('FAIL')} `;
				text = text.replace(/(?:\r\n|\r|\n)/g, `\n   ${' '.repeat( prefix.length )}`); //indent each line

				Readline.cursorTo( process.stdout, 0 );
				console.log(`${Chalk.bgRed(` ${prefix}`)}`);
				console.log(`${Chalk.bgRed(`   ${Chalk.black(text)}`)}`);
			},
		},
	}

})();


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
FontTest.fonts(); //running the test for fonts
