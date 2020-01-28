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
 * DisplayHelp
 *   Display the help generated from our CLIOPTIONS
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./constants.js'),
    CLIOPTIONS = _require.CLIOPTIONS;

var _require2 = require('./Render.js'),
    Render = _require2.Render;

var _require3 = require('./Chalk.js'),
    Chalk = _require3.Chalk;
/**
 * Display the help generated from our CLIOPTIONS
 */


var DisplayHelp = function DisplayHelp() {
  console.log(" ".concat(Render('cfonts', {
    align: 'left',
    colors: ['redBright', 'greenBright']
  }).string) + "This is a tool for sexy fonts in the console. Give your cli some love.\n\n" + "Usage: cfonts \"<value>\" [option1] <input1> [option2] <input1>,<input2> [option3]\n" + "Example: ".concat(Chalk.bold('$ cfonts "sexy font" -f chrome -a center -c red,green,gray'), "\n\n") + "Options:\n");
  var command = [];
  var largestSize = 0;
  Object.keys(CLIOPTIONS).forEach(function (option) {
    console.log(Chalk.bold("".concat(option, ", ").concat(CLIOPTIONS[option]["short"])));
    console.log(CLIOPTIONS[option].description);
    console.log("".concat(Chalk.bold('$'), " cfonts ").concat(CLIOPTIONS[option].example, "\n"));
  });
};

module.exports = exports = {
  DisplayHelp: DisplayHelp
};