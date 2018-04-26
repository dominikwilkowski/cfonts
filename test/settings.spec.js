/***************************************************************************************************************************************************************
 *
 * DEBUG, DEBUGLEVEL, CHARS, COLORS, BGCOLORS, ALIGNMENT, FONTFACES unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const DEBUG = CFonts.__test__.DEBUG;
const DEBUGLEVEL = CFonts.__test__.DEBUGLEVEL;
const CHARS = CFonts.__test__.CHARS;
const COLORS = CFonts.__test__.COLORS;
const BGCOLORS = CFonts.__test__.BGCOLORS;
const ALIGNMENT = CFonts.__test__.ALIGNMENT;
const FONTFACES = CFonts.__test__.FONTFACES;
const CLIOPTIONS = CFonts.__test__.CLIOPTIONS;
const PACKAGE = CFonts.__test__.PACKAGE


test(`DEBUG - Should be defined as boolean`, () => {
	expect( typeof DEBUG ).toEqual( 'boolean' );
});


test(`DEBUGLEVEL - Should be defined as number`, () => {
	expect( typeof DEBUGLEVEL ).toEqual( 'number' );
});


test(`CHARS - Should have 56 characters defined`, () => {
	expect( CHARS.length ).toEqual( 56 );
});


test(`COLORS - Should have n number of colors defined`, () => {
	expect( Object.keys( COLORS ).length > 0 ).toEqual( true );
});


test(`BGCOLORS - Should have n number of colors defined`, () => {
	expect( Object.keys( BGCOLORS ).length > 0 ).toEqual( true );
});


test(`ALIGNMENT - Should have all alignment options`, () => {
	expect( ALIGNMENT[ 0 ] ).toEqual( 'left' );
	expect( ALIGNMENT[ 1 ] ).toEqual( 'center' );
	expect( ALIGNMENT[ 2 ] ).toEqual( 'right' );
});


test(`FONTFACES - Should have a bunch of fontfaces defined`, () => {
	expect( Object.keys( FONTFACES ).length > 0 ).toEqual( true );
});


test(`CLIOPTIONS - Should have all keys`, () => {
	Object.keys( CLIOPTIONS ).map( option => {
		expect( typeof CLIOPTIONS[ option ].description ).toEqual( 'string' );
		expect( typeof CLIOPTIONS[ option ].example ).toEqual( 'string' );
		expect( typeof CLIOPTIONS[ option ].short ).toEqual( 'string' );
		expect( typeof CLIOPTIONS[ option ].default !== undefined ).toEqual( true );
	});
});


test(`PACKAGE - Should exist and have a version key`, () => {
	expect( PACKAGE.version.length > 0 ).toEqual( true );
});
