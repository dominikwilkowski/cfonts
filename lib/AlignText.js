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
 * AlignText
 *   Calculate the spaces to be added to the left of each line to align them either center or right
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;

var _require2 = require('./Size.js'),
    Size = _require2.Size;
/**
 * Calculate the spaces to be added to the left of each line to align them either center or right
 *
 * @param  {array}   output         - The output array the line shall be appended to
 * @param  {integer} lineLength     - The current line length
 * @param  {integer} characterLines - The amount of line breaks in one character
 * @param  {string}  align          - The alignment of the text, only `center` and `right` will do anything
 * @param  {object}  size           - The size of the terminal as an object, default: Size
 * @param  {integer} size.width     - The width of the terminal
 * @param  {integer} size.height    - The height of the terminal
 *
 * @return {array}                  - The output array with space added on the left for alignment
 */


var AlignText = function AlignText(output, lineLength, characterLines, align) {
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running AlignText", 1);
  var space = 0;

  if (align === 'center') {
    // calculate the size for center alignment
    space = Math.floor((size.width - lineLength) / 2);
    Debugging.report("AlignText: Center lineLength: ".concat(lineLength, ", size.width: ").concat(size.width, ", space: ").concat(space), 2);
  }

  if (align === 'right') {
    // calculate the size for right alignment
    space = size.width - lineLength;
    Debugging.report("AlignText: Right lineLength: ".concat(lineLength, ", size.width: ").concat(size.width, ", space: ").concat(space), 2);
  }

  if (space > 0) {
    // only add if there is something to add
    var lines = output.length - characterLines; // last line is characterLines tall and is located at the bottom of the output array

    space = ' '.repeat(space);

    for (var i = lines; i < output.length; i++) {
      // iterate over last line (which can be several line breaks long)
      output[i] = space + output[i];
    }
  }

  return output;
};

module.exports = exports = {
  AlignText: AlignText
};