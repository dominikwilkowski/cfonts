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
 * GetLongestLine
 *   Return the longest line of an Array
 *
 **************************************************************************************************************************************************************/

'use strict';

/**
 * Return the longest line of an Array
 *
 * @param  {array}  lines  - An array of strings
 *
 * @return {string}        - The longest string from within the array
 */
const GetLongestLine = (lines) =>
	lines.reduce((longestLine, line) => (line.length > longestLine.length && line.length !== 0 ? line : longestLine), '');

module.exports = exports = {
	GetLongestLine,
};
