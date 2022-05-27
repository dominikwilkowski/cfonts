/***************************************************************************************************************************************************************
 *
 * DisplayVersion unit tests
 *
 **************************************************************************************************************************************************************/

const { DisplayVersion } = require('../../src/DisplayVersion.js');

test(`DisplayVersion - Show the version`, () => {
	console.log = jest.fn();

	DisplayVersion();

	expect(console.log.mock.calls.length > 0).toBe(true);
});
