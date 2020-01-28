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
 * DisplayVersion
 *   Display the version of this package
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./constants.js'),
    PACKAGE = _require.PACKAGE;
/**
 * Display the version of this package
 */


var DisplayVersion = function DisplayVersion() {
  console.log(PACKAGE.version);
};

module.exports = exports = {
  DisplayVersion: DisplayVersion
};