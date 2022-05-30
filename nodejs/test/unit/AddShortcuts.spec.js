/***************************************************************************************************************************************************************
 *
 * AddShortcuts unit tests
 *
 **************************************************************************************************************************************************************/

const { AddShortcuts } = require('../../src/AddShortcuts.js');

test(`AddShortcuts - Should flatten shortcuts into object`, () => {
	const input = {
		one: {
			description: 'desc value',
			fallback_shortcut: false,
			short: 'oneshort',
		},
		two: {
			description: 'desc value',
			fallback_shortcut: 'twofallback',
			short: 'twoshort',
		},
	};

	const output = {
		one: {
			_name: 'one',
			description: 'desc value',
			short: 'oneshort',
			fallback_shortcut: false,
		},
		oneshort: {
			_name: 'one',
			description: 'desc value',
			short: 'oneshort',
			fallback_shortcut: false,
		},
		two: {
			_name: 'two',
			description: 'desc value',
			short: 'twoshort',
			fallback_shortcut: 'twofallback',
		},
		twoshort: {
			_name: 'two',
			description: 'desc value',
			short: 'twoshort',
			fallback_shortcut: 'twofallback',
		},
		twofallback: {
			_name: 'two',
			description: 'desc value',
			short: 'twoshort',
			fallback_shortcut: 'twofallback',
		},
	};

	expect(AddShortcuts(input)).toEqual(output);
});
