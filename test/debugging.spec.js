/***************************************************************************************************************************************************************
 *
 * Debugging unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const Debugging = CFonts.Debugging;


test(`Debugging - Show headline message when debug is enabled`, () => {
	console.log = jest.fn();

	Debugging.headline( 'text', 1, true, 1 );

	expect( console.log.mock.calls[0][0] ).toBe( '\u001b[47m\u001b[49m\n\u001b[47m\u001b[1m ☑  \u001b[22m text\u001b[49m' );
});


test(`Debugging - Show report message when debug is enabled`, () => {
	console.log = jest.fn();

	Debugging.report( 'text', 1, true, 1 );

	expect( console.log.mock.calls[0][0] )
		.toBe( '\u001b[47m\u001b[49m\n\u001b[47m\u001b[1m\u001b[32m ☑  \u001b[39m\u001b[22m \u001b[30mtext \u001b[39m\u001b[49m' );
});


test(`Debugging - Show error message when debug is enabled`, () => {
	console.error = jest.fn();

	Debugging.error( 'text', 1, true, 1 );

	expect( console.error.mock.calls[0][0] )
		.toBe( '\u001b[47m\u001b[49m\n\u001b[47m\u001b[31m ☒  \u001b[39m \u001b[30mtext \u001b[39m\u001b[49m' );
});


test(`Debugging - Don’t show message when debug is disabled`, () => {
	console.log = jest.fn();

	Debugging.headline( 'text', 1, false, 1 );
	Debugging.report( 'text', 1, false, 1 );
	Debugging.error( 'text', 1, false, 1 );

	expect( console.log.mock.calls.length ).toBe( 0 );
});


test(`Debugging - Don’t show message when debuglevel is too high`, () => {
	console.log = jest.fn();

	Debugging.headline( 'text', 1, true, 2 );
	Debugging.report( 'text', 1, true, 2 );
	Debugging.error( 'text', 1, true, 2 );

	expect( console.log.mock.calls.length ).toBe( 0 );
});
