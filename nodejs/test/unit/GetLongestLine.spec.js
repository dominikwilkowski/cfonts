/***************************************************************************************************************************************************************
 *
 * GetLongestLine unit tests
 *
 **************************************************************************************************************************************************************/

const { GetLongestLine } = require('../../src/GetLongestLine.js');

test(`GetLongestLine - Should return the longest line of an array`, () => {
	expect(GetLongestLine(['x'])).toBe('x');

	expect(GetLongestLine(['x', 'xxxxxx', 'xx'])).toBe('xxxxxx');

	expect(GetLongestLine(['', 'xxx', 'x'])).toBe('xxx');
});
