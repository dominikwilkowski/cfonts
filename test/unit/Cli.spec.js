/***************************************************************************************************************************************************************
 *
 * CLI unit tests
 *
 **************************************************************************************************************************************************************/


const { CLIOPTIONS } = require('../../src/constants.js');
const { Cli } = require('../../src/index.js');
const PKG = require('../../package.json');


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
	'--colors': {
		description: 'desc colors',
		short: '-c',
		options: true,
		default: 'system',
	},
};


test(`CLI - Calling Cli alone should output an error`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli();

	expect( console.log.mock.calls.length ).toBe( 0 );
	expect( console.error.mock.calls.length ).toBe( 1 );
});


test(`CLI - Text should be outputted`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script', 'text' ] );

	expect( console.log.mock.calls.length > 0 ).toBe( true );
	expect( console.error.mock.calls.length === 0 ).toBe( true );
});


test(`CLI - Text should be outputted even when options are not given completely`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script', 'text', '-c' ] );

	expect( console.log.mock.calls.length > 0 ).toBe( true );
	expect( console.error.mock.calls.length === 0 ).toBe( true );
});


test(`CLI - Help should be outputted`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script', '--help' ] );

	expect( console.log.mock.calls.length > 0 ).toBe( true );
	expect( console.error.mock.calls.length === 0 ).toBe( true );
});


test(`CLI - Version should be outputted`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script', '--version' ] );

	expect( console.log.mock.calls.length === 1 ).toBe( true );
	expect( console.error.mock.calls.length === 0 ).toBe( true );
});


test(`CLI - Should error out when no text has been given`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( true );
	expect( console.log.mock.calls.length === 0 ).toBe( true );
});


test(`CLI - Should log version`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '-v' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe( PKG.version );
});


test(`CLI - Should log the right two dots`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\n\n         \n' +
		'         \n' +
		'         \n' +
		'         \n' +
		' ██╗ ██╗ \n' +
		' ╚═╝ ╚═╝ \n\n'
	);
});


test(`CLI - Should log the right two dots with alignment center`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-a', 'center' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0].split('\n').length ).toBe( 10 );

	// '\n\n                                            \n' +
	// '                                            \n' +
	// '                                            \n' +
	// '                                            \n' +
	// '                                    ██╗ ██╗ \n' +
	// '                                    ╚═╝ ╚═╝ \n\n'
});


test(`CLI - Should log the right two dots in two colors`, () => {
	console.log = jest.fn();
	console.error = jest.fn();
	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-c', 'red,blue' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\n\n         \n' +
		'         \n' +
		'         \n' +
		'         \n' +
		' \u001b[38;2;255;0;0m██\u001b[39m\u001b[38;2;0;0;255m╗\u001b[39m \u001b[38;2;255;0;0m██\u001b[39m\u001b[38;2;0;0;255m╗\u001b[39m \n' +
		' \u001b[38;2;0;0;255m╚═╝\u001b[39m \u001b[38;2;0;0;255m╚═╝\u001b[39m' +
		' \n\n'
	);
});


test(`CLI - Should log the right two dots and background color`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-b', 'red' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\u001b[48;2;255;0;0m\n\n\n         \n' +
		'         \n' +
		'         \n' +
		'         \n' +
		' ██╗ ██╗ \n' +
		' ╚═╝ ╚═╝ \n\n\u001b[49m'
	);
});


test(`CLI - Should log the right two dots with letter spacing`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-l', '2' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\n\n            \n' +
		'            \n' +
		'            \n' +
		'            \n' +
		'  ██╗  ██╗  \n' +
		'  ╚═╝  ╚═╝  \n\n'
	);
});


test(`CLI - Should log the right two dots with lineheight`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '.|.', '-z', '3' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\n\n     \n' +
		'     \n' +
		'     \n' +
		'     \n' +
		' ██╗ \n' +
		' ╚═╝ \n' +
		'\n\n\n     \n' +
		'     \n' +
		'     \n' +
		'     \n' +
		' ██╗ \n' +
		' ╚═╝ \n\n'
	);
});


test(`CLI - Should log the right two dots without space`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-s' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'         \n' +
		'         \n' +
		'         \n' +
		'         \n' +
		' ██╗ ██╗ \n' +
		' ╚═╝ ╚═╝ '
	);
});


test(`CLI - Should log the right two dots with max-length set`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( CLIOPTIONS, [ 'node', 'script', '..', '-m', '1' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( false );
	expect( console.log.mock.calls[0][0] ).toBe(
		'\n\n     \n' +
		'     \n' +
		'     \n' +
		'     \n' +
		' ██╗ \n' +
		' ╚═╝ \n' +
		'\n     \n' +
		'     \n' +
		'     \n' +
		'     \n' +
		' ██╗ \n' +
		' ╚═╝ \n\n'
	);
});
