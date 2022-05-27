/***************************************************************************************************************************************************************
 *
 * AlignText unit tests
 *
 **************************************************************************************************************************************************************/

const { AlignText } = require('../../src/AlignText.js');

test(`AlignText - Should align text in the center`, () => {
	expect(AlignText(['x'], 1, 1, 'center', { width: 21 })).toEqual(['          x']);
	expect(AlignText(['x'], 1, 1, 'center', { width: 1 })).toEqual(['x']);
});

test(`AlignText - Should align text on the right`, () => {
	expect(AlignText(['x'], 1, 1, 'right', { width: 11 })).toEqual(['          x']);
	expect(AlignText(['x'], 1, 1, 'right', { width: 1 })).toEqual(['x']);
});

test(`AlignText - Should do nothing on left alignment`, () => {
	expect(AlignText(['x'], 1, 1, 'left', { width: 11 })).toEqual(['x']);
});

test(`AlignText - Should do nothing when using Size default value`, () => {
	expect(AlignText(['x'], 1, 1, 'left')).toEqual(['x']);
});

test(`AlignText - Should do nothing when align top or bottom`, () => {
	expect(AlignText(['x'], 1, 1, 'top', { width: 21 })).toEqual(['x']);
	expect(AlignText(['x'], 1, 1, 'top', { width: 1 })).toEqual(['x']);
	expect(AlignText(['x'], 1, 1, 'bottom', { width: 21 })).toEqual(['x']);
	expect(AlignText(['x'], 1, 1, 'bottom', { width: 1 })).toEqual(['x']);
});
