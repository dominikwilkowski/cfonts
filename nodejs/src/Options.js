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
 * Options
 *   Merge user settings with default options
 *
 **************************************************************************************************************************************************************/

'use strict';

const { COLORS, BGCOLORS, FONTFACES } = require('./constants.js');

/**
 * The options store with getter and setter methods
 *
 * @type {Object}
 */
const Options = {
	store: {},

	reset() {
		const defaults = {
			font: 'block',
			align: 'left',
			colors: [],
			background: 'transparent',
			letterSpacing: 1,
			lineHeight: 1,
			spaceless: false,
			maxLength: 0,
			gradient: false,
			independentGradient: false,
			transitionGradient: false,
			rawMode: false,
			env: 'node',
		};

		this.store = { ...defaults }; // cloning
	},

	/**
	 * Get the current options
	 *
	 * @return {object} - Our options as hey are stored in our object
	 */
	get get() {
		return this.store;
	},

	/**
	 * Merge settings into our options object
	 *
	 * @param  {object}                  options                     - The settings object
	 * @param  {string}                  options.font                - Font face, Default 'block'
	 * @param  {string}                  options.align               - Text alignment, Default: 'left'
	 * @param  {array}                   options.colors              - Colors for font, Default: []
	 * @param  {string}                  options.background          - Color string for background, Default 'Black'
	 * @param  {string}                  options.backgroundColor     - Alias for background
	 * @param  {number}                  options.letterSpacing       - Space between letters, Default: set by selected font face
	 * @param  {number}                  options.lineHeight          - Space between lines, Default: 1
	 * @param  {boolean}                 options.spaceless           - Don't output space before and after output, Default: false
	 * @param  {number}                  options.maxLength           - Maximum amount of characters per line, Default width of console window
	 * @param  {(string|array|boolean)}  options.gradient            - Gradient color pair, Default: false
	 * @param  {boolean}                 options.independentGradient - A switch to calculate gradient per line or not
	 * @param  {boolean}                 options.transitionGradient  - A switch for transition gradients
	 * @param  {string}                  options.env                 - The environment we run cfonts in
	 * @param  {object}                  options.allowedColors       - All allowed font colors
	 * @param  {object}                  options.allowedBG           - All allowed background colors
	 * @param  {object}                  options.allowedFont         - All allowed fontfaces
	 * @param  {boolean}                 options.rawMode             - A switch for raw mode in terminals
	 */
	set set({
		font = '',
		align,
		colors,
		background,
		backgroundColor,
		letterSpacing,
		lineHeight,
		spaceless,
		maxLength,
		gradient,
		independentGradient,
		transitionGradient,
		env,
		allowedColors = COLORS,
		allowedBG = BGCOLORS,
		allowedFont = FONTFACES,
		rawMode,
	}) {
		this.store.font = font !== '' ? allowedFont[font.toLowerCase()] || font : this.store.font;

		this.store.align = align !== undefined ? align.toLowerCase() : this.store.align;

		this.store.colors = Array.isArray(colors)
			? colors.map((color) => allowedColors[color.toLowerCase()] || color)
			: this.store.colors;

		const bg = backgroundColor || background;
		this.store.background = bg !== undefined ? allowedBG[bg.toLowerCase()] || bg : this.store.background;

		this.store.letterSpacing =
			letterSpacing !== undefined
				? parseInt(letterSpacing.toString())
				: font.toLowerCase() === 'console'
				? 0
				: this.store.letterSpacing;

		this.store.lineHeight =
			lineHeight !== undefined
				? parseInt(lineHeight.toString())
				: font.toLowerCase() === 'console'
				? 0
				: this.store.lineHeight;

		this.store.spaceless = typeof spaceless === 'boolean' ? spaceless : this.store.spaceless;

		this.store.maxLength = maxLength !== undefined ? maxLength : this.store.maxLength;

		this.store.gradient =
			gradient !== undefined && typeof gradient !== 'boolean'
				? Array.isArray(gradient)
					? gradient
					: gradient.split(',')
				: gradient === false
				? false
				: this.store.gradient;

		this.store.independentGradient =
			independentGradient !== undefined ? independentGradient : this.store.independentGradient;

		this.store.transitionGradient =
			transitionGradient !== undefined ? transitionGradient : this.store.transitionGradient;

		this.store.env = env !== undefined ? env : this.store.env;

		this.store.rawMode = rawMode !== undefined ? rawMode : this.store.rawMode;
	},
};

module.exports = exports = {
	Options,
};
