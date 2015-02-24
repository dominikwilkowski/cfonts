/*
 * cfonts
 * https://github.com/dominikwilkowski/cfonts
 *
 * Copyright (c) 2015 Dominik Wilkowski
 * Licensed under the MIT license.
 */

// 'use strict';

//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Dependencies
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
var fs = require('fs');
var chalk = require('chalk');
var changeCase = require('change-case');


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Custom functions
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
function colorize($font, OPTIONS, $character) {

	if($font.colors > 1) {
		for(var o = 0, ll = $font.colors; o < ll; o++) { //convert colors
			var open = new RegExp('<c' + (o + 1) + '>', 'g');
			var close = new RegExp('</c' + (o + 1) + '>', 'g');

			var color = OPTIONS.colors[o] || "white";

			$character = $character.replace(open, chalk.styles[color.toLowerCase()].open);
			$character = $character.replace(close, chalk.styles[color.toLowerCase()].close);
		}
	}

	return $character;

}


function equalWidth($character) {
	var charWidth = 0;

	for(var w = $character.length - 1; w >= 0; w--) {
		$char = $character[w].replace(/(<([^>]+)>)/ig, '');

		if( $char.length > charWidth ) {
			charWidth = $char.length;
		}
	};

	return charWidth;
}


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Main logic
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
function cfonts($SETTINGS) {

	//options
	var OPTIONS = {
		font: $SETTINGS.font || 'block', //define the font face
		colors: $SETTINGS.colors || [], //define all colors
		background: $SETTINGS.background || 'Black', //define the background color
		letterSpacing: $SETTINGS.letterSpacing === undefined ? 1 : $SETTINGS.letterSpacing, //define letter spacing
		space: $SETTINGS.space === undefined ? true : $SETTINGS.space, //define if the output text should have empty lines on top and on the bottom
		maxLength: $SETTINGS.maxLength || 10 //define how many character can be on one line
	};

	var $input = $SETTINGS.text; //text to be converted
	var FONTS = '*console*block*simple*3d*simple3d*'; //all supported font files
	var $font = {}; //the font object
	OPTIONS.background = changeCase.upperCaseFirst(OPTIONS.background); //background color case convertion


	if( FONTS.indexOf(OPTIONS.font) && OPTIONS.font !== 'console') { //getting font file

		var fontFile = __dirname + '/fonts/' + OPTIONS.font + '.json';
		$font = JSON.parse( fs.readFileSync(fontFile, 'utf8') );

	}
	else if(OPTIONS.font === 'console') { //writing in console text

		var $write = $input.replace('|', "\n");
		$font['colors'] = 0;

	}
	else { //throw error if the font does not exist
		console.log("\n" + '	Please use a font from the support stack: ' +
			chalk.styles.green.open + '[ console | block | simple | 3d | simple3d ]' + chalk.styles.green.close + "\n");
		process.exit(1);
	}


	if(OPTIONS.font !== 'console') { //only render a font if it isn't the console font

		var $output = [];
		for(var i = 0, length = $font.lines; i < length; i++) { //create first lines with buffer
			$output[i] = $font.buffer[i];
		}

		var $charLength = 0; //use for max character per line
		var $line = 0; //start with line 0
		var $letterSpacing = "";

		for(var i = 0, length = OPTIONS.letterSpacing; i < length; i++) { //letter spacing
			$letterSpacing += colorize($font, OPTIONS, $font.letterspace[i]);
		}


		for(var i = 0, length = $input.length; i < length; i++) { //iterate through the message

			var $char = $input.charAt(i).toUpperCase(); //only upper case is supported at this time

			if($charLength >= OPTIONS.maxLength || $char === "|") { //jump to next line after OPTIONS.maxLength characters or when line break is found
				$charLength = 0;
				$line += $font.lines;

				for(var l = $line, lll = $line + $font.lines; l < lll; l++) { //create new lines with buffer
					$output[l] = $font.buffer[ (l - $line) ];
				}
			}

			if( $font.chars[ $char ] !== undefined) { //make sure this character exists in the font
				$charLength++; //counting all printed characters

				var charWidth = equalWidth( $font.chars[ $char ] );

				for(var c = 0, l = $font.lines; c < l; c++) { //iterate over font face lines

					var $character = $font.chars[ $char ][c];

					if( $character.length < charWidth ) {
						for(var w = (charWidth - $character.length) - 1; w >= 0; w--) {
							$character += ' ';
						};
					}

					$character = colorize($font, OPTIONS, $character);

					$output[ ($line + c) ] += $character + $letterSpacing; //save output per character
				}
			}

		}


		var $write = $output.join("\n"); //convert to one line
	}


	if($font.colors <= 1) { //add text color if only one
		var color = OPTIONS.colors[0] || "white";

		$write = chalk.styles[color.toLowerCase()].open + $write + chalk.styles[color.toLowerCase()].close;
	}


	if(OPTIONS.space) { //add space
		$write = "\n\n" + $write + "\n\n";
	}


	console.log( chalk['bg' + OPTIONS.background]( $write ) ); //write out

}


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Module export
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
module.exports = cfonts;