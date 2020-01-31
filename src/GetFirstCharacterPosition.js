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
 * GetFirstCharacterPosition
 *   XXX
 *
 **************************************************************************************************************************************************************/

'use strict';


/**
 * Get the position of the first character out of all strings inside an array
 *
 * @param  {array} lines - An array of strings
 *
 * @return {integer}     - The position of the first character
 */
function GetFirstCharacterPosition( lines ) {
	const earliest = lines.reduce(
		( prevLine, line ) => ( ( line.length - line.trimStart().length ) < ( prevLine.length - prevLine.trimStart().length ) && line !== '' ? line : prevLine )
		, lines[ 0 ]
	);

	return ( earliest.length - earliest.trimStart().length );
};


module.exports = exports = {
	GetFirstCharacterPosition,
};
