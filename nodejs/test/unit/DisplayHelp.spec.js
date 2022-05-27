/***************************************************************************************************************************************************************
 *
 * DisplayHelp unit tests
 *
 **************************************************************************************************************************************************************/

const { DisplayHelp } = require('../../src/DisplayHelp.js');

test(`DisplayHelp - Show the help`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	DisplayHelp();

	expect(console.log.mock.calls.length > 0).toBe(true);
	expect(console.error.mock.calls.length === 0).toBe(true);
});
