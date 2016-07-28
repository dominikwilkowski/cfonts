/***************************************************************************************************************************************************************
 *
 * cfonts, Sexy fonts for the console. (CLI output)
 *
 * Testing the API side of things. Not automated yet :(
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE  GNU GPLv2
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 **************************************************************************************************************************************************************/

'use strict';


//--------------------------------------------------------------------------------------------------------------------------------------------------------------
// Dependencies
//--------------------------------------------------------------------------------------------------------------------------------------------------------------
const CFonts = require('../index');

let prettyFont = CFonts.render('H', {
	font: 'simple',
});

console.log(prettyFont.string);
console.log(prettyFont.array);
console.log(prettyFont.lines);
console.log(prettyFont.options);