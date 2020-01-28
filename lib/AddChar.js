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
 * AddChar
 *   Add a new character to the output array
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;

var _require2 = require('./Colorize.js'),
    Colorize = _require2.Colorize;
/**
 * Add a new character to the output array
 *
 * @param  {string}  CHAR       - The character to be added
 * @param  {array}   output     - The output array the line shall be appended to
 * @param  {integer} fontLines  - The number of lines this font has per character
 * @param  {object}  fontChars  - An object with all character arrays
 * @param  {integer} fontColors - The amount of colors allowed for this font
 * @param  {object}  colors     - Our options
 *
 * @return {array}              - The output array with new line
 */


var AddChar = function AddChar(CHAR, output, fontLines, fontChars, fontColors, colors) {
  Debugging.report("Running AddChar with \"".concat(CHAR, "\""), 1);
  var lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

  for (var i = lines; i < output.length; i++) {
    // iterate over last line
    var index = i - lines;
    output[i] += Colorize(fontChars[CHAR][index], fontColors, colors);
  }

  return output;
};

module.exports = exports = {
  AddChar: AddChar
};