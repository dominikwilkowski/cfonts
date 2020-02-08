/***************************************************************************************************************************************************************
 *
 * Render unit tests
 *
 **************************************************************************************************************************************************************/


const { Render } = require('../../src/Render.js');


beforeEach(() => {
	jest.resetModules();
});


test(`Render - Catch when a fontface is missing`, () => {
	console.error = jest.fn();

	jest.doMock('../../src/GetFont.js', () => ({ GetFont: jest.fn().mockReturnValue( false ) }) );
	const { GetFont } = require('../../src/GetFont.js');

	const { Render } = require('../../src/Render.js');

	const test = Render( 'x' );

	expect( GetFont.mock.calls.length ).toBe( 1 );
	expect( console.error.mock.calls.length ).toBe( 1 );
});


test(`Render - Render console string`, () => {
	const test = Render( 'text', {
		font: 'console',
	}, false, 1, { width: 100, height: 10 });

	expect( test.string ).toBe( '\n\ntext\n\n' );
	expect( test.array ).toEqual( ['\n\ntext\n\n'] );
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'console',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 0,
		lineHeight: 0,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Render console string with a color`, () => {
	const test = Render( 'text', {
		font: 'console',
		colors: ['red'],
	}, false, 1, { width: 100, height: 10 });

	expect( test.string ).toBe( '\n\n\u001b[38;2;255;0;0mtext\u001b[39m\n\n' );
	expect( test.array ).toEqual( ['\n\n\u001b[38;2;255;0;0mtext\u001b[39m\n\n'] );
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'console',
		align: 'left',
		colors: ['red'],
		background: 'transparent',
		letterSpacing: 0,
		lineHeight: 0,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Render console string with gradient will ignore color`, () => {
	const test = Render( 'text', {
		font: 'console',
		colors: ['red'],
		gradient: ['red','blue'],
	}, false, 1, { width: 100, height: 10 });

	expect( test.string ).toBe( '\n\n\u001b[38;2;255;0;0mt\u001b[39m\u001b[38;2;170;255;0me\u001b[39m\u001b[38;2;0;255;169mx\u001b[39m\u001b[38;2;0;0;255mt\u001b[39m\n\n' );
	expect( test.array ).toEqual( ['\n\n\u001b[38;2;255;0;0mt\u001b[39m\u001b[38;2;170;255;0me\u001b[39m\u001b[38;2;0;255;169mx\u001b[39m\u001b[38;2;0;0;255mt\u001b[39m\n\n'] );
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'console',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 0,
		lineHeight: 0,
		space: true,
		maxLength: 0,
		gradient: ['red','blue'],
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Fail on bad user input`, () => {
	console.error = jest.fn();

	const test1 = Render( void( 0 ), {}, false, 1, { width: 100, height: 10 });
	const test2 = Render( 'text', { font: 'notfound' }, false, 1, { width: 100, height: 10 });
	const test3 = Render( 'text', { align: 'notfound' }, false, 1, { width: 100, height: 10 });
	const test4 = Render( 'text', { colors: ['notfound'] }, false, 1, { width: 100, height: 10 });
	const test5 = Render( 'text', { background: 'notfound' }, false, 1, { width: 100, height: 10 });

	expect( test1 ).toBe( false );
	expect( test2 ).toBe( false );
	expect( test3 ).toBe( false );
	expect( test4 ).toBe( false );
	expect( test5 ).toBe( false );
});


test(`Render - Output debug infos`, () => {
	console.log = jest.fn();

	Render( 'text', {}, true, 1, { width: 100, height: 10 } );

	expect( console.log.mock.calls.length > 0 ).toBe( true );
	expect( console.log.mock.calls[ 0 ][ 0 ].includes('OPTIONS:') ).toBe( true );
});


test(`Render - Render block font`, () => {
	const test = Render( 'text', {}, false, 1, { width: 100, height: 10 } );

	expect( test.string ).toBe(
		'\n\n' +
		' ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'    ██║    █████╗    ╚███╔╝     ██║    \n' +
		'    ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'    ██║    █████╗    ╚███╔╝     ██║    ',
		'    ██║    ██╔══╝    ██╔██╗     ██║    ',
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Render letter spacing`, () => {
	const test1 = Render( 'text', {
		letterSpacing: 2,
	}, false, 1, { width: 100, height: 10 });

	expect( test1.string ).toBe(
		'\n\n' +
		'  ████████╗  ███████╗  ██╗  ██╗  ████████╗  \n' +
		'  ╚══██╔══╝  ██╔════╝  ╚██╗██╔╝  ╚══██╔══╝  \n' +
		'     ██║     █████╗     ╚███╔╝      ██║     \n' +
		'     ██║     ██╔══╝     ██╔██╗      ██║     \n' +
		'     ██║     ███████╗  ██╔╝ ██╗     ██║     \n' +
		'     ╚═╝     ╚══════╝  ╚═╝  ╚═╝     ╚═╝     \n\n'
	);
	expect( test1.array ).toEqual([
		'\n\n  ████████╗  ███████╗  ██╗  ██╗  ████████╗  ',
		'  ╚══██╔══╝  ██╔════╝  ╚██╗██╔╝  ╚══██╔══╝  ',
		'     ██║     █████╗     ╚███╔╝      ██║     ',
		'     ██║     ██╔══╝     ██╔██╗      ██║     ',
		'     ██║     ███████╗  ██╔╝ ██╗     ██║     ',
		'     ╚═╝     ╚══════╝  ╚═╝  ╚═╝     ╚═╝     \n\n',
	]);
	expect( test1.lines ).toBe( 1 );
	expect( test1.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 2,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});


	const test2 = Render( 'text', {
		letterSpacing: 10,
	}, false, 1, { width: 100, height: 10 });

	expect( test2.string ).toBe(
		'\n\n' +
		'          ████████╗          ███████╗          ██╗  ██╗          ████████╗          \n' +
		'          ╚══██╔══╝          ██╔════╝          ╚██╗██╔╝          ╚══██╔══╝          \n' +
		'             ██║             █████╗             ╚███╔╝              ██║             \n' +
		'             ██║             ██╔══╝             ██╔██╗              ██║             \n' +
		'             ██║             ███████╗          ██╔╝ ██╗             ██║             \n' +
		'             ╚═╝             ╚══════╝          ╚═╝  ╚═╝             ╚═╝             \n\n'
	);
	expect( test2.array ).toEqual([
		'\n\n          ████████╗          ███████╗          ██╗  ██╗          ████████╗          ',
		'          ╚══██╔══╝          ██╔════╝          ╚██╗██╔╝          ╚══██╔══╝          ',
		'             ██║             █████╗             ╚███╔╝              ██║             ',
		'             ██║             ██╔══╝             ██╔██╗              ██║             ',
		'             ██║             ███████╗          ██╔╝ ██╗             ██║             ',
		'             ╚═╝             ╚══════╝          ╚═╝  ╚═╝             ╚═╝             \n\n',
	]);
	expect( test2.lines ).toBe( 1 );
	expect( test2.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 10,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Center align block font`, () => {
	const test = Render( 'text', {
		align: 'center',
	}, false, 1, { width: 50, height: 10 });

	expect( test.string ).toBe(
		'\n\n' +
		'      ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		'      ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'         ██║    █████╗    ╚███╔╝     ██║    \n' +
		'         ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'         ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'         ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n      ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		'      ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'         ██║    █████╗    ╚███╔╝     ██║    ',
		'         ██║    ██╔══╝    ██╔██╗     ██║    ',
		'         ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'         ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'center',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Right align block font`, () => {
	const test = Render( 'text', {
		align: 'right',
	}, false, 1, { width: 50, height: 10 });

	expect( test.string ).toBe(
		'\n\n' +
		'            ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		'            ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'               ██║    █████╗    ╚███╔╝     ██║    \n' +
		'               ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'               ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'               ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n            ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		'            ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'               ██║    █████╗    ╚███╔╝     ██║    ',
		'               ██║    ██╔══╝    ██╔██╗     ██║    ',
		'               ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'               ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'right',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Break into new line on smaller viewports`, () => {
	const test = Render( 'text', {}, false, 1, { width: 20, height: 10 });

	expect( test.string ).toBe(
		'\n\n' +
		' ████████╗ ███████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ \n' +
		'    ██║    █████╗   \n' +
		'    ██║    ██╔══╝   \n' +
		'    ██║    ███████╗ \n' +
		'    ╚═╝    ╚══════╝ \n\n' +
		' ██╗  ██╗ ████████╗ \n' +
		' ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'  ╚███╔╝     ██║    \n' +
		'  ██╔██╗     ██║    \n' +
		' ██╔╝ ██╗    ██║    \n' +
		' ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n ████████╗ ███████╗ ',
		' ╚══██╔══╝ ██╔════╝ ',
		'    ██║    █████╗   ',
		'    ██║    ██╔══╝   ',
		'    ██║    ███████╗ ',
		'    ╚═╝    ╚══════╝ ',
		'',
		' ██╗  ██╗ ████████╗ ',
		' ╚██╗██╔╝ ╚══██╔══╝ ',
		'  ╚███╔╝     ██║    ',
		'  ██╔██╗     ██║    ',
		' ██╔╝ ██╗    ██║    ',
		' ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 2 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Add line break`, () => {
	const test = Render( 'te|xt', {}, false, 1, { width: 100, height: 10 });

	expect( test.string ).toBe(
		'\n\n' +
		' ████████╗ ███████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ \n' +
		'    ██║    █████╗   \n' +
		'    ██║    ██╔══╝   \n' +
		'    ██║    ███████╗ \n' +
		'    ╚═╝    ╚══════╝ \n\n' +
		' ██╗  ██╗ ████████╗ \n' +
		' ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'  ╚███╔╝     ██║    \n' +
		'  ██╔██╗     ██║    \n' +
		' ██╔╝ ██╗    ██║    \n' +
		' ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n ████████╗ ███████╗ ',
		' ╚══██╔══╝ ██╔════╝ ',
		'    ██║    █████╗   ',
		'    ██║    ██╔══╝   ',
		'    ██║    ███████╗ ',
		'    ╚═╝    ╚══════╝ ',
		'',
		' ██╗  ██╗ ████████╗ ',
		' ╚██╗██╔╝ ╚══██╔══╝ ',
		'  ╚███╔╝     ██║    ',
		'  ██╔██╗     ██║    ',
		' ██╔╝ ██╗    ██║    ',
		' ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 2 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Add line height`, () => {
	const test = Render( 'te|xt', { lineHeight: 2 }, false, 1, { width: 100, height: 10 });

	expect( test.string ).toBe(
		'\n\n' +
		' ████████╗ ███████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ \n' +
		'    ██║    █████╗   \n' +
		'    ██║    ██╔══╝   \n' +
		'    ██║    ███████╗ \n' +
		'    ╚═╝    ╚══════╝ \n\n\n' +
		' ██╗  ██╗ ████████╗ \n' +
		' ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'  ╚███╔╝     ██║    \n' +
		'  ██╔██╗     ██║    \n' +
		' ██╔╝ ██╗    ██║    \n' +
		' ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n ████████╗ ███████╗ ',
		' ╚══██╔══╝ ██╔════╝ ',
		'    ██║    █████╗   ',
		'    ██║    ██╔══╝   ',
		'    ██║    ███████╗ ',
		'    ╚═╝    ╚══════╝ ',
		'',
		'',
		' ██╗  ██╗ ████████╗ ',
		' ╚██╗██╔╝ ╚══██╔══╝ ',
		'  ╚███╔╝     ██║    ',
		'  ██╔██╗     ██║    ',
		' ██╔╝ ██╗    ██║    ',
		' ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 2 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 2,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Non supported characters are ignored`, () => {
	const test = Render( 'te*xt', {}, false, 1, { width: 100, height: 10 } );

	expect( test.string ).toBe(
		'\n\n' +
		' ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'    ██║    █████╗    ╚███╔╝     ██║    \n' +
		'    ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n'
	);
	expect( test.array ).toEqual([
		'\n\n ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'    ██║    █████╗    ╚███╔╝     ██║    ',
		'    ██║    ██╔══╝    ██╔██╗     ██║    ',
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Remove space`, () => {
	const test = Render( 'text', { space: false }, false, 1, { width: 100, height: 10 } );

	expect( test.string ).toBe(
		' ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'    ██║    █████╗    ╚███╔╝     ██║    \n' +
		'    ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    '
	);
	expect( test.array ).toEqual([
		' ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'    ██║    █████╗    ╚███╔╝     ██║    ',
		'    ██║    ██╔══╝    ██╔██╗     ██║    ',
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    ',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: false,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Add background color`, () => {
	const test = Render( 'text', { background: 'red' }, false, 1, { width: 100, height: 10 } );

	expect( test.string ).toBe(
		'\u001b[48;2;255;0;0m' +
		'\n\n\n' +
		' ████████╗ ███████╗ ██╗  ██╗ ████████╗ \n' +
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ \n' +
		'    ██║    █████╗    ╚███╔╝     ██║    \n' +
		'    ██║    ██╔══╝    ██╔██╗     ██║    \n' +
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    \n' +
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n' +
		'\u001b[49m'
	);
	expect( test.array ).toEqual([
		'\u001b[48;2;255;0;0m\n\n\n ████████╗ ███████╗ ██╗  ██╗ ████████╗ ',
		' ╚══██╔══╝ ██╔════╝ ╚██╗██╔╝ ╚══██╔══╝ ',
		'    ██║    █████╗    ╚███╔╝     ██║    ',
		'    ██║    ██╔══╝    ██╔██╗     ██║    ',
		'    ██║    ███████╗ ██╔╝ ██╗    ██║    ',
		'    ╚═╝    ╚══════╝ ╚═╝  ╚═╝    ╚═╝    \n\n\u001b[49m',
	]);
	expect( test.lines ).toBe( 1 );
	expect( test.options ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'red',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
	});
});


test(`Render - Returns false if font is not known`, () => {
	const test = Render( 'text', {
		font: 'unknown',
	}, false, 1, { width: 100, height: 10 });

	expect( test ).toBe( false );
});
