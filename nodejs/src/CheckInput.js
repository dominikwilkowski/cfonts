/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * CheckInput
 *   Check input for human errors
 *
 **************************************************************************************************************************************************************/

'use strict';

const { HEXTEST, Color } = require('./Color.js');
const { COLORS, BGCOLORS, GRADIENTCOLORS, GRADIENTS, ALIGNMENT, FONTFACES } = require('./constants.js');

/**
 * Check input for human errors
 *
 * @param  {string}  INPUT                  - The string you want to write out
 * @param  {string}  userFont               - The user specified font
 * @param  {array}   userColors             - The user specified colors
 * @param  {string}  userBackground         - The user specified background color
 * @param  {string}  userAlign              - The user specified alignment option
 * @param  {array}   userGradient           - The user specified gradient option
 * @param  {boolean} userTransitionGradient - The user specified gradient transition option
 * @param  {string}  userEnv                - The user specified environment
 * @param  {object}  fontfaces              - All allowed fontfaces
 * @param  {object}  colors                 - All allowed font colors
 * @param  {object}  bgcolors               - All allowed background colors
 * @param  {object}  gradientcolors         - All allowed gradient colors
 * @param  {object}  gradients              - All allowed gradients
 * @param  {array}   alignment              - All allowed alignments
 *
 * @typedef  {object} ReturnObject
 *   @property {boolean} pass               - Whether the input is valid
 *   @property {string}  message            - Possible error messages
 *
 * @return {ReturnObject}                   - An object with error messages and a pass key
 */
const CheckInput = (
	INPUT,
	userFont,
	userColors,
	userBackground,
	userAlign,
	userGradient,
	userTransitionGradient,
	userEnv,
	fontfaces = FONTFACES,
	colors = COLORS,
	bgcolors = BGCOLORS,
	gradientcolors = GRADIENTCOLORS,
	gradients = GRADIENTS,
	alignment = ALIGNMENT
) => {
	let result = {
		message: '',
		pass: true,
	};

	const { open: red_open, close: red_close } = Color('red');
	const { open: green_open, close: green_close } = Color('green');

	// checking input
	if (INPUT === undefined || INPUT === '') {
		return {
			message: 'Please provide text to convert',
			pass: false,
		};
	}

	// checking font
	if (Object.keys(fontfaces).indexOf(userFont.toLowerCase()) === -1) {
		return {
			message:
				`"${red_open}${userFont}${red_close}" is not a valid font option.\n` +
				`Please use a font from the supported stack:\n${green_open}${Object.keys(fontfaces)
					.map((font) => fontfaces[font])
					.join(', ')}${green_close}`,
			pass: false,
		};
	}

	// checking colors
	userColors.forEach((color) => {
		// check color usage
		if (Object.keys(colors).indexOf(color.toLowerCase()) === -1 && color !== 'candy' && !HEXTEST.test(color)) {
			result = {
				message:
					`"${red_open}${color}${red_close}" is not a valid font color option.\n` +
					`Please use a color from the supported stack or any valid hex color:\n` +
					`${green_open}${Object.keys(colors)
						.map((color) => colors[color])
						.join(', ')}, candy, "#3456ff", "#f80", etc...${green_close}`,
				pass: false,
			};
		}
	});

	// checking background colors
	if (Object.keys(bgcolors).indexOf(userBackground.toLowerCase()) === -1 && !HEXTEST.test(userBackground)) {
		return {
			message:
				`"${red_open}${userBackground}${red_close}" is not a valid background option.\n` +
				`Please use a color from the supported stack:\n` +
				`${green_open}${Object.keys(bgcolors)
					.map((bgcolor) => bgcolors[bgcolor])
					.join(', ')}, "#3456ff", "#f80", etc...${green_close}`,
			pass: false,
		};
	}

	// CHECKING ALIGNMENT
	if (alignment.indexOf(userAlign.toLowerCase()) === -1) {
		return {
			message:
				`"${red_open}${userAlign}${red_close}" is not a valid alignment option.\n` +
				`Please use an alignment option from the supported stack:\n${green_open}${alignment.join(' | ')}${green_close}`,
			pass: false,
		};
	}

	// CHECKING GRADIENT
	if (userGradient) {
		if (
			userGradient.length === 1 &&
			Object.keys(gradients).indexOf(userGradient[0].toLowerCase()) !== -1 &&
			userTransitionGradient
		) {
			return result;
		} else {
			if (userGradient.length < 2) {
				return {
					message:
						`"${red_open}${userGradient}${red_close}" is not a valid gradient option.\n` +
						`Please pass in${userTransitionGradient ? ' at least' : ''} two colors.`,
					pass: false,
				};
			}

			if (userGradient.length !== 2 && !userTransitionGradient) {
				return {
					message:
						`"${red_open}${userGradient}${red_close}" is not a valid gradient option.\n` + `Please pass in two colors.`,
					pass: false,
				};
			}

			// check validity of colors
			userGradient.forEach((color) => {
				if (Object.keys(gradientcolors).indexOf(color.toLowerCase()) === -1 && !HEXTEST.test(color)) {
					result = {
						message:
							`"${red_open}${color}${red_close}" is not a valid gradient color option.\n` +
							`Please use a color from the supported stack or any valid hex color:\n${green_open}${Object.keys(
								gradientcolors
							)
								.map((color) => colors[color])
								.join(', ')}, "#3456ff", "#f80", etc...${green_close}`,
						pass: false,
					};
				}
			});
		}
	}

	// CHECKING ENVIRONMENT
	if (userEnv !== 'node' && userEnv !== 'browser') {
		return {
			message:
				`"${red_open}${userEnv}${red_close}" is not a valid environment option.\n` +
				`Please use only the supported options:\n${green_open}node | browser${green_close}`,
			pass: false,
		};
	}

	return result;
};

module.exports = exports = {
	CheckInput,
};
