/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * AddShortcuts
 *   Flatten the shortcuts in our cli options object
 *
 **************************************************************************************************************************************************************/

'use strict';

/**
 * Flatten the shortcuts in our cli options object
 *
 * @param  {object} options - An object objects with a short key
 *
 * @return {object}         - All short keys flattened into first level
 */
const AddShortcuts = (options) => {
	const flatOptions = Object.assign({}, options);

	Object.keys(flatOptions).forEach((option) => {
		flatOptions[option]._name = option;
		flatOptions[flatOptions[option].short] = flatOptions[option];
		if (flatOptions[option].fallback_shortcut) {
			flatOptions[flatOptions[option].fallback_shortcut] = flatOptions[option];
		}
	});

	return flatOptions;
};

module.exports = exports = {
	AddShortcuts,
};
