/***************************************************************************************************************************************************************
 *
 * GetOptions unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js')
const GetOptions = CFonts.__test__.GetOptions;


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
	});
});
