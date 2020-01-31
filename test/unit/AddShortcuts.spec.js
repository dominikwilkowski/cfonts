/***************************************************************************************************************************************************************
 *
 * AddShortcuts unit tests
 *
 **************************************************************************************************************************************************************/


const { AddShortcuts } = require('../../src/AddShortcuts.js');


test(`AddShortcuts - Should flatten shortcuts into object`, () => {
	const input = {
		'one': {
			description: 'desc value',
			short: 'oneshort',
		},
		'two': {
			description: 'desc value',
			short: 'twoshort',
		},
	};

	const output = {
		'one': {
			_name: 'one',
			description: 'desc value',
			short: 'oneshort',
		},
		'oneshort': {
			_name: 'one',
			description: 'desc value',
			short: 'oneshort',
		},
		'two': {
			_name: 'two',
			description: 'desc value',
			short: 'twoshort',
		},
		'twoshort': {
			_name: 'two',
			description: 'desc value',
			short: 'twoshort',
		},
	};

	expect( AddShortcuts( input ) ).toEqual( output );
});
