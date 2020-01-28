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
 * Log
 *   Logging prettiness
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Chalk.js'),
    Chalk = _require.Chalk;
/**
 * Logging prettiness
 *
 * @type {object}
 */


var Log = {
  /**
   * Print error message to console.
   *
   * @param  {string} text - The sting you want to log
   */
  error: function error(text) {
    text = text.replace(/(?:\r\n|\r|\n)/g, '\n       '); // indent each line

    console.error("\n ".concat(Chalk.bold.red('Ouch:'), " ").concat(text, "\n"));
  }
};
module.exports = exports = {
  Log: Log
};