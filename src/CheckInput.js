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
 * CheckInput
 *   Check input for human errors
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging } = require('./Debugging.js');
const { Chalk } = require('./Chalk.js');
const {
	COLORS,
	BGCOLORS,
	ALIGNMENT,
	FONTFACES,
	HEXTEST,
} = require('./constants.js');


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
	userGradient,
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
			&& !HEXTEST.test( color )
		) {
			result = {
				message: `"${ Chalk.red( color ) }" is not a valid font color option.\n` +
					`Please use a color from the supported stack or any valid hex color:\n${ Chalk.green(`${
						Object.keys( colors ).map( color => colors[ color ] ).join(', ')
					}, candy, "#3456ff", "#f80", etc...`) }`,
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

	// CHECKING GRADIENT
	if( userGradient ) {
		const gradientColors = userGradient.split(',');

		if( gradientColors.length !== 2 ) {
			return {
				message: `"${ Chalk.red( userGradient ) }" is not a valid gradient option.\n` +
					`Please pass in at least two colors.`,
				pass: false,
			};
		}

		// check validity of colors
		gradientColors.forEach( color => {
			if(
				Object.keys( colors ).indexOf( color.toLowerCase() ) === -1
				&& !HEXTEST.test( color )
			) {
				result = {
					message: `"${ Chalk.red( color ) }" is not a valid gradient color option.\n` +
						`Please use a color from the supported stack or any valid hex color:\n${ Chalk.green(`${
							Object.keys( colors ).map( color => colors[ color ] ).join(', ')
						}, "#3456ff", "#f80", etc...`) }`,
					pass: false,
				};
			}
		});
	}

	return result;
};


module.exports = exports = {
	CheckInput,
};
