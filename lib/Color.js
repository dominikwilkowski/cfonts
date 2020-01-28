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
 * Color
 *   Abstraction for coloring hex-, keyword- and background-colors
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./UpperCaseFirst.js'),
    UpperCaseFirst = _require.UpperCaseFirst;

var _require2 = require('./Debugging.js'),
    Debugging = _require2.Debugging;

var _require3 = require('./constants.js'),
    HEXTEST = _require3.HEXTEST;

var _require4 = require('./Chalk.js'),
    Chalk = _require4.Chalk;
/**
 * Abstraction for coloring hex-, keyword- and background-colors
 *
 * @param  {string}  color    - The color to be used
 * @param  {boolean} bg       - Whether this is a background or not
 *
 * @typedef  {object} ReturnObject
 *   @property {string} open  - The open ansi code
 *   @property {string} close - The close ansi code
 *
 * @return {ReturnObject}     - An object with open and close ansi codes
 */


var Color = function Color(color) {
  var bg = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : false;

  // bail early if we use system color
  if (color === 'system' || process.env.FORCE_COLOR == 0) {
    return {
      open: '',
      close: ''
    };
  } // bail if this is a chalk defined color


  if (color.includes('Bright')) {
    if (bg) {
      color = "bg".concat(UpperCaseFirst(color));
    }

    return {
      open: Chalk[color]._styler.open,
      close: Chalk[color]._styler.close
    };
  }

  var kind = HEXTEST.test(color) ? 'hex' : "".concat(bg ? 'bgK' : 'k', "eyword");
  var styles;

  try {
    styles = Chalk[kind](color)._styler;
  } catch (error) {
    Debugging.error("The color ".concat(Chalk.yellow(color), " could not be found. Sorry about this."));
    return {
      open: '',
      close: ''
    };
  }

  return {
    open: styles.open,
    close: styles.close
  };
};

module.exports = exports = {
  Color: Color
};