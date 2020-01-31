/***************************************************************************************************************************************************************
 *
 * GetOptions unit tests
 *
 **************************************************************************************************************************************************************/


const { GetOptions } = require('../../src/GetOptions.js');


test(`GetOptions - Should return default options`, () => {
	expect( GetOptions({}) ).toEqual({
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
	});
});


test(`GetOptions - Should be able to handle casing`, () => {
	const COLORS = { color1: 'coLor1', color2: 'coLor2', color3: 'coLor3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'fonT1', font2: 'FONT2', font3: 'fONt3' };

	expect( GetOptions(
		{
			colors: ['color1', 'color3'],
			background: 'bgcolor2',
			font: 'font3',
		},
		COLORS,
		BGCOLORS,
		FONTFACES
	)).toEqual({
		font: 'fONt3',
		align: 'left',
		colors: [ 'coLor1', 'coLor3' ],
		background: 'bgCOLor2',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});
});


test(`GetOptions - Should be able to handle background and backgroundColor`, () => {
	const COLORS = { color1: 'color1', color2: 'color2', color3: 'color3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'font1', font2: 'font2', font3: 'font3' };

	expect( GetOptions(
		{
			backgroundColor: 'bgcolor3',
		},
		COLORS,
		BGCOLORS,
		FONTFACES
	)).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'bgcoLOR3',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});
});


test(`GetOptions - Should merge options with defaults`, () => {
	expect( GetOptions({ font: 'xxx' }) ).toEqual({
		font: 'xxx',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ align: 'xxx' }) ).toEqual({
		font: 'block',
		align: 'xxx',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ colors: ['xxx'] }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: ['xxx'],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ background: 'xxx' }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'xxx',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ backgroundColor: 'xxx' }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'xxx',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ letterSpacing: 555 }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 555,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ lineHeight: 555 }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 555,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ space: false }) ).toEqual({
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
	});

	expect( GetOptions({ maxLength: 555 }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 555,
		gradient: false,
		independentGradient: false,
	});

	expect( GetOptions({ gradient: 'red,green' }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: ['red', 'green'],
		independentGradient: false,
	});

	expect( GetOptions({ gradient: ['red','green'] }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: ['red', 'green'],
		independentGradient: false,
	});

	expect( GetOptions({ independentGradient: true }) ).toEqual({
		font: 'block',
		align: 'left',
		colors: [],
		background: 'transparent',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: true,
	});
});
