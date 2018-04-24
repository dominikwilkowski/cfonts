/***************************************************************************************************************************************************************
 *
 * CleanInput unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const CleanInput = CFonts.__test__.CleanInput;
const CHARS = CFonts.CHARS;


test(`CleanInput - Should white list characters`, () => {
	expect( CleanInput( 'abcd', ['A','B','C'] ) ).toEqual( 'abc' );
	expect( CleanInput( 'abdc', ['A','B','C'] ) ).toEqual( 'abc' );
	expect( CleanInput( 'ab c', ['A','B','C'] ) ).toEqual( 'abc' );
	expect( CleanInput( 'abc', ['A','B','C'] ) ).toEqual( 'abc' );
	expect( CleanInput( 'abc•', ['A','B','C'] ) ).toEqual( 'abc' );
	expect( CleanInput( ' abc', ['A','B','C'] ) ).toEqual( 'abc' );
});


test(`CleanInput - Should keep all letters that are allowed`, () => {
	expect( CleanInput( CHARS.join(' ') ) ).toEqual( CHARS.join(' ') );
});
