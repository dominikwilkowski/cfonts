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
 * Say
 *   Print to console
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Debugging, DEBUG } = require('./Debugging.js');
const { Render } = require('./Render.js');
const { Chalk } = require('./Chalk.js');
const { Size } = require('./Size.js');


/**
 * Print to console
 *
 * @param same as render method
 */
const Say = ( INPUT, SETTINGS = {}, debug = DEBUG.enabled, debuglevel = DEBUG.level, size = Size ) => {
	Debugging.report(`Running say`, 1);

	DEBUG.enabled = debug;
	DEBUG.level = debuglevel;

	const write = Render( INPUT, SETTINGS, debug, debuglevel, size );

	console.log( write.string ); // write out
};


module.exports = exports = {
	Say,
};
