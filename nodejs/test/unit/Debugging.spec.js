/***************************************************************************************************************************************************************
 *
 * Debugging unit tests
 *
 **************************************************************************************************************************************************************/

const { DEBUG, Debugging } = require('../../src/Debugging.js');

test(`DEBUG.enabled - Should be defined as boolean`, () => {
	expect(typeof DEBUG.enabled).toEqual('boolean');
});

test(`DEBUG.level - Should be defined as number`, () => {
	expect(typeof DEBUG.level).toEqual('number');
});

test(`Debugging - Show headline message when debug is enabled`, () => {
	console.log = jest.fn();

	Debugging.headline('text', 1, true, 1);

	expect(console.log.mock.calls[0][0]).toBe('\u001b[40m\n\u001b[1m ☑  \u001b[22m text\u001b[49m');
});

test(`Debugging - Show report message when debug is enabled`, () => {
	console.log = jest.fn();

	Debugging.report('text', 1, true, 1);

	expect(console.log.mock.calls[0][0]).toBe(
		'\x1B[40m\n\x1B[1m\x1B[32m ☑  \x1B[39m\x1B[22m \x1B[37mtext\x1B[39m\x1B[49m'
	);
});

test(`Debugging - Show error message when debug is enabled`, () => {
	console.error = jest.fn();

	Debugging.error('text', 1, true, 1);

	expect(console.error.mock.calls[0][0]).toBe('\x1B[40m\n\x1B[31m ☒  \x1B[39m \x1B[37mtext\x1B[39m\x1B[49m');
});

test(`Debugging - Don’t show message when debug is disabled`, () => {
	console.log = jest.fn();

	Debugging.headline('text', 1, false, 1);
	Debugging.report('text', 1, false, 1);
	Debugging.error('text', 1, false, 1);

	expect(console.log.mock.calls.length).toBe(0);
});

test(`Debugging - Don’t show message when debuglevel is too high`, () => {
	console.log = jest.fn();

	Debugging.headline('text', 1, true, 2);
	Debugging.report('text', 1, true, 2);
	Debugging.error('text', 1, true, 2);

	expect(console.log.mock.calls.length).toBe(0);
});

test(`Debugging - Debugging is off by default`, () => {
	console.log = jest.fn();

	Debugging.headline('text');
	Debugging.report('text');
	Debugging.error('text');

	expect(console.log.mock.calls.length).toBe(0);
});
