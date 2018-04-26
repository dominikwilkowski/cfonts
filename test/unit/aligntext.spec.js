/***************************************************************************************************************************************************************
 *
 * AlignText unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../../src/lib.js');
const AlignText = CFonts.__test__.AlignText;


test(`AlignText - Should align text in the center`, () => {
	expect( AlignText( ['x'], 1, 1, 'center', { width: 21 } ) ).toEqual( ['          x'] );
	expect( AlignText( ['x'], 1, 1, 'center', { width: 1 } ) ).toEqual( ['x'] );
});


test(`AlignText - Should align text on the right`, () => {
	expect( AlignText( ['x'], 1, 1, 'right', { width: 11 } ) ).toEqual( ['          x'] );
	expect( AlignText( ['x'], 1, 1, 'right', { width: 1 } ) ).toEqual( ['x'] );
});


test(`AlignText - Should do nothing on left alignment`, () => {
	expect( AlignText( ['x'], 1, 1, 'left', { width: 11 } ) ).toEqual( ['x'] );
});
