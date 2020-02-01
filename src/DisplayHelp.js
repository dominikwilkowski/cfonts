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

const { CLIOPTIONS } = require('./constants.js');
const { Render } = require('./Render.js');
const { Chalk } = require('./Chalk.js');


/**
 * Display the help generated from our CLIOPTIONS
 */
const DisplayHelp = () => {
	const { string: headline } = Render('cfonts', {
		align: 'left',
		gradient: [ 'red','green' ],
	});

	console.log(
		` ${ headline }` +
		`This is a tool for sexy fonts in the console. Give your cli some love.\n\n` +
		`Usage: cfonts "<value>" [option1] <input1> [option2] <input1>,<input2> [option3]\n` +
		`Example: ${ Chalk.bold('$ cfonts "sexy font" -f chrome -a center -c red,green,gray') }\n\n` +
		`Options:\n`
	);

	Object.keys( CLIOPTIONS ).forEach( option => {
		console.log( Chalk.bold(`${ option }, ${ CLIOPTIONS[ option ].short }`) );
		console.log( CLIOPTIONS[ option ].description );
		console.log(`${ Chalk.bold('$') } cfonts ${ CLIOPTIONS[ option ].example }\n`);
	});
};


module.exports = exports = {
	DisplayHelp,
};
