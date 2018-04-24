/***************************************************************************************************************************************************************
 *
 * Size unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const Size = CFonts.__test__.Size;


test(`Size - Should have two keys with numbers`, () => {
	expect( typeof Size.width ).toEqual( 'number' );
	expect( typeof Size.height ).toEqual( 'number' );
});
