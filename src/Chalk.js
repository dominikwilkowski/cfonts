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


// We pass on the FORCE_COLOR env var to chalk so we can force it in ci
const Chalk = new chalkOriginal.Instance({
	...(
		process.env.FORCE_COLOR
			? { level: parseInt( process.env.FORCE_COLOR ) }
			: null
	)
});


module.exports = exports = {
	Chalk,
};
