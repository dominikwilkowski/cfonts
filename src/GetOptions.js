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
 * GetOptions
 *   Merge user settings with default options
 *
 **************************************************************************************************************************************************************/

'use strict';

const {
	COLORS,
	BGCOLORS,
	FONTFACES,
} = require('./constants.js');


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
 *   @param  {integer} gradient      - Gradient color pair, Default: false
 *
 * @return {object}                  - Our merged options
 */
const GetOptions = (
	{ font, align, colors, background, backgroundColor, letterSpacing, lineHeight, space, maxLength, gradient, independentGradient },
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
	gradient: gradient
		? Array.isArray( gradient )
			? gradient
			: gradient.split(',')
		: false,
	independentGradient: independentGradient || false,
});


module.exports = exports = {
	GetOptions,
};
