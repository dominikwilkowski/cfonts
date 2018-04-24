/***************************************************************************************************************************************************************
 *
 * RenderConsole unit tests
 *
 **************************************************************************************************************************************************************/


const StripColor = require('./_common.js').StripColor;
const CFonts = require('../src/lib.js');
const RenderConsole = CFonts.__test__.RenderConsole;


test(`RenderConsole - Should output default text`, () => {
	const test = RenderConsole( 'test', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test.output ).toEqual( ['test'] );
	expect( test.lines ).toEqual( 1 );
});


test(`RenderConsole - Should output multi-line text`, () => {
	const test = RenderConsole( 'test|test', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test.output ).toEqual( ['test', 'test'] );
	expect( test.lines ).toEqual( 2 );
});


test(`RenderConsole - Should output long text`, () => {
	const test = RenderConsole( 'this is a very long line to test multi lines', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 10, height: 10 } );

	expect( test.output ).toEqual( ['this is a', 'very long', 'line to te', 'st multi l', 'ines'] );
	expect( test.lines ).toEqual( 5 );
});


test(`RenderConsole - Should center align text`, () => {
	const test = RenderConsole( 'center', {
		align: 'center',
		colors: [],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 10, height: 10 } );

	expect( test.output ).toEqual( ['  center'] );
	expect( test.lines ).toEqual( 1 );
});


test(`RenderConsole - Should right align text`, () => {
	const test = RenderConsole( 'right', {
		align: 'right',
		colors: [],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 10, height: 10 } );

	expect( test.output ).toEqual( ['     right'] );
	expect( test.lines ).toEqual( 1 );
});


test(`RenderConsole - Should insert letter spacing`, () => {
	const test1 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 2,
		lineHeight: 1,
	}, { width: 10, height: 10 } );

	expect( test1.output ).toEqual( ['t e x t'] );
	expect( test1.lines ).toEqual( 1 );

	const test2 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 3,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test2.output ).toEqual( ['t  e  x  t'] );
	expect( test2.lines ).toEqual( 1 );

	const test3 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 10,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test3.output ).toEqual( ['t         e         x         t'] );
	expect( test3.lines ).toEqual( 1 );
});


test(`RenderConsole - Should insert letter spacing with multi lines`, () => {
	const test1 = RenderConsole( 'text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 2,
		lineHeight: 1,
	}, { width: 10, height: 10 } );

	expect( test1.output ).toEqual( ['t e x t', 't e x t'] );
	expect( test1.lines ).toEqual( 2 );

	const test2 = RenderConsole( 'text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 3,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test2.output ).toEqual( ['t  e  x  t', 't  e  x  t'] );
	expect( test2.lines ).toEqual( 2 );

	const test3 = RenderConsole( 'text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 10,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test3.output ).toEqual( ['t         e         x         t', 't         e         x         t'] );
	expect( test3.lines ).toEqual( 2 );
});


test(`RenderConsole - Should insert line height`, () => {
	const test1 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 2,
	}, { width: 100, height: 10 } );

	expect( test1.output ).toEqual( ['text', ''] );
	expect( test1.lines ).toEqual( 2 );

	const test2 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 3,
	}, { width: 100, height: 10 } );

	expect( test2.output ).toEqual( ['text', '', ''] );
	expect( test2.lines ).toEqual( 3 );

	const test3 = RenderConsole( 'text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 10,
	}, { width: 100, height: 10 } );

	expect( test3.output ).toEqual( ['text', '', '', '', '', '', '', '', '', ''] );
	expect( test3.lines ).toEqual( 10 );
});


test(`RenderConsole - Should insert line height between lines`, () => {
	const test1 = RenderConsole( 'text|text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 2,
	}, { width: 100, height: 10 } );

	expect( test1.output ).toEqual( ['text', '', 'text', '', 'text', ''] );
	expect( test1.lines ).toEqual( 6 );

	const test2 = RenderConsole( 'text|text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 3,
	}, { width: 100, height: 10 } );

	expect( test2.output ).toEqual( ['text', '', '', 'text', '', '', 'text', '', ''] );
	expect( test2.lines ).toEqual( 9 );

	const test3 = RenderConsole( 'text|text|text', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 10,
	}, { width: 100, height: 10 } );

	expect( test3.output ).toEqual([
		'text', '', '', '', '', '', '', '', '', '',
		'text', '', '', '', '', '', '', '', '', '',
		'text', '', '', '', '', '', '', '', '', '',
	]);
	expect( test3.lines ).toEqual( 30 );
});


test(`RenderConsole - Should insert line height for long lines`, () => {
	const test1 = RenderConsole( 'this is a long line', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 2,
	}, { width: 10, height: 10 } );

	expect( test1.output ).toEqual( ['this is a', '', 'long line', ''] );
	expect( test1.lines ).toEqual( 4 );

	const test2 = RenderConsole( 'this is a long line', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 3,
	}, { width: 10, height: 10 } );

	expect( test2.output ).toEqual( ['this is a', '', '', 'long line', '', ''] );
	expect( test2.lines ).toEqual( 6 );

	const test3 = RenderConsole( 'this is a long line', {
		align: 'left',
		colors: [],
		letterSpacing: 1,
		lineHeight: 10,
	}, { width: 10, height: 10 } );

	expect( test3.output ).toEqual([
		'this is a', '', '', '', '', '', '', '', '', '',
		'long line', '', '', '', '', '', '', '', '', '',
	]);
	expect( test3.lines ).toEqual( 20 );
});


test(`RenderConsole - Should output colored text`, () => {
	const test = RenderConsole( 'test', {
		align: 'left',
		colors: ['red'],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( test.output ).toEqual( ['test'] );
});


test(`RenderConsole - Should output candy colored text`, () => {
	const test = RenderConsole( 'test', {
		align: 'left',
		colors: ['candy'],
		letterSpacing: 1,
		lineHeight: 1,
	}, { width: 100, height: 10 } );

	expect( StripColor( test.output[ 0 ] ) ).toEqual( 'test' );
	expect( require('util').inspect( test.output[ 0 ] ).length > 'test'.length ).toEqual( true );
	expect( test.lines ).toEqual( 1 );
});
