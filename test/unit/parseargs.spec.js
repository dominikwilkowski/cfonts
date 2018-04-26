/***************************************************************************************************************************************************************
 *
 * ParseArgs unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../../src/lib.js');
const ParseArgs = CFonts.__test__.ParseArgs;


test(`ParseArgs - Return defaults without arguments`, () => {
	const options = {
		'--1': {
			description: 'desc value 1',
			short: '-1',
			options: true,
			default: '2',
		},
		'--2': {
			description: 'desc value 2',
			short: '-2',
			options: ['one', 'two', 'three'],
			default: 'one',
		},
		'--3': {
			description: 'desc value 3',
			short: '-3',
			default: false,
		},
	};

	const result = {
		text: undefined,
		1: '2',
		2: 'one',
		3: false,
	};

	expect( ParseArgs( options, [ 'node', 'script' ] ) ).toEqual( result );
	expect( ParseArgs( options, [] ) ).toEqual( result );
});


test(`ParseArgs - Parse out variables no matter the order`, () => {
	const options = {
		'--1': {
			description: 'desc value 1',
			short: '-1',
			options: true,
			default: '2',
		},
		'--2': {
			description: 'desc value 2',
			short: '-2',
			options: ['one', 'two', 'three'],
			default: 'one',
		},
		'--3': {
			description: 'desc value 3',
			short: '-3',
			default: false,
		},
	};

	const result = {
		text: 'text',
		1: 'x',
		2: 'two',
		3: true,
	};

	expect( ParseArgs( options, [ 'node', 'script', 'text', '--1', 'x', '--2', 'two', '--3' ] ) ).toEqual( result );
	expect( ParseArgs( options, [ 'node', 'script', 'text', '--3', '--1', 'x', '--2', 'two' ] ) ).toEqual( result );
	expect( ParseArgs( options, [ 'node', 'script', 'text', '--2', 'two', '--1', 'x', '--3' ] ) ).toEqual( result );
});


test(`ParseArgs - Ignore unknown options`, () => {
	const options = {
		'--1': {
			description: 'desc value 1',
			short: '-1',
			options: true,
			default: '2',
		},
		'--2': {
			description: 'desc value 2',
			short: '-2',
			options: ['one', 'two', 'three'],
			default: 'one',
		},
		'--3': {
			description: 'desc value 3',
			short: '-3',
			default: false,
		},
	};

	const result = {
		text: 'text',
		1: 'x',
		2: 'one',
		3: false,
	};

	expect( ParseArgs( options, [ 'node', 'script', 'text', '--1', 'x', 'unknown' ] ) ).toEqual( result );
	expect( ParseArgs( options, [ 'node', 'script', 'text', '--1', 'x', '--2', 'unknown' ] ) ).toEqual( result );
});


test(`ParseArgs - Help flag can be on text place`, () => {
	const options = {
		'--1': {
			description: 'desc value 1',
			short: '-1',
			options: true,
			default: '2',
		},
		'--2': {
			description: 'desc value 2',
			short: '-2',
			options: ['one', 'two', 'three'],
			default: 'one',
		},
		'--3': {
			description: 'desc value 3',
			short: '-3',
			default: false,
		},
	};

	const result1 = {
		text: '--help',
		1: '2',
		2: 'one',
		3: false,
		help: true,
	};

	const result2 = {
		text: '-h',
		1: '2',
		2: 'one',
		3: false,
		help: true,
	};

	expect( ParseArgs( options, [ 'node', 'script', '--help' ] ) ).toEqual( result1 );
	expect( ParseArgs( options, [ 'node', 'script', '-h' ] ) ).toEqual( result2 );
});


test(`ParseArgs - Version flag can be on text place`, () => {
	const options = {
		'--1': {
			description: 'desc value 1',
			short: '-1',
			options: true,
			default: '2',
		},
		'--2': {
			description: 'desc value 2',
			short: '-2',
			options: ['one', 'two', 'three'],
			default: 'one',
		},
		'--3': {
			description: 'desc value 3',
			short: '-3',
			default: false,
		},
	};

	const result1 = {
		text: '--version',
		1: '2',
		2: 'one',
		3: false,
		version: true,
	};

	const result2 = {
		text: '-v',
		1: '2',
		2: 'one',
		3: false,
		version: true,
	};

	expect( ParseArgs( options, [ 'node', 'script', '--version' ] ) ).toEqual( result1 );
	expect( ParseArgs( options, [ 'node', 'script', '-v' ] ) ).toEqual( result2 );
});
