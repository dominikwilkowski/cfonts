/***************************************************************************************************************************************************************
 *
 * AddLetterSpacing unit tests
 *
 **************************************************************************************************************************************************************/

const { AddLetterSpacing } = require('../../src/AddLetterSpacing.js');

test(`AddLetterSpacing - Add a letter space to the last line for single line font`, () => {
	expect(AddLetterSpacing(['', '', ''], 1, [' '], 1, [], 1)).toEqual(['', '', ' ']);
});

test(`AddLetterSpacing - Add a letter space even when the font doesn't have it specified`, () => {
	expect(AddLetterSpacing(['', '', ''], 1, [''], 1, [], 1)).toEqual(['', '', ' ']);
	expect(AddLetterSpacing(['', '', ''], 1, [''], 1, [], 2)).toEqual(['', '', '  ']);
});

test(`AddLetterSpacing - Add a letter space to the last line for single line font with one color`, () => {
	expect(AddLetterSpacing(['', '', ''], 1, [' '], 1, ['red'], 1)).toEqual(['', '', '\u001b[31m \u001b[39m']);
});

test(`AddLetterSpacing - Add a letter space to the last line for single line font with multiple colors`, () => {
	expect(AddLetterSpacing(['', '', ''], 1, ['<c1> </c1>'], 2, ['red', 'red'], 1)).toEqual([
		'',
		'',
		'\u001b[31m \u001b[39m',
	]);
	expect(AddLetterSpacing(['', '', ''], 1, ['<c2>#</c2>'], 2, ['red', 'red'], 1)).toEqual([
		'',
		'',
		'\u001b[31m#\u001b[39m',
	]);
});

test(`AddLetterSpacing - Add a letter space to the last line for multiple line font`, () => {
	expect(AddLetterSpacing(['', '', '', '', '', ''], 3, [' ', ' ', ' '], 1, [], 1)).toEqual(['', '', '', ' ', ' ', ' ']);
	expect(AddLetterSpacing(['', '', '', '', '', ''], 3, ['_', '_', '_'], 1, [], 1)).toEqual(['', '', '', '_', '_', '_']);
});

test(`AddLetterSpacing - Add a letter space to the last line for multiple line font with one color`, () => {
	expect(AddLetterSpacing(['', '', '', '', '', ''], 3, [' ', ' ', ' '], 1, ['red'], 1)).toEqual([
		'',
		'',
		'',
		'\u001b[31m \u001b[39m',
		'\u001b[31m \u001b[39m',
		'\u001b[31m \u001b[39m',
	]);
	expect(AddLetterSpacing(['', '', '', '', '', ''], 3, ['_', '_', '_'], 1, ['red'], 1)).toEqual([
		'',
		'',
		'',
		'\u001b[31m_\u001b[39m',
		'\u001b[31m_\u001b[39m',
		'\u001b[31m_\u001b[39m',
	]);
});

test(`AddLetterSpacing - Add a letter space to the last line for multiple line font with multiple colors`, () => {
	expect(
		AddLetterSpacing(['', '', '', '', '', ''], 3, ['<c1> </c1>', '<c1> </c1>', '<c1> </c1>'], 2, ['red', 'red'], 1)
	).toEqual(['', '', '', '\u001b[31m \u001b[39m', '\u001b[31m \u001b[39m', '\u001b[31m \u001b[39m']);
	expect(
		AddLetterSpacing(['', '', '', '', '', ''], 3, ['<c2> </c2>', '<c2> </c2>', '<c2> </c2>'], 2, ['red', 'red'], 1)
	).toEqual(['', '', '', '\u001b[31m \u001b[39m', '\u001b[31m \u001b[39m', '\u001b[31m \u001b[39m']);

	expect(
		AddLetterSpacing(['', '', '', '', '', ''], 3, ['<c1>_</c1>', '<c1>_</c1>', '<c1>_</c1>'], 2, ['red', 'red'], 1)
	).toEqual(['', '', '', '\u001b[31m_\u001b[39m', '\u001b[31m_\u001b[39m', '\u001b[31m_\u001b[39m']);
	expect(
		AddLetterSpacing(['', '', '', '', '', ''], 3, ['<c2>_</c2>', '<c2>_</c2>', '<c2>_</c2>'], 2, ['red', 'red'], 1)
	).toEqual(['', '', '', '\u001b[31m_\u001b[39m', '\u001b[31m_\u001b[39m', '\u001b[31m_\u001b[39m']);
});
