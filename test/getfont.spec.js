/***************************************************************************************************************************************************************
 *
 * GetFont unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const GetFont = CFonts.__test__.GetFont;


const fontArray = [
	'name',
	'version',
	'homepage',
	'colors',
	'lines',
	'buffer',
	'letterspace',
	'chars',
];


Object.keys( CFonts.__test__.FONTFACES )
	.filter( font => font !== 'console' )
	.map( font => {
		test(`GetFont - ${ font } font should exist and have the right keys`, () => {
			expect( Object.keys( GetFont( font ) ) ).toEqual( fontArray );
		});
});


test(`GetFont - Should return false if the font doesn’t exist`, () => {
	expect( GetFont( 'does-not-exist' ) ).toEqual( false );
});
