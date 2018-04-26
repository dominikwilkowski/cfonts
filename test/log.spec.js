/***************************************************************************************************************************************************************
 *
 * Log unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const Log = CFonts.__test__.Log;


test(`Log - Display error message`, () => {
	console.error = jest.fn();

	Log.error( 'text' );

	expect( console.error.mock.calls[0][0] )
		.toBe( '\n \u001b[1m\u001b[31mOuch:\u001b[39m\u001b[22m text\n' );
});


test(`Log - Indent each line in error message`, () => {
	console.error = jest.fn();

	Log.error( 'text\ntext' );

	expect( console.error.mock.calls[0][0] )
		.toBe( '\n \u001b[1m\u001b[31mOuch:\u001b[39m\u001b[22m text\n       text\n' );
});
