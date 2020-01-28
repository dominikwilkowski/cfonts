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
 * AddLine
 *   Add a new line to the output array
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;
/**
 * Add a new line to the output array
 *
 * @param  {array}   output      - The output array the line shall be appended to
 * @param  {integer} fontLines   - The number of lines this font has per character
 * @param  {array}   FontBuffer  - An array of the space we add at the beginning of each line
 * @param  {integer} lineHeight  - The user defined line height
 *
 * @return {array}               - The output array with new line
 */


var AddLine = function AddLine(output, fontLines, FontBuffer, lineHeight) {
  Debugging.report("Running AddLine", 1);

  if (output.length === 0) {
    lineHeight = 0;
  }

  var lines = fontLines + output.length + lineHeight;
  var length = output.length;

  for (var i = length; i < lines; i++) {
    var index = i - length;

    if (index > lineHeight) {
      output[i] = FontBuffer[index - lineHeight];
    } else {
      output[i] = '';
    }
  }

  return output;
};

module.exports = exports = {
  AddLine: AddLine
};