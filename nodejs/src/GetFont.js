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
 * GetFont
 *   Get a selected JSON font-file object
 *
 **************************************************************************************************************************************************************/

'use strict';

const path = require('path');

const { Debugging } = require('./Debugging.js');

/**
 * Get a selected JSON font-file object
 *
 * @param  {string} font - The name of the font to be returned
 *
 * @return {object}      - The font object of that file
 */
const GetFont = (font) => {
	Debugging.report(`Running GetFont`, 1);

	// try loading the font file
	try {
		let FONTFACE = require(path.normalize(`../fonts/${font}.json`)); // read font file

		Debugging.report(`GetFont: Fontface path selected: "${font}.json"`, 2);

		return FONTFACE;
	} catch (error) {
		Debugging.error(`Font file for "${font}" errored out: ${error}`, 2);

		return false;
	}
};

module.exports = exports = {
	GetFont,
};
