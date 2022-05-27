/***************************************************************************************************************************************************************
 *
 * Bin bindings tests
 *
 **************************************************************************************************************************************************************/

const StripColor = require('./_common.js').StripColor;
const Spawn = require('child_process').spawnSync;
const path = require('path');

beforeEach(() => {
	jest.resetModules();
});

test(`Bin - Should call Cli function`, () => {
	jest.doMock('../../lib/index.js', () => ({ Cli: jest.fn() }));
	const Cfonts = require('../../lib/index.js');

	require('../../src/bin.js');

	expect(Cfonts.Cli.mock.calls.length).toBe(1);
});

test(`Bin - Will output with no flags`, () => {
	const output = Spawn('node', [path.normalize(`${__dirname}/../../src/bin.js`), 'abc'], {
		encoding: 'utf8',
		env: { ...process.env },
	});

	const expected =
		'' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗\n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝\n' +
		' ███████║ ██████╔╝ ██║     \n' +
		' ██╔══██║ ██╔══██╗ ██║     \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗\n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝\n' +
		'\n\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will output with color flag`, () => {
	const output = Spawn('node', [path.normalize(`${__dirname}/../../src/bin.js`), 'abc', '-c', 'red'], {
		encoding: 'utf8',
		env: { ...process.env },
	});

	const expected =
		'' +
		'\n\n' +
		'  \u001b[31m█████\u001b[39m╗  \u001b[31m██████\u001b[39m╗   \u001b[31m██████\u001b[39m╗\n' +
		' \u001b[31m██\u001b[39m╔══\u001b[31m██\u001b[39m╗ \u001b[31m██\u001b[39m╔══\u001b[31m██\u001b[39m╗ \u001b[31m██\u001b[39m╔════╝\n' +
		' \u001b[31m███████\u001b[39m║ \u001b[31m██████\u001b[39m╔╝ \u001b[31m██\u001b[39m║     \n' +
		' \u001b[31m██\u001b[39m╔══\u001b[31m██\u001b[39m║ \u001b[31m██\u001b[39m╔══\u001b[31m██\u001b[39m╗ \u001b[31m██\u001b[39m║     \n' +
		' \u001b[31m██\u001b[39m║\u001b[31m  ██\u001b[39m║ \u001b[31m██████\u001b[39m╔╝ ╚\u001b[31m██████\u001b[39m╗\n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝\n' +
		'\n\n';

	const expectedWithoutColor =
		'' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗\n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝\n' +
		' ███████║ ██████╔╝ ██║     \n' +
		' ██╔══██║ ██╔══██╗ ██║     \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗\n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝\n' +
		'\n\n';

	expect(output.stdout).toBe(expected);
	expect(StripColor(output.stdout)).toBe(expectedWithoutColor);
	expect(output.stderr).toBe('');
});

test(`Bin - Will output with hex color flag`, () => {
	const output = Spawn('node', [path.normalize(`${__dirname}/../../src/bin.js`), 'abc', '-c', '#ff8800', '#0088ff'], {
		encoding: 'utf8',
		env: { ...process.env },
	});

	const expected =
		'' +
		'\n\n' +
		'  \u001b[38;2;255;136;0m█████\u001b[39m╗  \u001b[38;2;255;136;0m██████\u001b[39m╗   \u001b[38;2;255;136;0m██████\u001b[39m╗\n' +
		' \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m╔════╝\n' +
		' \u001b[38;2;255;136;0m███████\u001b[39m║ \u001b[38;2;255;136;0m██████\u001b[39m╔╝ \u001b[38;2;255;136;0m██\u001b[39m║     \n' +
		' \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m║ \u001b[38;2;255;136;0m██\u001b[39m╔══\u001b[38;2;255;136;0m██\u001b[39m╗ \u001b[38;2;255;136;0m██\u001b[39m║     \n' +
		' \u001b[38;2;255;136;0m██\u001b[39m║\u001b[38;2;255;136;0m  ██\u001b[39m║ \u001b[38;2;255;136;0m██████\u001b[39m╔╝ ╚\u001b[38;2;255;136;0m██████\u001b[39m╗\n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝\n' +
		'\n\n';

	const expectedWithoutColor =
		'' +
		'\n\n' +
		'  █████╗  ██████╗   ██████╗\n' +
		' ██╔══██╗ ██╔══██╗ ██╔════╝\n' +
		' ███████║ ██████╔╝ ██║     \n' +
		' ██╔══██║ ██╔══██╗ ██║     \n' +
		' ██║  ██║ ██████╔╝ ╚██████╗\n' +
		' ╚═╝  ╚═╝ ╚═════╝   ╚═════╝\n' +
		'\n\n';

	expect(output.stdout).toBe(expected);
	expect(StripColor(output.stdout)).toBe(expectedWithoutColor);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all non-gradient flags with console`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|xxx',
			'-f',
			'console',
			'-a',
			'right',
			'-c',
			'red',
			'-b',
			'white',
			'-l',
			'3',
			'-z',
			'4',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[47m\n' +
		'                                                                             \x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31ma\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31mb\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31mc\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                             \x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31mx\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31mx\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31m\x1B[39m\x1B[31mx\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags and gradient with console`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|xxx',
			'-f',
			'console',
			'-a',
			'right',
			'-g',
			'red,blue',
			'-b',
			'white',
			'-l',
			'3',
			'-z',
			'4',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[47m\n' +
		'                                                                                \x1B[38;2;255;0;0ma\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;127;255;0m \x1B[39m\x1B[38;2;0;255;0mb\x1B[39m\x1B[38;2;0;255;127m \x1B[39m\x1B[38;2;0;255;255m \x1B[39m\x1B[38;2;0;127;255m \x1B[39m\x1B[38;2;0;0;255mc\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                                \x1B[38;2;255;0;0mx\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;127;255;0m \x1B[39m\x1B[38;2;0;255;0mx\x1B[39m\x1B[38;2;0;255;127m \x1B[39m\x1B[38;2;0;255;255m \x1B[39m\x1B[38;2;0;127;255m \x1B[39m\x1B[38;2;0;0;255mx\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags, gradient and i flag with console`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|xx',
			'-f',
			'console',
			'-a',
			'right',
			'-g',
			'red,blue',
			'-i',
			'-b',
			'white',
			'-l',
			'3',
			'-z',
			'4',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[47m\n' +
		'                                                                                \x1B[38;2;255;0;0ma\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;127;255;0m \x1B[39m\x1B[38;2;0;255;0mb\x1B[39m\x1B[38;2;0;255;127m \x1B[39m\x1B[38;2;0;255;255m \x1B[39m\x1B[38;2;0;127;255m \x1B[39m\x1B[38;2;0;0;255mc\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                                 \x1B[38;2;255;0;0mx\x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;0;255;0m \x1B[39m\x1B[38;2;0;255;255m \x1B[39m\x1B[38;2;0;0;255mx\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags, gradient, i flag and transition with console`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|xx',
			'-f',
			'console',
			'-a',
			'right',
			'-g',
			'red,blue',
			'-i',
			'-t',
			'-b',
			'white',
			'-l',
			'3',
			'-z',
			'4',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[47m\n' +
		'                                                                                \x1B[38;2;255;0;0ma\x1B[39m\x1B[38;2;223;0;31m \x1B[39m\x1B[38;2;191;0;63m \x1B[39m\x1B[38;2;159;0;95m \x1B[39m\x1B[38;2;127;0;127mb\x1B[39m\x1B[38;2;95;0;159m \x1B[39m\x1B[38;2;63;0;191m \x1B[39m\x1B[38;2;31;0;223m \x1B[39m\x1B[38;2;0;0;255mc\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                                 \x1B[38;2;255;0;0mx\x1B[39m\x1B[38;2;191;0;63m \x1B[39m\x1B[38;2;127;0;127m \x1B[39m\x1B[38;2;63;0;191m \x1B[39m\x1B[38;2;0;0;255mx\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all non-gradient flags with chrome font`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|aa',
			'-f',
			'chrome',
			'-a',
			'right',
			'-c',
			'blue,green,yellow',
			'-b',
			'red',
			'-l',
			'4',
			'-z',
			'3',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[41m\n' +
		'                                                               \x1B[34m╔═╗\x1B[39m    \x1B[34m╔╗ \x1B[39m    \x1B[34m╔═╗\x1B[39m\n' +
		'                                                               \x1B[32m╠═╣\x1B[39m    \x1B[32m╠╩╗\x1B[39m    \x1B[32m║  \x1B[39m\n' +
		'                                                               \x1B[33m╩ ╩\x1B[39m    \x1B[33m╚═╝\x1B[39m    \x1B[33m╚═╝\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                      \x1B[34m╔═╗\x1B[39m    \x1B[34m╔═╗\x1B[39m\n' +
		'                                                                      \x1B[32m╠═╣\x1B[39m    \x1B[32m╠═╣\x1B[39m\n' +
		'                                                                      \x1B[33m╩ ╩\x1B[39m    \x1B[33m╩ ╩\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags and gradient with chrome font`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|aa',
			'-f',
			'chrome',
			'-a',
			'right',
			'-g',
			'blue,green',
			'-b',
			'red',
			'-l',
			'4',
			'-z',
			'3',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[41m\n' +
		'                                                               \x1B[38;2;0;0;255m╔\x1B[39m\x1B[38;2;63;0;255m═\x1B[39m\x1B[38;2;127;0;255m╗\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╔\x1B[39m\x1B[38;2;255;0;0m╗\x1B[39m\x1B[38;2;255;63;0m \x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╔\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╠\x1B[39m\x1B[38;2;63;0;255m═\x1B[39m\x1B[38;2;127;0;255m╣\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╠\x1B[39m\x1B[38;2;255;0;0m╩\x1B[39m\x1B[38;2;255;63;0m╗\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m║\x1B[39m\x1B[38;2;63;255;0m \x1B[39m\x1B[38;2;0;255;0m \x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╩\x1B[39m\x1B[38;2;63;0;255m \x1B[39m\x1B[38;2;127;0;255m╩\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╚\x1B[39m\x1B[38;2;255;0;0m═\x1B[39m\x1B[38;2;255;63;0m╝\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╚\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╝\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                               \x1B[38;2;0;0;255m \x1B[39m\x1B[38;2;63;0;255m \x1B[39m\x1B[38;2;127;0;255m \x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╔\x1B[39m\x1B[38;2;255;0;0m═\x1B[39m\x1B[38;2;255;63;0m╗\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╔\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m \x1B[39m\x1B[38;2;63;0;255m \x1B[39m\x1B[38;2;127;0;255m \x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╠\x1B[39m\x1B[38;2;255;0;0m═\x1B[39m\x1B[38;2;255;63;0m╣\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╠\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╣\x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m \x1B[39m\x1B[38;2;63;0;255m \x1B[39m\x1B[38;2;127;0;255m \x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╩\x1B[39m\x1B[38;2;255;0;0m \x1B[39m\x1B[38;2;255;63;0m╩\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╩\x1B[39m\x1B[38;2;63;255;0m \x1B[39m\x1B[38;2;0;255;0m╩\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags, gradient and i flag with chrome font`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|aa',
			'-f',
			'chrome',
			'-a',
			'right',
			'-g',
			'blue,green',
			'-i',
			'-b',
			'red',
			'-l',
			'4',
			'-z',
			'3',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'' +
		'\x1B[41m\n' +
		'                                                               \x1B[38;2;0;0;255m╔\x1B[39m\x1B[38;2;63;0;255m═\x1B[39m\x1B[38;2;127;0;255m╗\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╔\x1B[39m\x1B[38;2;255;0;0m╗\x1B[39m\x1B[38;2;255;63;0m \x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╔\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╠\x1B[39m\x1B[38;2;63;0;255m═\x1B[39m\x1B[38;2;127;0;255m╣\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╠\x1B[39m\x1B[38;2;255;0;0m╩\x1B[39m\x1B[38;2;255;63;0m╗\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m║\x1B[39m\x1B[38;2;63;255;0m \x1B[39m\x1B[38;2;0;255;0m \x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╩\x1B[39m\x1B[38;2;63;0;255m \x1B[39m\x1B[38;2;127;0;255m╩\x1B[39m\x1B[38;2;191;0;255m \x1B[39m\x1B[38;2;255;0;255m \x1B[39m\x1B[38;2;255;0;191m \x1B[39m\x1B[38;2;255;0;127m \x1B[39m\x1B[38;2;255;0;63m╚\x1B[39m\x1B[38;2;255;0;0m═\x1B[39m\x1B[38;2;255;63;0m╝\x1B[39m\x1B[38;2;255;127;0m \x1B[39m\x1B[38;2;255;191;0m \x1B[39m\x1B[38;2;255;255;0m \x1B[39m\x1B[38;2;191;255;0m \x1B[39m\x1B[38;2;127;255;0m╚\x1B[39m\x1B[38;2;63;255;0m═\x1B[39m\x1B[38;2;0;255;0m╝\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                      \x1B[38;2;0;0;255m╔\x1B[39m\x1B[38;2;113;0;255m═\x1B[39m\x1B[38;2;226;0;255m╗\x1B[39m\x1B[38;2;255;0;170m \x1B[39m\x1B[38;2;255;0;56m \x1B[39m\x1B[38;2;255;56;0m \x1B[39m\x1B[38;2;255;170;0m \x1B[39m\x1B[38;2;226;255;0m╔\x1B[39m\x1B[38;2;113;255;0m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                                      \x1B[38;2;0;0;255m╠\x1B[39m\x1B[38;2;113;0;255m═\x1B[39m\x1B[38;2;226;0;255m╣\x1B[39m\x1B[38;2;255;0;170m \x1B[39m\x1B[38;2;255;0;56m \x1B[39m\x1B[38;2;255;56;0m \x1B[39m\x1B[38;2;255;170;0m \x1B[39m\x1B[38;2;226;255;0m╠\x1B[39m\x1B[38;2;113;255;0m═\x1B[39m\x1B[38;2;0;255;0m╣\x1B[39m\n' +
		'                                                                      \x1B[38;2;0;0;255m╩\x1B[39m\x1B[38;2;113;0;255m \x1B[39m\x1B[38;2;226;0;255m╩\x1B[39m\x1B[38;2;255;0;170m \x1B[39m\x1B[38;2;255;0;56m \x1B[39m\x1B[38;2;255;56;0m \x1B[39m\x1B[38;2;255;170;0m \x1B[39m\x1B[38;2;226;255;0m╩\x1B[39m\x1B[38;2;113;255;0m \x1B[39m\x1B[38;2;0;255;0m╩\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});

test(`Bin - Will run all flags, gradient, i flag and transition with chrome font`, () => {
	const output = Spawn(
		'node',
		[
			path.normalize(`${__dirname}/../../src/bin.js`),
			'abc|aa',
			'-f',
			'chrome',
			'-a',
			'right',
			'-g',
			'blue,green',
			'-i',
			'-t',
			'-b',
			'red',
			'-l',
			'4',
			'-z',
			'3',
			'-s',
			'-m',
			'80',
		],
		{
			encoding: 'utf8',
			env: { ...process.env },
		}
	);

	const expected =
		'\x1B[41m\n' +
		'                                                               \x1B[38;2;0;0;255m╔\x1B[39m\x1B[38;2;0;15;239m═\x1B[39m\x1B[38;2;0;31;223m╗\x1B[39m\x1B[38;2;0;47;207m \x1B[39m\x1B[38;2;0;63;191m \x1B[39m\x1B[38;2;0;79;175m \x1B[39m\x1B[38;2;0;95;159m \x1B[39m\x1B[38;2;0;111;143m╔\x1B[39m\x1B[38;2;0;127;127m╗\x1B[39m\x1B[38;2;0;143;111m \x1B[39m\x1B[38;2;0;159;95m \x1B[39m\x1B[38;2;0;175;79m \x1B[39m\x1B[38;2;0;191;63m \x1B[39m\x1B[38;2;0;207;47m \x1B[39m\x1B[38;2;0;223;31m╔\x1B[39m\x1B[38;2;0;239;15m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╠\x1B[39m\x1B[38;2;0;15;239m═\x1B[39m\x1B[38;2;0;31;223m╣\x1B[39m\x1B[38;2;0;47;207m \x1B[39m\x1B[38;2;0;63;191m \x1B[39m\x1B[38;2;0;79;175m \x1B[39m\x1B[38;2;0;95;159m \x1B[39m\x1B[38;2;0;111;143m╠\x1B[39m\x1B[38;2;0;127;127m╩\x1B[39m\x1B[38;2;0;143;111m╗\x1B[39m\x1B[38;2;0;159;95m \x1B[39m\x1B[38;2;0;175;79m \x1B[39m\x1B[38;2;0;191;63m \x1B[39m\x1B[38;2;0;207;47m \x1B[39m\x1B[38;2;0;223;31m║\x1B[39m\x1B[38;2;0;239;15m \x1B[39m\x1B[38;2;0;255;0m \x1B[39m\n' +
		'                                                               \x1B[38;2;0;0;255m╩\x1B[39m\x1B[38;2;0;15;239m \x1B[39m\x1B[38;2;0;31;223m╩\x1B[39m\x1B[38;2;0;47;207m \x1B[39m\x1B[38;2;0;63;191m \x1B[39m\x1B[38;2;0;79;175m \x1B[39m\x1B[38;2;0;95;159m \x1B[39m\x1B[38;2;0;111;143m╚\x1B[39m\x1B[38;2;0;127;127m═\x1B[39m\x1B[38;2;0;143;111m╝\x1B[39m\x1B[38;2;0;159;95m \x1B[39m\x1B[38;2;0;175;79m \x1B[39m\x1B[38;2;0;191;63m \x1B[39m\x1B[38;2;0;207;47m \x1B[39m\x1B[38;2;0;223;31m╚\x1B[39m\x1B[38;2;0;239;15m═\x1B[39m\x1B[38;2;0;255;0m╝\x1B[39m\n' +
		'\n' +
		'\n' +
		'\n' +
		'                                                                      \x1B[38;2;0;0;255m╔\x1B[39m\x1B[38;2;0;28;226m═\x1B[39m\x1B[38;2;0;56;198m╗\x1B[39m\x1B[38;2;0;85;170m \x1B[39m\x1B[38;2;0;113;141m \x1B[39m\x1B[38;2;0;141;113m \x1B[39m\x1B[38;2;0;170;85m \x1B[39m\x1B[38;2;0;198;56m╔\x1B[39m\x1B[38;2;0;226;28m═\x1B[39m\x1B[38;2;0;255;0m╗\x1B[39m\n' +
		'                                                                      \x1B[38;2;0;0;255m╠\x1B[39m\x1B[38;2;0;28;226m═\x1B[39m\x1B[38;2;0;56;198m╣\x1B[39m\x1B[38;2;0;85;170m \x1B[39m\x1B[38;2;0;113;141m \x1B[39m\x1B[38;2;0;141;113m \x1B[39m\x1B[38;2;0;170;85m \x1B[39m\x1B[38;2;0;198;56m╠\x1B[39m\x1B[38;2;0;226;28m═\x1B[39m\x1B[38;2;0;255;0m╣\x1B[39m\n' +
		'                                                                      \x1B[38;2;0;0;255m╩\x1B[39m\x1B[38;2;0;28;226m \x1B[39m\x1B[38;2;0;56;198m╩\x1B[39m\x1B[38;2;0;85;170m \x1B[39m\x1B[38;2;0;113;141m \x1B[39m\x1B[38;2;0;141;113m \x1B[39m\x1B[38;2;0;170;85m \x1B[39m\x1B[38;2;0;198;56m╩\x1B[39m\x1B[38;2;0;226;28m \x1B[39m\x1B[38;2;0;255;0m╩\x1B[39m\x1B[49m\n';

	expect(output.stdout).toBe(expected);
	expect(output.stderr).toBe('');
});
