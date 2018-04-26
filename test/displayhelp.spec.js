/***************************************************************************************************************************************************************
 *
 * DisplayHelp unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const DisplayHelp = CFonts.__test__.DisplayHelp;


test(`DisplayHelp - Show the help`, () => {
	console.log = jest.fn();

	DisplayHelp();

	expect( console.log.mock.calls.length > 0 ).toBe( true );
});
