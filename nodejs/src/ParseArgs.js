/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPLv3
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * ParseArgs
 *   Parse cli arguments into a nice object
 *
 **************************************************************************************************************************************************************/

'use strict';

const { AddShortcuts } = require('./AddShortcuts.js');
const { CLIOPTIONS } = require('./constants.js');
const { Debugging } = require('./Debugging.js');

/**
 * Parse cli arguments into a nice object
 *
 * @param  {object} inputOptions - All possible options registered for this app
 * @param  {array}  inputArgs    - The arguments given to us in our cli, default: process.argv
 *
 * @return {object}              - An object of all options with at least their default values
 */
const ParseArgs = (inputOptions = CLIOPTIONS, inputArgs = process.argv) => {
	const parsedArgs = {
		text: inputArgs[2],
	};

	// create defaults
	Object.keys(inputOptions).forEach((option) => {
		const name = option.replace('--', '');

		parsedArgs[name] = inputOptions[option].default;
	});

	if (inputArgs[2] === '--help' || inputArgs[2] === '-h') {
		parsedArgs.help = true;
	}

	if (inputArgs[2] === '--version' || inputArgs[2] === '-v') {
		parsedArgs.version = true;
	}

	const args = inputArgs.splice(3); // the first two are node specific, the third is our text

	const options = AddShortcuts(inputOptions);

	for (let index = 0; args.length > index; index++) {
		const option = options[args[index]];

		if (option) {
			const name = option._name.replace('--', '');

			if (option.options !== undefined) {
				index++;
				const value = args[index];

				parsedArgs[name] = value;
			} else {
				parsedArgs[name] = true;
			}
		} else {
			Debugging.report(`The cli argument ${args[index]} was not found and ignored`, 2);
		}
	}

	return parsedArgs;
};

module.exports = exports = {
	ParseArgs,
};
