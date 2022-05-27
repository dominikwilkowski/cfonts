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
 * DisplayVersion
 *   Display the version of this package
 *
 **************************************************************************************************************************************************************/

'use strict';

const { PACKAGE } = require('./constants.js');

/**
 * Display the version of this package
 */
const DisplayVersion = () => {
	console.log(PACKAGE.version);
};

module.exports = exports = {
	DisplayVersion,
};
