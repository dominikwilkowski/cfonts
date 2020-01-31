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
 * Cli
 *   Run cli commands
 *
 **************************************************************************************************************************************************************/

'use strict';

const { AddLetterSpacing } = require('./AddLetterSpacing.js');
const { DisplayVersion } = require('./DisplayVersion.js');
const { DisplayHelp } = require('./DisplayHelp.js');
const { CLIOPTIONS } = require('./constants.js');
const { Debugging } = require('./Debugging.js');
const { ParseArgs } = require('./ParseArgs.js');
const { Render } = require('./Render.js');
const { Chalk } = require('./Chalk.js');
const { Log } = require('./Log.js');
const { Say } = require('./Say.js');


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
		gradient: args.gradient,
		independentGradient: args['independent-gradient'],
	}, args.debug, args.debugLevel );
};


module.exports = exports = {
	render: Render,
	say: Say,
	Cli,
};
