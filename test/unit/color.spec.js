/***************************************************************************************************************************************************************
 *
 * Color unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../../src/lib.js');
const Color = CFonts.__test__.Color;


test(`Color - Return right object for colors`, () => {
	const output1 = {
		open: '\u001b[38;2;0;0;0m',
		close: '\u001b[39m',
	};

	const output2 = {
		open: '\u001b[38;2;255;255;0m',
		close: '\u001b[39m',
	};

	const output3 = {
		open: '\u001b[38;2;0;255;255m',
		close: '\u001b[39m',
	};

	expect( Color( 'black' ) ).toEqual( output1 );
	expect( Color( 'yellow' ) ).toEqual( output2 );
	expect( Color( 'cyan' ) ).toEqual( output3 );
});


test(`Color - Return empty object for system color`, () => {
	const output = {
		open: '',
		close: '',
	};

	expect( Color( 'system' ) ).toEqual( output );
});


test(`Color - Return right object for chalk colors`, () => {
	const outputRed = {
		open: '\u001b[91m',
		close: '\u001b[39m',
	};

	const outputMagenta = {
		open: '\u001b[95m',
		close: '\u001b[39m',
	};

	const outputBlue = {
		open: '\u001b[94m',
		close: '\u001b[39m',
	};

	expect( Color( 'redBright' ) ).toEqual( outputRed );
	expect( Color( 'magentaBright' ) ).toEqual( outputMagenta );
	expect( Color( 'blueBright' ) ).toEqual( outputBlue );
});


test(`Color - Return right object for hex colors`, () => {
	const output1 = {
		open: '\u001b[38;2;255;136;0m',
		close: '\u001b[39m',
	};

	const output2 = {
		open: '\u001b[38;2;0;136;255m',
		close: '\u001b[39m',
	};

	expect( Color( '#ff8800' ) ).toEqual( output1 );
	expect( Color( '#0088ff' ) ).toEqual( output2 );
});


test(`Color - Return right object for background colors`, () => {
	const output1 = {
		open: '\u001b[48;2;0;255;255m',
		close: '\u001b[49m',
	};

	const output2 = {
		open: '\u001b[103m',
		close: '\u001b[49m',
	};

	const output3 = {
		open: '\u001b[48;2;255;255;255m',
		close: '\u001b[49m',
	};

	expect( Color( 'cyan', true ) ).toEqual( output1 );
	expect( Color( 'yellowBright', true ) ).toEqual( output2 );
	expect( Color( 'white', true ) ).toEqual( output3 );
});


test(`Color - Return empty object for unknown colors`, () => {
	const output = {
		open: '',
		close: '',
	};

	expect( Color( 'null' ) ).toEqual( output );
	expect( Color( 'doesNotExists' ) ).toEqual( output );
	expect( Color( 'nada' ) ).toEqual( output );
});


test(`Color - Return empty object when FORCE_COLOR env var is set to 0`, () => {
	const output = {
		open: '',
		close: '',
	};

	process.env.FORCE_COLOR = 0;
	expect( Color( 'blue' ) ).toEqual( output );
	expect( Color( 'redBright' ) ).toEqual( output );
	expect( Color( 'yellow', true ) ).toEqual( output );
	process.env.FORCE_COLOR = 3;
});
