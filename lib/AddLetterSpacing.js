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
 * AddLetterSpacing
 *   Add letter spacing for the next character
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;

var _require2 = require('./Colorize.js'),
    Colorize = _require2.Colorize;
/**
 * Add letter spacing for the next character
 *
 * @param  {array}   output          - The output array the line shall be appended to
 * @param  {integer} fontLines       - The number of lines this font has per character
 * @param  {array}   fontLetterspace - A space between the letters
 * @param  {integer} fontColors      - The amount of colors allowed for this font
 * @param  {array}   colors          - The user defined colors
 * @param  {integer} letterSpacing   - The user defined letter spacing
 *
 * @return {array}                   - The output array with space
 */


var AddLetterSpacing = function AddLetterSpacing(output, fontLines, fontLetterspace, fontColors, colors, letterSpacing) {
  Debugging.report("Running AddLetterSpacing", 1);
  var lines = output.length - fontLines; // last line is fontLines tall and is located at the bottom of the output array

  for (var i = lines; i < output.length; i++) {
    // iterate over last line
    var index = i - lines;
    var space = Colorize(fontLetterspace[index], fontColors, colors);

    if (space.length === 0 && letterSpacing > 0) {
      Debugging.report("AddLetterSpacing: Adding space to letter spacing", 1);
      space = ' ';
    }

    output[i] += space.repeat(letterSpacing);
  }

  return output;
};

module.exports = exports = {
  AddLetterSpacing: AddLetterSpacing
};