/***************************************************************************************************************************************************************
 *
 * Options unit tests
 *
 **************************************************************************************************************************************************************/

const { Options } = require('../../src/Options.js');

test(`Options - Should return default options`, () => {
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
		env: 'node',
	};

	Options.reset();
	Options.set = {};
	expect(Options.get).toEqual(DEFAULTS);
});

test(`Options - Should be able to handle casing`, () => {
	const COLORS = { color1: 'coLor1', color2: 'coLor2', color3: 'coLor3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'fonT1', font2: 'FONT2', font3: 'fONt3' };

	Options.reset();
	Options.set = {
		colors: ['color1', 'color3'],
		background: 'bgcolor2',
		font: 'font3',
		allowedColors: COLORS,
		allowedBG: BGCOLORS,
		allowedFont: FONTFACES,
	};

	expect(Options.get).toEqual({
		font: 'fONt3',
		align: 'left',
		colors: ['coLor1', 'coLor3'],
		background: 'bgCOLor2',
		letterSpacing: 1,
		lineHeight: 1,
		space: true,
		maxLength: 0,
		gradient: false,
		independentGradient: false,
		transitionGradient: false,
		env: 'node',
	});
});

test(`Options - Should be able to handle background and backgroundColor`, () => {
	const COLORS = { color1: 'color1', color2: 'color2', color3: 'color3' };
	const BGCOLORS = { bgcolor1: 'bgCOlor1', bgcolor2: 'bgCOLor2', bgcolor3: 'bgcoLOR3' };
	const FONTFACES = { font1: 'font1', font2: 'font2', font3: 'font3' };

	Options.reset();
	Options.set = {
		backgroundColor: 'bgcolor3',
		allowedColors: COLORS,
		allowedBG: BGCOLORS,
		allowedFont: FONTFACES,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge font option with defaults`, () => {
	Options.reset();
	Options.set = {
		font: 'xxx',
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge align option with defaults`, () => {
	Options.reset();
	Options.set = {
		align: 'xxx',
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge colors option with defaults`, () => {
	Options.reset();
	Options.set = {
		colors: ['xxx'],
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge background option with defaults`, () => {
	Options.reset();
	Options.set = {
		background: 'xxx',
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge backgroundColor option with defaults`, () => {
	Options.reset();
	Options.set = {
		backgroundColor: 'xxx',
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge letterSpacing option with defaults`, () => {
	Options.reset();
	Options.set = {
		letterSpacing: 555,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge lineHeight option with defaults`, () => {
	Options.reset();
	Options.set = {
		lineHeight: 555,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge space option with defaults`, () => {
	Options.reset();
	Options.set = {
		space: false,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge maxLength option with defaults`, () => {
	Options.reset();
	Options.set = {
		maxLength: 555,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge string-gradient option with defaults`, () => {
	Options.reset();
	Options.set = {
		gradient: 'red,green',
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge array-gradient option with defaults`, () => {
	Options.reset();
	Options.set = {
		gradient: ['red', 'green'],
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge independentGradient option with defaults`, () => {
	Options.reset();
	Options.set = {
		independentGradient: true,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge independentGradient option with defaults`, () => {
	Options.reset();
	Options.set = {
		transitionGradient: true,
	};

	expect(Options.get).toEqual({
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
		env: 'node',
	});
});

test(`Options - Should merge env option with defaults`, () => {
	Options.reset();
	Options.set = {
		env: 'browser',
	};

	expect(Options.get).toEqual({
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
		env: 'browser',
	});
});
