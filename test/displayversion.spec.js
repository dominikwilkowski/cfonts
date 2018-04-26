/***************************************************************************************************************************************************************
 *
 * DisplayVersion unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const DisplayVersion = CFonts.DisplayVersion;


test(`DisplayVersion - Show the version`, () => {
	console.log = jest.fn();

	DisplayVersion();

	expect( console.log.mock.calls.length > 0 ).toBe( true );
});
