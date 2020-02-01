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
 * Debugging
 *   Debugging prettiness
 *
 **************************************************************************************************************************************************************/

'use strict';

const { Chalk } = require('./Chalk.js');


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

	set enabled( value ) {
		this.store.enabled = value;
	},

	get enabled() {
		return this.store.enabled;
	},

	set level( value ) {
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
	headline: ( text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level ) => {
		if( debug && level >= debuglevel ) {
			console.log(
				Chalk.bgWhite(`\n${ Chalk.bold(' \u2611  ') } ${ text }`)
			);
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
	report: ( text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level ) => {
		if( debug && level >= debuglevel ) {
			console.log(
				Chalk.bgWhite(`\n${ Chalk.bold.green(' \u2611  ') } ${ Chalk.black(`${ text } `) }`)
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
	error: ( text, level = 99, debug = DEBUG.enabled, debuglevel = DEBUG.level ) => {
		if( debug && level >= debuglevel ) {
			console.error(
				Chalk.bgWhite(`\n${ Chalk.red(' \u2612  ') } ${ Chalk.black(`${ text } `) }`)
			);
		}
	},
};


module.exports = exports = {
	DEBUG,
	Debugging,
};
