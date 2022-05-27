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
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 **************************************************************************************************************************************************************/

'use strict';

// Dependencies
const { CHARS: CFontsChars, FONTFACES: CFontsFontfaces } = require('../src/constants.js');
const { Size } = require('../src/Size.js');
const Readline = require('readline');
const Chalk = require(`chalk`);
const Path = require('path');
const Fs = require(`fs`);

// global defaults
const DEBUG = true;
const DEBUGLEVEL = 2;
const CHARS = CFontsChars.filter((font) => font !== '|'); // we don’t need the pipe in the char-set
const FONTFACES = Object.keys(CFontsFontfaces).map((font) => CFontsFontfaces[font]); // console is a font but not a font-file

/**
 * Test each font for common issues
 *
 * @param  {array} FONTFACES - All font faces to be checked in an array
 * @param  {array} CHARS     - All characters that should be included
 */
const FontTest = (FONTFACES, CHARS) => {
	Debugging.report(`Running FontTest`, 1);

	let font = '';
	let fontFile = '';
	let FONTFACE = {};

	for (let i in FONTFACES) {
		font = FONTFACES[i];
		fontFile = Path.normalize(`${__dirname}/../../fonts/${font}.json`);

		Log.headline(`${font}`);
		Log.check(`Checking: "${font}" existence`);

		try {
			FONTFACE = JSON.parse(Fs.readFileSync(fontFile, 'utf8'));

			Log.subdone(`FOUND!`);
		} catch (e) {
			Log.suberror(`NOT FOUND: "${fontFile}"`);
		}

		Attributes(FONTFACE);
		Letterspace_size(FONTFACE);
		Chars(FONTFACE, CHARS);
		Width(FONTFACE, CHARS);
		Lines(FONTFACE, CHARS);
	}

	console.log(`\n`);
	if (Log.errors > 0) {
		process.exit(1);
	}
};

/**
 * Test the font has all attributes
 *
 * @param  {object} FONTFACE - The font object for this font
 */
const Attributes = (FONTFACE) => {
	Debugging.report(`Running Attributes`, 1);
	Log.check(`Checking font attributes of "${FONTFACE.name}"`);
	let fails = [];

	if (FONTFACE.name === undefined) {
		fails.push(`name`);
	}

	if (FONTFACE.version === undefined) {
		fails.push(`version`);
	}

	if (FONTFACE.homepage === undefined) {
		fails.push(`homepage`);
	}

	if (FONTFACE.colors === undefined) {
		fails.push(`colors`);
	}

	if (FONTFACE.lines === undefined) {
		fails.push(`lines`);
	}

	if (FONTFACE.buffer === undefined) {
		fails.push(`buffer`);
	}

	if (FONTFACE.letterspace === undefined) {
		fails.push(`letterspace`);
	}

	if (FONTFACE.letterspace_size === undefined) {
		fails.push(`letterspace_size`);
	}

	if (FONTFACE.chars === undefined) {
		fails.push(`chars`);
	}

	if (fails.length > 0) {
		Log.suberror(`Font has missing attributes: "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
	} else {
		Log.subdone(`All THERE!`);
	}
};

/**
 * Test the font to include all characters
 *
 * @param  {object} FONTFACE - The font object for this font
 * @param  {array}  CHARS    - All characters that should be included
 */
const Letterspace_size = (FONTFACE) => {
	Debugging.report(`Running Letterspace_size`, 1);
	Log.check(`Checking letterspace_size in "${FONTFACE.name}"`);

	let width = 0;
	FONTFACE.letterspace.forEach((item) => {
		let char = item.replace(/(<([^>]+)>)/gi, ''); // get character and strip color infos

		if (width < char.length) {
			width = char.length;
		}
	});

	if (FONTFACE.letterspace_size !== width) {
		Log.suberror(
			`Font has wrong letterspace_size attribute. Is: "${FONTFACE.letterspace_size}" but should be "${width}" in font: "${FONTFACE.name}"`
		);
	} else {
		Log.subdone(`All CORRECT!`);
	}
};

/**
 * Test the font to include all characters
 *
 * @param  {object} FONTFACE - The font object for this font
 * @param  {array}  CHARS    - All characters that should be included
 */
const Chars = (FONTFACE, CHARS) => {
	Debugging.report(`Running Chars`, 1);
	Log.check(`Checking all characters in "${FONTFACE.name}"`);
	let fails = [];

	for (let i in CHARS) {
		let char = CHARS[i];

		if (FONTFACE.chars[char] === undefined) {
			fails.push(char);
		}
	}

	if (fails.length > 0) {
		Log.suberror(`Character could not be found: "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
	} else {
		Log.subdone(`All THERE!`);
	}
};

/**
 * Test each character has the right amount of lines
 *
 * @param  {object} FONTFACE - The font object for this font
 * @param  {array}  CHARS    - All characters that should be included
 */
const Lines = (FONTFACE, CHARS) => {
	Debugging.report(`Running Lines`, 1);
	Log.check(`Checking all character lines in "${FONTFACE.name}"`);
	let fails = [];

	if (FONTFACE.buffer.length !== FONTFACE.lines) {
		fails.push(`buffer`);
	}

	if (FONTFACE.letterspace.length !== FONTFACE.lines) {
		fails.push(`letterspace`);
	}

	for (let i in CHARS) {
		let char = CHARS[i];

		if (FONTFACE.chars[char] !== undefined) {
			if (FONTFACE.chars[char].length !== FONTFACE.lines) {
				fails.push(char);
			}
		}
	}

	if (fails.length > 0) {
		Log.suberror(`Character lines are not correct in "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
	} else {
		Log.subdone(`All CORRECT!`);
	}
};

/**
 * Test each character to have the same width
 *
 * @param  {object} FONTFACE - The font object for this font
 * @param  {array}  CHARS    - All characters that should be included
 */
const Width = (FONTFACE, CHARS) => {
	Debugging.report(`Running Width`, 1);
	Log.check(`Checking all character widths in "${FONTFACE.name}"`);
	let fails = [];

	for (let i in CHARS) {
		let char = CHARS[i];

		if (FONTFACE.chars[char] !== undefined) {
			let character = FONTFACE.chars[char];
			let width = 0;

			for (let i = 0; i < character.length; i++) {
				character[i] = character[i].replace(/(<([^>]+)>)/gi, '');

				if (i === 0) {
					width = character[i].length;
				}

				if (width !== character[i].length) {
					fails.push(`${char}((${width}) ${i}=${character[i].length})`);
					// break; // if we break here we fail on the first issue rather than collecting them all
				}
			}
		}
	}

	if (fails.length > 0) {
		Log.suberror(`Character width is not consistent in "${fails.join(' ')}" in font: "${FONTFACE.name}"`);
	} else {
		Log.subdone(`All CONSISTENT!`);
	}
};

/**
 * A collection of methods to print debug message that will be logged to console if debug mode is enabled
 *
 * @type {Object}
 */
const Debugging = {
	/**
	 * Print a headline preferably at the beginning of your app
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	headline: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.bold(' \u2611  ')} ${text}`));
		}
	},

	/**
	 * Print a message to report starting a process
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	report: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.bold.green(' \u2611  ')} ${Chalk.black(`${text} `)}`));
		}
	},

	/**
	 * Print a message to report an error
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	error: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.red(' \u2612  ')} ${Chalk.black(`${text} `)}`));
		}
	},

	/**
	 * Print a message to report an interaction
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	interaction: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.blue(' \u261C  ')} ${Chalk.black(`${text} `)}`));
		}
	},

	/**
	 * Print a message to report data has been sent
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	send: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.bold.cyan(' \u219D  ')} ${Chalk.black(`${text} `)}`));
		}
	},

	/**
	 * Print a message to report data has been received
	 *
	 * @param  [text]  {string}  - The sting you want to log
	 * @param  [level] {integer} - (optional) The debug level. Show equal and greater levels. Default: 99
	 */
	received: (text, level = 99) => {
		if (DEBUG && level >= DEBUGLEVEL) {
			console.log(Chalk.bgWhite(`\n${Chalk.bold.cyan(' \u219C  ')} ${Chalk.black(`${text} `)}`));
		}
	},
};

/**
 * A collection of methods to print messages to console
 *
 * @type {Object}
 */
const Log = {
	errors: 0,

	/**
	 * Start a test category
	 *
	 * @param  {string} text - The name of the test
	 */
	headline: (text) => {
		let space = Math.floor((Size.width - text.length - 6) / 2);
		if (space < 0) {
			space = 1;
		}

		console.log(`\n${Chalk.bgWhite.black(`\n${' '.repeat(space)}══ ${text} ══`)}`);
	},

	/**
	 * Log a check and keep space for the result
	 *
	 * @param  {string} text - The sting you want to log
	 */
	check: (text) => {
		let prefix = `${Chalk.bold.green(' \u231B     ')} ${Chalk.bold.black('Testing:')} `;

		text = text.replace(/(?:\r\n|\r|\n)/g, `\n${' '.repeat(prefix.length)}`);
		process.stdout.write(`\n${Chalk.bgWhite(`${prefix}${Chalk.black(text)}`)}`);
	},

	/**
	 * Note positive outcome for previously run check method
	 *
	 * @param  {string} text - Success message (not logged)
	 */
	subdone: (text) => {
		let prefix = ` ${Chalk.bold.black('OK')} `;
		text = text.replace(/(?:\r\n|\r|\n)/g, `\n   ${' '.repeat(prefix.length)}`);

		Readline.cursorTo(process.stdout, 0);
		console.log(`${Chalk.bgGreen(` ${prefix}`)}`);
		// console.log(`${ Chalk.bgGreen(`   ${ Chalk.black( text ) }`) }`); // not outputting message will keep things cleaner
	},

	/**
	 * Note negative outcome for previously run check method
	 *
	 * @param  {string} text - Error message
	 */
	suberror: (text) => {
		Log.errors++;

		let prefix = ` ${Chalk.bold.black('FAIL')} `;
		text = text.replace(/(?:\r\n|\r|\n)/g, `\n   ${' '.repeat(prefix.length)}`);

		Readline.cursorTo(process.stdout, 0);
		console.log(`${Chalk.bgRed(` ${prefix}`)}`);
		console.log(`${Chalk.bgRed(`   ${Chalk.black(text)}`)}`);
	},
};

FontTest(FONTFACES, CHARS);
