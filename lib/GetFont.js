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
 * GetFont
 *   Get a selected JSON font-file object
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;
/**
 * Get a selected JSON font-file object
 *
 * @param  {string} font - The name of the font to be returned
 *
 * @return {object}      - The font object of that file
 */


var GetFont = function GetFont(font) {
  Debugging.report("Running GetFont", 1); // try loading the font file

  try {
    var FONTFACE = require("../fonts/".concat(font, ".json")); // read font file


    Debugging.report("GetFont: Fontface path selected: \"".concat(font, ".json\""), 2);
    return FONTFACE;
  } catch (error) {
    Debugging.error("Font file for \"".concat(font, "\" errored out: ").concat(error), 2);
    return false;
  }
};

module.exports = exports = {
  GetFont: GetFont
};