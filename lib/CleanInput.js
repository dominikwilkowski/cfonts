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
 * CleanInput
 *   Filter only allowed character
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./constants.js'),
    CHARS = _require.CHARS;
/**
 * Filter only allowed character
 *
 * @param  {string} INPUT - The input text to be filtered
 * @param  {array}  chars - An array of all allowed characters
 *
 * @return {string}       - The filtered input text
 */


var CleanInput = function CleanInput(INPUT) {
  var chars = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : CHARS;

  if (typeof INPUT === 'string') {
    var clean = INPUT.replace(/(?:\r\n|\r|\n)/g, '|').split('').filter(function (_char) {
      return chars.includes(_char.toUpperCase());
    }).join('');
    return clean;
  } else {
    return '';
  }
};

module.exports = exports = {
  CleanInput: CleanInput
};