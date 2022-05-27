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
 * Say
 *   Print to console
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging, DEBUG } = require('./Debugging.js');
const { Render } = require('./Render.js');
const { Size } = require('./Size.js');

/**
 * Print to console
 *
 * @param  {string}  INPUT       - The string you want to write out
 * @param  {object}  SETTINGS    - Settings object
 * @param  {boolean} debug       - A flag to enable debug mode
 * @param  {number}  debuglevel  - The debug level we want to show
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {number}  size.width  - The width of the terminal
 * @param  {number}  size.height - The height of the terminal
 */
const Say = (INPUT, SETTINGS = {}, debug = DEBUG.enabled, debuglevel = DEBUG.level, size = Size) => {
	Debugging.report(`Running say`, 1);

	DEBUG.enabled = debug;
	DEBUG.level = debuglevel;

	const write = Render(INPUT, SETTINGS, debug, debuglevel, size);

	if (write) {
		console.log(write.string); // write out
	}
};

module.exports = exports = {
	Say,
};
