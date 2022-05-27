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
 * DisplayHelp
 *   Display the help generated from our CLIOPTIONS
 *
 **************************************************************************************************************************************************************/

'use strict';

const { CLIOPTIONS } = require('./constants.js');
const { Render } = require('./Render.js');
const { Color } = require('./Color.js');

/**
 * Display the help generated from our CLIOPTIONS
 */
const DisplayHelp = () => {
	const { string: headline } = Render('cfonts', {
		align: 'left',
		gradient: ['red', 'green'],
	});

	console.log(
		` ${headline}` +
			`This is a tool for sexy fonts in the console. Give your cli some love.\n\n` +
			`Usage: cfonts "<value>" [option1] <input1> [option2] <input1>,<input2> [option3]\n` +
			`Example: \u001b[1m$ cfonts "sexy font" -f chrome -a center -c red,green,gray\u001b[22m\n\n` +
			`Options:\n`
	);

	const { open, close } = Color('green');

	Object.keys(CLIOPTIONS).forEach((option) => {
		console.log(`\u001b[1m${option}, ${CLIOPTIONS[option].short}\u001b[22m`);
		console.log(CLIOPTIONS[option].description);
		console.log(
			`\u001b[1m$\u001b[22m cfonts ${CLIOPTIONS[option].example
				.replace(/\[green-open\]/g, open)
				.replace(/\[green-close\]/g, close)}\n`
		);
	});
};

module.exports = exports = {
	DisplayHelp,
};
