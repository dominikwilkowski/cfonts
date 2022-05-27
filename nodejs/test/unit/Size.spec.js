/***************************************************************************************************************************************************************
 *
 * Size unit tests
 *
 **************************************************************************************************************************************************************/

beforeEach(() => {
	jest.resetModules();
});

test(`Size - Should have two keys with numbers`, () => {
	jest.doMock('window-size', () => ({ width: 200, height: 200 }));
	const WinSize = require('window-size');

	const { Size } = require('../../src/Size.js');

	expect(typeof Size.width).toEqual('number');
	expect(typeof Size.height).toEqual('number');
});

test(`Size - Should have only sizes that are larger than 0`, () => {
	jest.doMock('window-size', () => ({ width: 200, height: 200 }));
	const WinSize = require('window-size');

	const { Size } = require('../../src/Size.js');

	expect(Size.width > 0).toBe(true);
	expect(Size.height > 0).toBe(true);
});

test(`Size - Should fallback to default sizes where window-size returns 0 size`, () => {
	jest.doMock('window-size', () => ({ width: 0, height: 0 }));
	const WinSize = require('window-size');

	const { Size } = require('../../src/Size.js');

	expect(Size.width).toBe(80);
	expect(Size.height).toBe(24);
});

test(`Size - Should fallback to default sizes where window-size returns nothing`, () => {
	jest.doMock('window-size', () => false);
	const WinSize = require('window-size');

	const { Size } = require('../../src/Size.js');

	expect(Size.width).toBe(80);
	expect(Size.height).toBe(24);
});
