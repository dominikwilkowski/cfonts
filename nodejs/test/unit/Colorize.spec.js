/***************************************************************************************************************************************************************
 *
 * Colorize unit tests
 *
 **************************************************************************************************************************************************************/

const { StripColor } = require('./_common.js');
const { Colorize } = require('../../src/Colorize.js');

test(`Colorize - An undefined string will return just that`, () => {
	expect(Colorize(undefined, 1, [])).toEqual(undefined);
});

test(`Colorize - Strings without color wont change`, () => {
	const test = 'test string without color placeholders';

	expect(Colorize(test, 1, [])).toEqual(test);
});

test(`Colorize - Strings with one color are replaced`, () => {
	const test = 'test string with one color placeholders';

	expect(Colorize(test, 1, ['red'])).toEqual('\u001b[31mtest string with one color placeholders\u001b[39m');
});

test(`Colorize - Strings with two colors are replaced`, () => {
	const test = 'test string <c1>with</c1> one <c2>color</c2> placeholders';

	expect(Colorize(test, 2, ['red', '#ff8800'])).toEqual(
		'test string \u001b[31mwith\u001b[39m one \u001b[38;2;255;136;0mcolor\u001b[39m placeholders'
	);
});
