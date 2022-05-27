/***************************************************************************************************************************************************************
 *
 * CharLength unit tests
 *
 **************************************************************************************************************************************************************/

const { CharLength } = require('../../src/CharLength.js');

test(`CharLength - Should return the largest character length`, () => {
	const character1 = [''];
	const character2 = ['', '  '];
	const character3 = ['      ', ''];
	const character4 = ['', '   ', ''];
	const character5 = [' ', '', '        ', '', '            ', ' '];

	expect(CharLength(character1, character1.length, 0)).toEqual(0);
	expect(CharLength(character2, character2.length, 0)).toEqual(2);
	expect(CharLength(character3, character3.length, 0)).toEqual(6);
	expect(CharLength(character4, character4.length, 0)).toEqual(3);
	expect(CharLength(character5, character5.length, 0)).toEqual(12);

	expect(CharLength(character1, character1.length, 5)).toEqual(1);
	expect(CharLength(character2, character2.length, 5)).toEqual(2);
	expect(CharLength(character3, character3.length, 5)).toEqual(6);
	expect(CharLength(character4, character4.length, 5)).toEqual(3);
	expect(CharLength(character5, character5.length, 5)).toEqual(12);

	expect(CharLength(character1, character1.length, 1)).toEqual(1);
});
