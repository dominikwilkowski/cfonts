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

var _require = require('./Chalk.js'),
    Chalk = _require.Chalk;
/**
 * DEBUG object for tracking debug mode and level
 *
 * @type {Object}
 */


var DEBUG = {
  store: {
    enabled: false,
    level: 2
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
  }

};
/**
 * Debugging prettiness
 *
 * @type {object}
 */

var Debugging = {
  /**
   * Return a headline preferably at the beginning of your app
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  headline: function headline(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG.enabled;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUG.level;

    if (debug && level >= debuglevel) {
      console.log(Chalk.bgWhite("\n".concat(Chalk.bold(" \u2611  "), " ").concat(text)));
    }
  },

  /**
   * Return a message to report starting a process
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  report: function report(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG.enabled;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUG.level;

    if (debug && level >= debuglevel) {
      console.log(Chalk.bgWhite("\n".concat(Chalk.bold.green(" \u2611  "), " ").concat(Chalk.black("".concat(text, " ")))));
    }
  },

  /**
   * Return a message to report an error
   *
   * @param  {string}  text  - The sting you want to log
   * @param  {integer} level - The debug level. Show equal and greater levels. Default: 99
   */
  error: function error(text) {
    var level = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : 99;
    var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG.enabled;
    var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUG.level;

    if (debug && level >= debuglevel) {
      console.error(Chalk.bgWhite("\n".concat(Chalk.red(" \u2612  "), " ").concat(Chalk.black("".concat(text, " ")))));
    }
  }
};
module.exports = exports = {
  DEBUG: DEBUG,
  Debugging: Debugging
};