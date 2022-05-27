/***************************************************************************************************************************************************************
 *
 * AddLine unit tests
 *
 **************************************************************************************************************************************************************/

const { AddLine } = require('../../src/AddLine.js');

test(`AddLine - Adding a line to a single-line font`, () => {
	const input = ['line 1', 'line 2'];

	expect(AddLine([...input], 1, [''], 0)).toEqual([...input, '']);
	expect(AddLine([...input], 1, [''], 1)).toEqual([...input, '', '']);
	expect(AddLine([...input], 1, [''], 5)).toEqual([...input, '', '', '', '', '', '']);
});

test(`AddLine - Adding a line to a multi-line font`, () => {
	const input = [',-, ,-,', '|-| |-}', '| | |_}'];

	expect(AddLine([...input], 3, ['', '', ''], 0)).toEqual([...input, '', '', '']);
	expect(AddLine([...input], 3, ['', '', ''], 1)).toEqual([...input, '', '', '', '']);
	expect(AddLine([...input], 3, ['', '', ''], 10)).toEqual([
		...input,
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
		'',
	]);
});
