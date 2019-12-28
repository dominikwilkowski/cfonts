/***************************************************************************************************************************************************************
 *
 * CleanInput unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../../src/lib.js');
const UpperCaseFirst = CFonts.__test__.UpperCaseFirst;


test(`UpperCaseFirst - Should uppercase the first character`, () => {
	expect( UpperCaseFirst( '' ) ).toEqual( '' );
	expect( UpperCaseFirst( 'abcd' ) ).toEqual( 'Abcd' );
	expect( UpperCaseFirst( 'ABCD' ) ).toEqual( 'ABCD' );
	expect( UpperCaseFirst( 'test' ) ).toEqual( 'Test' );
});

test(`UpperCaseFirst - Should not cause issues`, () => {
	expect( UpperCaseFirst( '' ) ).toEqual( '' );
	expect( UpperCaseFirst( null ) ).toEqual( null );
	expect( UpperCaseFirst( undefined ) ).toEqual( undefined );
	expect( UpperCaseFirst( 1 ) ).toEqual( 1 );
	expect( UpperCaseFirst( ' abcd' ) ).toEqual( ' abcd' );
});
