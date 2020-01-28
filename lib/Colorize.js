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
 * Colorize
 *   Replace placeholders with color information
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging;

var _require2 = require('./Color.js'),
    Color = _require2.Color;
/**
 * Replace placeholders with color information
 *
 * @param  {string}  character    - The string to be converted
 * @param  {integer} fontColors   - The number of allowed colors for this font
 * @param  {array}   optionColors - An array of user defined colors
 *
 * @return {string}               - The character with color ansi escape sequences for CLI
 */


var Colorize = function Colorize(character, fontColors, optionColors) {
  Debugging.report("Running Colorize", 1);
  var candyColors = [// allowed candy colors
  'red', 'green', 'yellow', 'magenta', 'cyan', 'redBright', 'greenBright', 'yellowBright', 'blueBright', 'magentaBright', 'cyanBright'];

  if (character !== undefined) {
    if (fontColors > 1) {
      // we have to replace all color placeholder with ansi escape sequences
      for (var i = 0; i < fontColors; i++) {
        var color = optionColors[i] === 'candy' ? candyColors[Math.floor(Math.random() * candyColors.length)] : optionColors[i] || 'system';

        var _Color = Color(color),
            openNew = _Color.open,
            closeNew = _Color.close;

        var open = new RegExp("<c".concat(i + 1, ">"), 'g');
        var close = new RegExp("</c".concat(i + 1, ">"), 'g');
        character = character.replace(open, openNew);
        character = character.replace(close, closeNew);
      }
    } // if only one color is allowed there won't be any color placeholders in the characters


    if (fontColors === 1) {
      var _color = optionColors[0] === 'candy' ? candyColors[Math.floor(Math.random() * candyColors.length)] : optionColors[0] || 'system';

      var _Color2 = Color(_color),
          _openNew = _Color2.open,
          _closeNew = _Color2.close;

      character = _openNew + character + _closeNew;
    }
  }

  return character;
};

module.exports = exports = {
  Colorize: Colorize
};