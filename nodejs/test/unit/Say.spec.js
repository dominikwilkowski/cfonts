/***************************************************************************************************************************************************************
 *
 * Say unit tests
 *
 **************************************************************************************************************************************************************/

const { Say } = require('../../src/Say.js');

test(`Say - Say will print to console log`, () => {
	console.log = jest.fn();

	Say('text');

	expect(console.log.mock.calls[0][0].length > 0).toBe(true);
});

test(`Say - Say not print anything when render returns an error`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Say('text', { font: 'bogus' });

	expect(console.log.mock.calls.length === 0).toBe(true);
	expect(console.error.mock.calls[0][0].length > 0).toBe(true);
});
