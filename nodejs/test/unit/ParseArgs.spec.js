/***************************************************************************************************************************************************************
 *
 * ParseArgs unit tests
 *
 **************************************************************************************************************************************************************/

const { CLIOPTIONS } = require('../../src/constants.js');
const { ParseArgs } = require('../../src/ParseArgs.js');

test(`ParseArgs - Return defaults without arguments`, () => {
	const options = {
		'--version': {
			_name: 'version',
			short: '-v',
			fallback_shortcut: '-V',
		},
		'--help': {
			_name: 'help',
			short: '-h',
			fallback_shortcut: false,
		},
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

	const defaultResult = {
		align: 'left',
		background: 'transparent',
		colors: 'system',
		debug: false,
		'debug-level': 1,
		font: 'block',
		gradient: false,
		'independent-gradient': false,
		'transition-gradient': false,
		help: false,
		'letter-spacing': undefined,
		'line-height': undefined,
		'max-length': 0,
		spaceless: false,
		text: undefined,
		version: false,
		env: 'node',
	};

	process.argv = ['node', 'script']; // we have to remove process.argv so args passed to our testing lib don't break our test

	expect(ParseArgs(options, ['node', 'script'])).toEqual(result);
	expect(ParseArgs(options, [])).toEqual(result);
	expect(ParseArgs(options)).toEqual(result);
	expect(ParseArgs()).toEqual(defaultResult);
});

test(`ParseArgs - Parse out variables no matter the order`, () => {
	const options = {
		'--version': {
			_name: 'version',
			short: '-v',
			fallback_shortcut: '-V',
		},
		'--help': {
			_name: 'help',
			short: '-h',
			fallback_shortcut: false,
		},
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

	expect(ParseArgs(options, ['node', 'script', 'text', '--1', 'x', '--2', 'two', '--3'])).toEqual(result);
	expect(ParseArgs(options, ['node', 'script', 'text', '--3', '--1', 'x', '--2', 'two'])).toEqual(result);
	expect(ParseArgs(options, ['node', 'script', 'text', '--2', 'two', '--1', 'x', '--3'])).toEqual(result);
});

test(`ParseArgs - Ignore flags not in the options`, () => {
	const options = {
		'--version': {
			_name: 'version',
			short: '-v',
			fallback_shortcut: '-V',
		},
		'--help': {
			_name: 'help',
			short: '-h',
			fallback_shortcut: false,
		},
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
		3: true,
	};

	expect(ParseArgs(options, ['node', 'script', 'text', '--1', 'x', '--4', 'two', '--3'])).toEqual(result);
});

test(`ParseArgs - Help flag can be on text place`, () => {
	const options = {
		'--version': {
			_name: 'version',
			short: '-v',
			fallback_shortcut: '-V',
		},
		'--help': {
			_name: 'help',
			short: '-h',
			fallback_shortcut: false,
		},
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

	expect(ParseArgs(options, ['node', 'script', '--help'])).toEqual(result1);
	expect(ParseArgs(options, ['node', 'script', '-h'])).toEqual(result2);
});

test(`ParseArgs - Version flag can be on text place`, () => {
	const options = {
		'--version': {
			_name: 'version',
			short: '-v',
			fallback_shortcut: '-V',
		},
		'--help': {
			_name: 'help',
			short: '-h',
			fallback_shortcut: false,
		},
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

	expect(ParseArgs(options, ['node', 'script', '--version'])).toEqual(result1);
	expect(ParseArgs(options, ['node', 'script', '-v'])).toEqual(result2);
});
