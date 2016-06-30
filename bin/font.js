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


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Dependencies
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
const Fs = require('fs');
const Chalk = require('chalk');
const Program = require('commander');
const CFonts = require('./../index.js');

const Package = JSON.parse(Fs.readFileSync(__dirname + '/../package.json', 'utf8'));
const Version = Package.version;


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Setting up command line tool
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
Program
	.description(`This is a tool for sexy fonts in the console. Give your cli some love.`)
	.version(`v${Version}`)
	.usage(`"<value>" [option1] <input1> [option2] <input1>,<input2> [option3]`)
	.option(`-f, --font            <keyword>`,               `define "font face"`, `block`)
	.option(`-a, --align           <keyword>`,               `define "alignment" for the text`, `left`)
	.option(`-c, --colors          <keyword>,<keyword>...`,  `provide colors for text`, `white`)
	.option(`-b, --background      <keyword>`,               `provide background color`, `Black`)
	.option(`-l, --letter-spacing  <n>`,                     `define letter spacing {integer}`)
	.option(`-z, --line-height     <n>`,                     `define line height {integer}`, 1)
	.option(`-s, --spaceless`,                               `surpress space on top and on the bottom`)
	.option(`-m, --max-length     <keyword>`,                `define how many character can be on one line`)
	.action(function( text ) {
			Program.text = text; //add flagless option for text
	 })
	.on('--help', function() { //adding options for each keyword section
		console.log( Chalk.bold(`  Font face options:`) );
		console.log(`  [ ${CFonts.FONTFACES.join(', ')} ]\n`);

		console.log( Chalk.bold(`  Alignment options:`) );
		console.log(`  [ ${CFonts.ALIGNMENT.join(', ')} ]\n`);

		console.log( Chalk.bold(`  Color options:`) );
		console.log(`  [ ${CFonts.COLORS.join(', ')} ]\n`);

		console.log( Chalk.bold(`  background color options:`) );
		console.log(`  [ ${CFonts.BGCOLORS.join(', ')} ]\n`);
	})
	.parse( process.argv );


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Execute programm
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
if(Program.text !== undefined) {
	//log OPTIONS for debugging
	if( CFonts.DEBUG ) {
		CFonts.debugging.report(
			`OPTIONS:\n` +
			`  CFonts.say("${Program.text}", {\n` +
			`	'font': "${Program.font}",\n` +
			`	'align': "${Program.align}",\n` +
			`	'colors': ${Program.colors ? JSON.stringify( Program.colors.split(',') ) : []},\n` +
			`	'background': "${Program.background}",\n` +
			`	'letterSpacing': ${Program.letterSpacing},\n` +
			`	'lineHeight': ${Program.lineHeight},\n` +
			`	'space': ${Program.spaceless ? false : true},\n` +
			`	'maxLength': ${Program.maxLength}\n` +
			`  });`,
			3
		);
	}

	//execute cfonts
	CFonts.say(Program.text, {
		'font': Program.font,
		'align': Program.align,
		'colors': Program.colors ? Program.colors.split(',') : [],
		'background': Program.background,
		'letterSpacing': Program.letterSpacing,
		'lineHeight': Program.lineHeight,
		'space': Program.spaceless ? false : true,
		'maxLength': Program.maxLength
	});

}
else { //we do need text to convert
	CFonts.log.error(
		`Please provide text to convert with ${Chalk.green(`cfonts -t "Text"`)}\n` +
		`Run ${Chalk.green(`cfonts --help`)} for more infos`
	);
}