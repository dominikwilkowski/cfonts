/***************************************************************************************************************************************************************
 *
 * CleanInput unit tests
 *
 **************************************************************************************************************************************************************/

const { CleanInput } = require('../../src/CleanInput.js');
const { CHARS } = require('../../src/constants.js');

test(`CleanInput - Should white list characters`, () => {
	expect(CleanInput('abcd', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('abdc', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('ab c', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('abc', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('abc•', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput(' abc', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('ab c', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('a|b|c', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('a\nb\r\nc', ['A', 'B', 'C'])).toEqual('abc');
	expect(CleanInput('ab\tc', ['A', 'B', 'C', 'T'])).toEqual('abc');
	expect(CleanInput('abtc', ['A', 'B', 'C', 'T'])).toEqual('abtc');
});

test(`CleanInput - Should keep all letters that are allowed`, () => {
	expect(CleanInput(CHARS.join(' '))).toEqual(CHARS.join(' '));
});
