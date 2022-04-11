/***************************************************************************************************************************************************************
 *
 * Chalk unit tests
 *
 **************************************************************************************************************************************************************/


const { StripColor } = require('./_common.js');


beforeEach(() => {
	jest.resetModules();
});

test(`Chalk - Chalk should be working on level 3 by default (in this test)`, () => {
	const { Chalk } = require('../../src/Chalk.js');

	expect( Chalk.green('x') ).toBe(`\u001b[32mx\u001b[39m`);
});

test(`Chalk - Chalk should take the environment variable into account`, () => {
	process.env.FORCE_COLOR = 0;
	const { Chalk } = require('../../src/Chalk.js');

	expect( Chalk.green('x') ).toBe(`x`);
});

test(`Chalk - Chalk should take the environment variable into account`, () => {
	delete process.env.FORCE_COLOR;
	const { Chalk } = require('../../src/Chalk.js');

	expect( StripColor( Chalk.green('x') ) ).toBe(`x`);
});
