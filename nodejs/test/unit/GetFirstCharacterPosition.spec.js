/***************************************************************************************************************************************************************
 *
 * GetFirstCharacterPosition unit tests
 *
 **************************************************************************************************************************************************************/

const { GetFirstCharacterPosition } = require('../../src/GetFirstCharacterPosition.js');

test(`GetFirstCharacterPosition - Get the first character of a padded string`, () => {
	expect(GetFirstCharacterPosition(['x'])).toBe(0);
	expect(GetFirstCharacterPosition(['  x'])).toBe(2);
	expect(GetFirstCharacterPosition(['    x'])).toBe(4);

	expect(GetFirstCharacterPosition(['     x', ' x', '   x'])).toBe(1);
	expect(GetFirstCharacterPosition(['     x', '', '   x'])).toBe(3);
});
