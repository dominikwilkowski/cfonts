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
 * Size
 *   Abstraction for windows size
 *
 **************************************************************************************************************************************************************/

'use strict';

const WinSize = require('window-size');

/**
 * Abstraction for windows size
 *
 * @type {object}
 */
const Size = {
	width: WinSize ? (WinSize.width > 0 ? WinSize.width : 80) : 80,
	height: WinSize ? (WinSize.height > 0 ? WinSize.height : 24) : 24,
};

module.exports = exports = {
	Size,
};
