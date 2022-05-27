/***************************************************************************************************************************************************************
 *
 * CleanInput unit tests
 *
 **************************************************************************************************************************************************************/

const { UpperCaseFirst } = require('../../src/UpperCaseFirst.js');

test(`UpperCaseFirst - Should uppercase the first character`, () => {
	expect(UpperCaseFirst('')).toEqual('');
	expect(UpperCaseFirst('abcd')).toEqual('Abcd');
	expect(UpperCaseFirst('ABCD')).toEqual('ABCD');
	expect(UpperCaseFirst('test')).toEqual('Test');
});

test(`UpperCaseFirst - Should not cause issues`, () => {
	expect(UpperCaseFirst('')).toEqual('');
	expect(UpperCaseFirst(null)).toEqual(null);
	expect(UpperCaseFirst(undefined)).toEqual(undefined);
	expect(UpperCaseFirst(1)).toEqual(1);
	expect(UpperCaseFirst(' abcd')).toEqual(' abcd');
});
