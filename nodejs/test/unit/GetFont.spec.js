/***************************************************************************************************************************************************************
 *
 * GetFont unit tests
 *
 **************************************************************************************************************************************************************/

const { FONTFACES } = require('../../src/constants.js');
const { GetFont } = require('../../src/GetFont.js');

const fontArray = [
	'name',
	'version',
	'homepage',
	'colors',
	'lines',
	'buffer',
	'letterspace',
	'letterspace_size',
	'chars',
];

Object.keys(FONTFACES)
	.filter((font) => font !== 'console')
	.map((fontkey) => {
		const font = FONTFACES[fontkey];

		test(`GetFont - ${font} font should exist and have the right keys`, () => {
			expect(Object.keys(GetFont(font))).toEqual(fontArray);
		});
	});

test(`GetFont - Should return false if the font doesn’t exist`, () => {
	expect(GetFont('does-not-exist')).toEqual(false);
});
