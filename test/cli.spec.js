/***************************************************************************************************************************************************************
 *
 * CLI unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js');
const Cli = CFonts.__test__.Cli;


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


test(`CLI - Text should be outputted`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script', 'text' ] );

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


test(`CLI - Version should be outputted`, () => {
	console.log = jest.fn();
	console.error = jest.fn();

	Cli( options, [ 'node', 'script' ] );

	expect( console.error.mock.calls.length > 0 ).toBe( true );
	expect( console.log.mock.calls.length === 0 ).toBe( true );
});
