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
 * UpperCaseFirst
 *   Upper case the first character of an input string
 *
 **************************************************************************************************************************************************************/

'use strict';

/**
 * Upper case the first character of an input string.
 *
 * @author https://github.com/blakeembrey/change-case/tree/master/packages/upper-case-first
 *
 * @param  {string} input - A string to be converted
 *
 * @return {string}       - A string with the first letter in upper case
 */
const UpperCaseFirst = (input) => (typeof input === 'string' ? input.charAt(0).toUpperCase() + input.substr(1) : input);

module.exports = exports = {
	UpperCaseFirst,
};
