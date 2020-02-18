/***************************************************************************************************************************************************************
 *
 * Options unit tests
 *
 **************************************************************************************************************************************************************/


const { Options } = require('../../src/Options.js');


const DEFAULTS = {
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
};

test(`Options - Should return default options`, () => {
	Options.set = {};
	expect( Options.get ).toEqual( DEFAULTS );
});


test(`Options - Should be able to handle casing`, () => {
	const COLORS = { color1: 'coLor1', color2: 'coLor2', color3: 'coLor3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'fonT1', font2: 'FONT2', font3: 'fONt3' };

	Options.set = DEFAULTS;
	Options.set = {
		colors: ['color1', 'color3'],
		background: 'bgcolor2',
		font: 'font3',
		allowedColors: COLORS,
		allowedBG: BGCOLORS,
		allowedFont: FONTFACES,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});


test(`Options - Should be able to handle background and backgroundColor`, () => {
	const COLORS = { color1: 'color1', color2: 'color2', color3: 'color3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'font1', font2: 'font2', font3: 'font3' };

	Options.set = DEFAULTS;
	Options.set = {
		backgroundColor: 'bgcolor3',
		allowedColors: COLORS,
		allowedBG: BGCOLORS,
		allowedFont: FONTFACES,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});


test(`Options - Should merge font option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		font: 'xxx',
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge align option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		align: 'xxx',
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge colors option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		colors: ['xxx'],
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge background option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		background: 'xxx',
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge backgroundColor option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		backgroundColor: 'xxx',
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge letterSpacing option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		letterSpacing: 555,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge lineHeight option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		lineHeight: 555,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge space option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		space: false,
	};

	expect( Options.get ).toEqual({
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

test(`Options - Should merge maxLength option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		maxLength: 555,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge string-gradient option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		gradient: 'red,green',
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge array-gradient option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		gradient: ['red','green'],
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge independentGradient option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		independentGradient: true,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: false,
	});
});

test(`Options - Should merge independentGradient option with defaults`, () => {
	Options.set = DEFAULTS;
	Options.set = {
		transitionGradient: true,
	};

	expect( Options.get ).toEqual({
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
		transitionGradient: true,
	});
});
