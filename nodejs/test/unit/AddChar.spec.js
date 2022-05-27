/***************************************************************************************************************************************************************
 *
 * AddChar unit tests
 *
 **************************************************************************************************************************************************************/

const { AddChar } = require('../../src/AddChar.js');

test(`AddChar - Add a single line letter without color`, () => {
	const fontChars = {
		A: ['A'],
	};

	expect(AddChar('A', [''], 1, fontChars, 1, [])).toEqual(['A']);
});

test(`AddChar - Add a single line letter with color`, () => {
	const fontChars = {
		A: ['A'],
	};

	expect(AddChar('A', [''], 1, fontChars, 1, ['red'])).toEqual(['\u001b[31mA\u001b[39m']);
});

test(`AddChar - Add a multi line letter without color`, () => {
	const fontChars = {
		A: [',-,', '|-|', '| |'],
	};

	expect(AddChar('A', ['', '', ''], 3, fontChars, 1, [])).toEqual(fontChars.A);
});

test(`AddChar - Add a multi line letter with color`, () => {
	const fontChars = {
		A: [',-,', '|-|', '| |'],
	};

	const result = ['\u001b[31m,-,\u001b[39m', '\u001b[31m|-|\u001b[39m', '\u001b[31m| |\u001b[39m'];

	expect(AddChar('A', ['', '', ''], 3, fontChars, 1, ['red'])).toEqual(result);
});

test(`AddChar - Add a multi line letter with multiple colors`, () => {
	const fontChars = {
		A: [',<c1>-</c1>,', '|-|', '| <c2>|</c2>'],
	};

	const result = [',\u001b[31m-\u001b[39m,', '|-|', '| \u001b[31m|\u001b[39m'];

	expect(AddChar('A', ['', '', ''], 3, fontChars, 2, ['red', 'red'])).toEqual(result);
});
