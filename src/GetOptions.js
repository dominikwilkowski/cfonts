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
 * @param  {SETTINGS}   SETTINGS                      - Some or all of the allowed settings
 * @param  {object}     allowedColors                 - All allowed font colors
 * @param  {object}     allowedBG                     - All allowed background colors
 * @param  {object}     allowedFont                   - All allowed fontfaces
 *
 * @typedef  {object} SETTINGS
 *   @property  {string}          font                - Font face, Default 'block'
 *   @property  {string}          align               - Text alignment, Default: 'left'
 *   @property  {array}           colors              - Colors for font, Default: []
 *   @property  {string}          background          - Chalk color string for background, Default 'Black'
 *   @property  {string}          backgroundColor     - Alias for background
 *   @property  {number}          letterSpacing       - Space between letters, Default: set by selected font face
 *   @property  {number}          lineHeight          - Space between lines, Default: 1
 *   @property  {boolean}         space               - Output space before and after output, Default: true
 *   @property  {number}          maxLength           - Maximum amount of characters per line, Default width of console window
 *   @property  {(string|array)}  gradient            - Gradient color pair, Default: false
 *   @property  {boolean}         independentGradient - A switch to calculate gradient per line or not
 *
 * @return   {object}                                 - Our merged options
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
	letterSpacing: typeof letterSpacing !== 'undefined' && letterSpacing > 0
		? parseInt( letterSpacing.toString() )
		: 1,
	lineHeight: lineHeight === undefined
		? 1
		: parseInt( lineHeight.toString() ),
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
