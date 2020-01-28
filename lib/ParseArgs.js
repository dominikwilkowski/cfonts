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
 * ParseArgs
 *   Parse cli arguments into a nice object
 *
 **************************************************************************************************************************************************************/
'use strict';

var _require = require('./AddShortcuts.js'),
    AddShortcuts = _require.AddShortcuts;

var _require2 = require('./constants.js'),
    CLIOPTIONS = _require2.CLIOPTIONS;

var _require3 = require('./Debugging.js'),
    Debugging = _require3.Debugging;
/**
 * Parse cli arguments into a nice object
 *
 * @param  {array} inputOptions - All possible options registered for this app
 * @param  {array} inputArgs    - The arguments given to us in our cli, default: process.argv
 *
 * @return {object}             - An object of all options with at least their default values
 */


var ParseArgs = function ParseArgs() {
  var inputOptions = arguments.length > 0 && arguments[0] !== undefined ? arguments[0] : CLIOPTIONS;
  var inputArgs = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : process.argv;
  var parsedArgs = {
    text: inputArgs[2]
  }; // create defaults

  Object.keys(inputOptions).forEach(function (option) {
    var name = option.replace('--', '');
    parsedArgs[name] = inputOptions[option]["default"];
  });

  if (inputArgs[2] === '--help' || inputArgs[2] === '-h') {
    parsedArgs.help = true;
  }

  if (inputArgs[2] === '--version' || inputArgs[2] === '-v') {
    parsedArgs.version = true;
  }

  var args = inputArgs.splice(3); // the first two are node specific, the third is our text

  var options = AddShortcuts(inputOptions);

  for (var index = 0; args.length > index; index++) {
    var option = options[args[index]];

    if (option) {
      var name = option._name.replace('--', '');

      if (option.options !== undefined) {
        index++;
        var value = args[index];
        parsedArgs[name] = value;
      } else {
        parsedArgs[name] = true;
      }
    } else {
      Debugging.report("The cli argument ".concat(args[index], " was not found and ignored"), 2);
    }
  }

  ;
  return parsedArgs;
};

module.exports = exports = {
  ParseArgs: ParseArgs
};