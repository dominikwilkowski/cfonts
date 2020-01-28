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
 * Say
 *   Print to console
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./Debugging.js'),
    Debugging = _require.Debugging,
    DEBUG = _require.DEBUG;

var _require2 = require('./Render.js'),
    Render = _require2.Render;

var _require3 = require('./Chalk.js'),
    Chalk = _require3.Chalk;

var _require4 = require('./Size.js'),
    Size = _require4.Size;
/**
 * Print to console
 *
 * @param same as render method
 */


var Say = function Say(INPUT) {
  var SETTINGS = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : {};
  var debug = arguments.length > 2 && arguments[2] !== undefined ? arguments[2] : DEBUG.enabled;
  var debuglevel = arguments.length > 3 && arguments[3] !== undefined ? arguments[3] : DEBUG.level;
  var size = arguments.length > 4 && arguments[4] !== undefined ? arguments[4] : Size;
  Debugging.report("Running say", 1);
  DEBUG.enabled = debug;
  DEBUG.level = debuglevel;
  var write = Render(INPUT, SETTINGS, debug, debuglevel, size);
  console.log(write.string); // write out
};

module.exports = exports = {
  Say: Say
};