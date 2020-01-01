/***************************************************************************************************************************************************************
 *
 * Bin bindings tests
 *
 **************************************************************************************************************************************************************/


const StripColor = require('./_common.js').StripColor;
const Spawn = require('child_process').spawnSync;
const path = require('path');


test(`Bin - Will output with no flags`, () => {
	const output = Spawn(
		'node',
		[ path.normalize(`${__dirname }/../../src/bin.js`), 'abc' ],
		{
			encoding : 'utf8',
			env: { ...process.env },
		}
	);

	const expected = '' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗ \n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝ \n' +
		' ███████║ ██████╔╝ ██║      \n' +
		' ██╔══██║ ██╔══██╗ ██║      \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗ \n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝ \n' +
		'\n\n';

	expect( output.stdout ).toBe( expected );
	expect( output.stderr ).toBe( '' );
});


test(`Bin - Will output with color flag`, () => {
	const output = Spawn(
		'node',
		[ path.normalize(`${__dirname }/../../src/bin.js`), 'abc', '-c', 'red' ],
		{
			encoding : 'utf8',
			env: { ...process.env },
		}
	);

	const expected = '' +
		'\n\n' +
		'  \u001b[38;2;255;0;0m█████\u001b[39m╗  \u001b[38;2;255;0;0m██████\u001b[39m╗   \u001b[38;2;255;0;0m██████\u001b[39m╗ \n' +
		' \u001b[38;2;255;0;0m██\u001b[39m╔══\u001b[38;2;255;0;0m██\u001b[39m╗ \u001b[38;2;255;0;0m██\u001b[39m╔══\u001b[38;2;255;0;0m██\u001b[39m╗ \u001b[38;2;255;0;0m██\u001b[39m╔════╝ \n' +
		' \u001b[38;2;255;0;0m███████\u001b[39m║ \u001b[38;2;255;0;0m██████\u001b[39m╔╝ \u001b[38;2;255;0;0m██\u001b[39m║      \n' +
		' \u001b[38;2;255;0;0m██\u001b[39m╔══\u001b[38;2;255;0;0m██\u001b[39m║ \u001b[38;2;255;0;0m██\u001b[39m╔══\u001b[38;2;255;0;0m██\u001b[39m╗ \u001b[38;2;255;0;0m██\u001b[39m║      \n' +
		' \u001b[38;2;255;0;0m██\u001b[39m║\u001b[38;2;255;0;0m  ██\u001b[39m║ \u001b[38;2;255;0;0m██████\u001b[39m╔╝ ╚\u001b[38;2;255;0;0m██████\u001b[39m╗ \n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝ \n' +
		'\n\n';

	const expectedWithoutColor = '' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗ \n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝ \n' +
		' ███████║ ██████╔╝ ██║      \n' +
		' ██╔══██║ ██╔══██╗ ██║      \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗ \n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝ \n' +
		'\n\n';

	expect( output.stdout ).toBe( expected );
	expect( StripColor( output.stdout ) ).toBe( expectedWithoutColor );
	expect( output.stderr ).toBe( '' );
});


test(`Bin - Will output with hex color flag`, () => {
	const output = Spawn(
		'node',
		[ path.normalize(`${__dirname }/../../src/bin.js`), 'abc', '-c', '#ff8800', '#0088ff' ],
		{
			encoding : 'utf8',
			env: { ...process.env },
		}
	);

	const expected = '' +
		'\n\n' +
		'  \u001b[38;2;255;136;0m█████\u001b[39m╗  \u001b[38;2;255;136;0m██████\u001b[39m╗   \u001b[38;2;255;136;0m██████\u001b[39m╗ \n' +
		' \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m╔════╝ \n' +
		' \u001b[38;2;255;136;0m███████\u001b[39m║ \u001b[38;2;255;136;0m██████\u001b[39m╔╝ \u001b[38;2;255;136;0m██\u001b[39m║      \n' +
		' \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m║ \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m║      \n' +
		' \u001b[38;2;255;136;0m██\u001b[39m║\u001b[38;2;255;136;0m  ██\u001b[39m║ \u001b[38;2;255;136;0m██████\u001b[39m╔╝ ╚\u001b[38;2;255;136;0m██████\u001b[39m╗ \n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝ \n' +
		'\n\n';

	const expectedWithoutColor = '' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗ \n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝ \n' +
		' ███████║ ██████╔╝ ██║      \n' +
		' ██╔══██║ ██╔══██╗ ██║      \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗ \n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝ \n' +
		'\n\n';

	expect( output.stdout ).toBe( expected );
	expect( StripColor( output.stdout ) ).toBe( expectedWithoutColor );
	expect( output.stderr ).toBe( '' );
});
