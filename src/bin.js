#!/usr/bin/env node
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
const CFonts = require('../lib/index.js');
const Chalk = require('chalk');


const CLI = ( inputOptions = CFonts.CLIOPTIONS, inputArgs = process.argv ) => {
	const args = CFonts.ParseArgs( inputOptions, inputArgs );

	CFonts.Debugging.report(
		`OPTIONS:\n` +
		`  CFonts.say("${ args.text }", {\n` +
		`    font: "${ args.font }",\n` +
		`    align: "${ args.align }",\n` +
		`    colors: ${ args.colors ? JSON.stringify( args.colors.split(',') ) : [] },\n` +
		`    background: "${ args.background }",\n` +
		`    letterSpacing: ${ args.letterSpacing },\n` +
		`    lineHeight: ${ args.lineHeight },\n` +
		`    space: ${ args.spaceless },\n` +
		`    maxLength: ${ args.maxLength }\n` +
		`  }, ${ args.debug }, ${ args.debugLevel } );`,
		3,
		args.debug,
		args.debugLevel
	);

	if( args.help ) {
		CFonts.DisplayHelp();
		return;
	}

	if( args.version ) {
		CFonts.DisplayVersion();
		return;
	}

	if( !args.text ) {
		CFonts.Log.error(
			`Please provide text to convert with ${ Chalk.green(`cfonts "Text"`) }\n` +
			`Run ${ Chalk.green(`cfonts --help`) } for more infos`
		);
	}

	CFonts.say( args.text, {
		font: args.font,
		align: args.align,
		colors: args.colors ? args.colors.split(',') : [],
		background: args.background,
		letterSpacing: args.letterSpacing,
		lineHeight: args.lineHeight,
		space: args.spaceless,
		maxLength: args.maxLength
	}, args.debug, args.debugLevel );
};

CLI();
