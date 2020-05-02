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
 * Chalk
 *   We pass on the FORCE_COLOR env var to chalk so we can force it in ci
 *
 **************************************************************************************************************************************************************/

'use strict';

const chalkOriginal = require(`chalk`);

// all possible level of chalk: https://github.com/chalk/chalk#chalklevel
const level = {
	'0': 0, // All colors disabled
	'1': 1, // Basic 16 colors support
	'2': 2, // ANSI 256 colors support
	'3': 3, // Truecolor 16 million colors support
}

// We pass on the FORCE_COLOR env var to chalk so we can force it in ci
const Chalk = new chalkOriginal.Instance({
	...(
		process.env.FORCE_COLOR
			? { level: level[ process.env.FORCE_COLOR ] }
			: null
	)
});


module.exports = exports = {
	Chalk,
};
