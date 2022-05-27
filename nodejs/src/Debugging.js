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
 * Debugging
 *   Debugging prettiness
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Color } = require('./Color.js');

/**
 * DEBUG object for tracking debug mode and level
 *
 * @type {Object}
 */
const DEBUG = {
	store: {
		enabled: false,
		level: 2,
	},

	set enabled(value) {
		this.store.enabled = value;
	},

	get enabled() {
		return this.store.enabled;
	},

	set level(value) {
		this.store.level = value;
	},

	get level() {
		return this.store.level;
	},
};

/**
 * Debugging prettiness
 *
 * @type {object}
 */
const Debugging = {
	/**
	 * Return a headline preferably at the beginning of your app
	 *
	 * @param  {string}  text       - The sting you want to log
	 * @param  {number}  level      - The debug level. Show equal and greater levels. Default: 99
	 * @param  {boolean} debug      - Global debug mode on/off
	 * @param  {number}  debuglevel - Global debug level
	 */
	headline: (text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level) => {
		if (debug && level >= debuglevel) {
			const { open, close } = Color('black', true);
			console.log(`${open}\n\u001b[1m \u2611  \u001b[22m ${text}${close}`);
		}
	},

	/**
	 * Return a message to report starting a process
	 *
	 * @param  {string}  text       - The sting you want to log
	 * @param  {number}  level      - The debug level. Show equal and greater levels. Default: 99
	 * @param  {boolean} debug      - Global debug mode on/off
	 * @param  {number}  debuglevel - Global debug level
	 */
	report: (text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level) => {
		if (debug && level >= debuglevel) {
			const { open: blackbg_open, close: blackbg_close } = Color('black', true);
			const { open: green_open, close: green_close } = Color('green');
			const { open: white_open, close: white_close } = Color('white');
			console.log(
				`${blackbg_open}\n\u001b[1m${green_open} \u2611  ${green_close}\u001b[22m ${white_open}${text}${white_close}${blackbg_close}`
			);
		}
	},

	/**
	 * Return a message to report an error
	 *
	 * @param  {string}  text       - The sting you want to log
	 * @param  {number}  level      - The debug level. Show equal and greater levels. Default: 99
	 * @param  {boolean} debug      - Global debug mode on/off
	 * @param  {number}  debuglevel - Global debug level
	 */
	error: (text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level) => {
		if (debug && level >= debuglevel) {
			const { open: blackbg_open, close: blackbg_close } = Color('black', true);
			const { open: red_open, close: red_close } = Color('red');
			const { open: white_open, close: white_close } = Color('white');
			console.error(
				`${blackbg_open}\n${red_open} \u2612  ${red_close} ${white_open}${text}${white_close}${blackbg_close}`
			);
		}
	},
};

module.exports = exports = {
	DEBUG,
	Debugging,
};
