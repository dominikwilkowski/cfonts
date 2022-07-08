/***************************************************************************************************************************************************************
 *
 * Color unit tests
 *
 **************************************************************************************************************************************************************/

const { Options } = require('../../src/Options.js');
const {
	Rgb2hsv,
	Hsv2rgb,
	Rgb2hex,
	Hex2rgb,
	Hsv2hsvRad,
	HsvRad2hsv,
	Hex2hsvRad,
	HsvRad2hex,
	rgb2ansi_16m,
	rgb2ansi256Code,
	rgb2ansi_256,
	ansi_2562ansi_16,
	get_term_color_support,
	Color,
} = require('../../src/Color.js');
const { inspect } = require('util');

beforeEach(() => {
	delete process.env.NO_COLOR;
	process.env.FORCE_COLOR = '3';
});

test(`Color - Rgb2hsv - Convert RGB to HSV correctly`, () => {
	// unit test each line
	expect(Rgb2hsv({ r: 0, g: 0, b: 0 })).toEqual([0, 0, 0]);
	expect(Rgb2hsv({ r: 155, g: 150, b: 100 })).toEqual([54.54545454545456, 35.48387096774193, 60.78431372549019]);
	expect(Rgb2hsv({ r: 166, g: 20, b: 100 })).toEqual([327.1232876712329, 87.95180722891565, 65.09803921568627]);
	expect(Rgb2hsv({ r: 250, g: 255, b: 245 })).toEqual([90.00000000000009, 3.9215686274509776, 100]);
	expect(Rgb2hsv({ r: 110, g: 100, b: 250 })).toEqual([244, 60, 98.0392156862745]);
	expect(Rgb2hsv({ r: 255, g: 0, b: 0 })).toEqual([0, 100, 100]);

	// some random tests
	expect(Rgb2hsv({ r: 114, g: 103, b: 130 })).toEqual([264.44444444444446, 20.769230769230763, 50.98039215686274]);
	expect(Rgb2hsv({ r: 255, g: 150, b: 0 })).toEqual([35.294117647058826, 100, 100]);
	expect(Rgb2hsv({ r: 5, g: 78, b: 55 })).toEqual([161.0958904109589, 93.58974358974359, 30.58823529411765]);
	expect(Rgb2hsv({ r: 35, g: 10, b: 170 })).toEqual([249.375, 94.11764705882352, 66.66666666666666]);
	expect(Rgb2hsv({ r: 1, g: 6, b: 3 })).toEqual([144, 83.33333333333334, 2.3529411764705883]);
	expect(Rgb2hsv({ r: 100, g: 100, b: 100 })).toEqual([0, 0, 39.21568627450981]);
	expect(Rgb2hsv({ r: 100, g: 20, b: 100 })).toEqual([300, 80, 39.21568627450981]);

	// reverse tests from Hsv2rgb
	expect(Rgb2hsv({ r: 0, g: 0, b: 0 })).toEqual([0, 0, 0]);
	expect(Rgb2hsv({ r: 51, g: 46, b: 41 })).toEqual([29.99999999999998, 19.607843137254903, 20]);
	expect(Rgb2hsv({ r: 48, g: 51, b: 41 })).toEqual([78.00000000000003, 19.607843137254903, 20]);
	expect(Rgb2hsv({ r: 41, g: 51, b: 41 })).toEqual([120, 19.607843137254903, 20]);
	expect(Rgb2hsv({ r: 41, g: 51, b: 51 })).toEqual([180, 19.607843137254903, 20]);
	expect(Rgb2hsv({ r: 41, g: 41, b: 51 })).toEqual([240, 19.607843137254903, 20]);
	expect(Rgb2hsv({ r: 51, g: 41, b: 51 })).toEqual([300, 19.607843137254903, 20]);
});

test(`Color - Hsv2rgb - Convert HSV to RGB correctly`, () => {
	// unit test each line
	expect(Hsv2rgb(0, 0, 0)).toEqual({ r: 0, g: 0, b: 0 });
	expect(Hsv2rgb(30, 20, 20)).toEqual({ r: 51, g: 45.9, b: 40.800000000000004 });
	expect(Hsv2rgb(80, 20, 20)).toEqual({ r: 47.6, g: 51, b: 40.800000000000004 });
	expect(Hsv2rgb(120, 20, 20)).toEqual({ r: 40.800000000000004, g: 51, b: 40.800000000000004 });
	expect(Hsv2rgb(180, 20, 20)).toEqual({ r: 40.800000000000004, g: 51, b: 51 });
	expect(Hsv2rgb(240, 20, 20)).toEqual({ r: 40.800000000000004, g: 40.800000000000004, b: 51 });
	expect(Hsv2rgb(300, 20, 20)).toEqual({ r: 51, g: 40.800000000000004, b: 51 });
	expect(Hsv2rgb(360, 100, 100)).toEqual({ r: 255, g: 0, b: 0 });

	// Reverse tests from Rgb2hsv
	expect(Hsv2rgb(0, 0, 0)).toEqual({ r: 0, g: 0, b: 0 });
	expect(Hsv2rgb(54.54545454545456, 35.48387096774193, 60.78431372549019)).toEqual({
		r: 155,
		g: 150.00000000000003,
		b: 100.00000000000001,
	});
	expect(Hsv2rgb(327.1232876712329, 87.95180722891565, 65.09803921568627)).toEqual({
		r: 165.99999999999997,
		g: 20.000000000000007,
		b: 99.99999999999991,
	});
	expect(Hsv2rgb(90.00000000000009, 3.9215686274509776, 100)).toEqual({ r: 250, g: 255, b: 245 });
	expect(Hsv2rgb(244, 60, 98.0392156862745)).toEqual({ r: 109.99999999999996, g: 100, b: 250 });
	expect(Hsv2rgb(264.44444444444446, 20.769230769230763, 50.98039215686274)).toEqual({
		r: 114,
		g: 103.00000000000001,
		b: 130,
	});
	expect(Hsv2rgb(35.294117647058826, 100, 100)).toEqual({ r: 255, g: 150, b: 0 });
	expect(Hsv2rgb(161.0958904109589, 93.58974358974359, 30.58823529411765)).toEqual({
		r: 5,
		g: 78,
		b: 55.00000000000001,
	});
	expect(Hsv2rgb(249.375, 94.11764705882352, 66.66666666666666)).toEqual({
		r: 35.00000000000002,
		g: 10.00000000000002,
		b: 169.99999999999997,
	});
	expect(Hsv2rgb(144, 83.33333333333334, 2.3529411764705883)).toEqual({
		r: 0.9999999999999991,
		g: 6,
		b: 2.999999999999999,
	});
	expect(Hsv2rgb(0, 0, 39.21568627450981)).toEqual({
		r: 100.00000000000001,
		g: 100.00000000000001,
		b: 100.00000000000001,
	});
	expect(Hsv2rgb(300, 80, 39.21568627450981)).toEqual({ r: 100.00000000000001, g: 20, b: 100.00000000000001 });
});

test(`Color - Rgb2hex - Convert RGB to HEX correctly`, () => {
	expect(Rgb2hex(0, 0, 0)).toBe('#000000');
	expect(Rgb2hex(255, 255, 255)).toBe('#ffffff');
	expect(Rgb2hex(0, 255, 255)).toBe('#00ffff');
	expect(Rgb2hex(255, 0, 255)).toBe('#ff00ff');
	expect(Rgb2hex(255, 255, 0)).toBe('#ffff00');
	expect(Rgb2hex(127, 127, 127)).toBe('#7f7f7f');
	expect(Rgb2hex(255, 136, 0)).toBe('#ff8800');
});

test(`Color - Hex2rgb - Convert HEX to RGB correctly`, () => {
	expect(Hex2rgb('#000000')).toEqual([0, 0, 0]);
	expect(Hex2rgb('#ffffff')).toEqual([255, 255, 255]);
	expect(Hex2rgb('#00ffff')).toEqual([0, 255, 255]);
	expect(Hex2rgb('#ff00ff')).toEqual([255, 0, 255]);
	expect(Hex2rgb('#ffff00')).toEqual([255, 255, 0]);
	expect(Hex2rgb('#ffffffff')).toEqual([255, 255, 255]);
	expect(Hex2rgb('#fff')).toEqual([255, 255, 255]);
	expect(Hex2rgb('#ffff')).toEqual([255, 255, 255]);
	expect(Hex2rgb('#7f7f7f')).toEqual([127, 127, 127]);
	expect(Hex2rgb('#ff8800')).toEqual([255, 136, 0]);
});

test(`Color - Hsv2hsvRad - Convert HSV to HSVrad correctly`, () => {
	expect(Hsv2hsvRad([0, 0, 0])).toEqual([0, 0, 0]);
	expect(Hsv2hsvRad([360, 0, 0])).toEqual([6.283185307179586, 0, 0]);
	expect(Hsv2hsvRad([180, 0, 0])).toEqual([3.141592653589793, 0, 0]);
});

test(`Color - HsvRad2hsv - Convert HSVrad to HSV correctly`, () => {
	expect(HsvRad2hsv(0, 0, 0)).toEqual([0, 0, 0]);
	expect(HsvRad2hsv(6.283185307179586, 0, 0)).toEqual([360, 0, 0]);
	expect(HsvRad2hsv(3.141592653589793, 0, 0)).toEqual([180, 0, 0]);
});

test(`Color - Hex2hsvRad - Convert HEX to HSVrad correctly`, () => {
	expect(Hex2hsvRad('#000000')).toEqual([0, 0, 0]);
	expect(Hex2hsvRad('#ffffff')).toEqual([0, 0, 100]);
	expect(Hex2hsvRad('#00ffff')).toEqual([3.141592653589793, 100, 100]);
	expect(Hex2hsvRad('#ff00ff')).toEqual([5.235987755982989, 100, 100]);
	expect(Hex2hsvRad('#ffff00')).toEqual([1.0471975511965976, 100, 100]);
});

test(`Color - HsvRad2hex - Convert HSVrad to HEX correctly`, () => {
	expect(HsvRad2hex(0, 0, 0)).toEqual('#000000');
	expect(HsvRad2hex(0, 0, 100)).toEqual('#ffffff');
	expect(HsvRad2hex(3.141592653589793, 100, 100)).toEqual('#00ffff');
	expect(HsvRad2hex(5.235987755982989, 100, 100)).toEqual('#ff00ff');
	expect(HsvRad2hex(1.0471975511965976, 100, 100)).toEqual('#ffff00');
});

test(`Color - rgb2ansi_16m - Convert RGB to ANSI16million correctly`, () => {
	expect(inspect(rgb2ansi_16m(0, 0, 0))).toEqual(inspect('\x1B[38;2;0;0;0m'));
	expect(inspect(rgb2ansi_16m(255, 255, 255))).toEqual(inspect('\x1B[38;2;255;255;255m'));
	expect(inspect(rgb2ansi_16m(0, 0, 0, true))).toEqual(inspect('\x1B[48;2;0;0;0m'));
	expect(inspect(rgb2ansi_16m(255, 255, 255, true))).toEqual(inspect('\x1B[48;2;255;255;255m'));
});

test(`Color - rgb2ansi256Code - Convert RGB values to ANSI256 escape code correctly`, () => {
	expect(rgb2ansi256Code(0, 0, 0)).toEqual(16);
	expect(rgb2ansi256Code(200, 200, 200)).toEqual(251);
	expect(rgb2ansi256Code(255, 255, 255)).toEqual(231);
	expect(rgb2ansi256Code(100, 50, 150)).toEqual(97);
});

test(`Color - rgb2ansi_256 - Convert RGB to ANSI256 correctly`, () => {
	expect(inspect(rgb2ansi_256(0, 0, 0))).toEqual(inspect('\x1B[38;5;16m'));
	expect(inspect(rgb2ansi_256(255, 255, 255))).toEqual(inspect('\x1B[38;5;231m'));
	expect(inspect(rgb2ansi_256(0, 0, 0, true))).toEqual(inspect('\x1B[48;5;16m'));
	expect(inspect(rgb2ansi_256(255, 255, 255, true))).toEqual(inspect('\x1B[48;5;231m'));
});

test(`Color - ansi_2562ansi_16 - Convert RGB to ANSI256 correctly`, () => {
	expect(inspect(ansi_2562ansi_16(6))).toEqual(inspect('\x1B[16m'));
	expect(inspect(ansi_2562ansi_16(8))).toEqual(inspect('\x1B[90m'));
	expect(inspect(ansi_2562ansi_16(16))).toEqual(inspect('\x1B[0m'));
	expect(inspect(ansi_2562ansi_16(17))).toEqual(inspect('\x1B[34m'));
	expect(inspect(ansi_2562ansi_16(20))).toEqual(inspect('\x1B[94m'));
	expect(inspect(ansi_2562ansi_16(22))).toEqual(inspect('\x1B[33m'));
	expect(inspect(ansi_2562ansi_16(28))).toEqual(inspect('\x1B[32m'));
	expect(inspect(ansi_2562ansi_16(31))).toEqual(inspect('\x1B[36m'));
	expect(inspect(ansi_2562ansi_16(40))).toEqual(inspect('\x1B[92m'));
	expect(inspect(ansi_2562ansi_16(50))).toEqual(inspect('\x1B[96m'));
	expect(inspect(ansi_2562ansi_16(52))).toEqual(inspect('\x1B[31m'));
	expect(inspect(ansi_2562ansi_16(55))).toEqual(inspect('\x1B[35m'));
	expect(inspect(ansi_2562ansi_16(160))).toEqual(inspect('\x1B[91m'));
	expect(inspect(ansi_2562ansi_16(164))).toEqual(inspect('\x1B[95m'));
	expect(inspect(ansi_2562ansi_16(190))).toEqual(inspect('\x1B[93m'));
	expect(inspect(ansi_2562ansi_16(194))).toEqual(inspect('\x1B[97m'));
	expect(inspect(ansi_2562ansi_16(229))).toEqual(inspect('\x1B[97m'));
	expect(inspect(ansi_2562ansi_16(253))).toEqual(inspect('\x1B[97m'));
	expect(inspect(ansi_2562ansi_16(232))).toEqual(inspect('\x1B[30m'));
	expect(inspect(ansi_2562ansi_16(240))).toEqual(inspect('\x1B[90m'));
	expect(inspect(ansi_2562ansi_16(247))).toEqual(inspect('\x1B[37m'));

	for (let i = 0; i <= 255; i++) {
		expect(`${inspect(ansi_2562ansi_16(i)) !== "'\\x1B[undefinedm'"}${i}`).toEqual(`true${i}`);
		expect(`${inspect(ansi_2562ansi_16(i)) !== "'\\x1B[NaNdm'"}${i}`).toEqual(`true${i}`);
	}

	expect(inspect(ansi_2562ansi_16(52, true))).toEqual(inspect('\x1B[41m'));
});

test(`Color - get_term_color_support - Detect the ANSI support correctly`, () => {
	expect(get_term_color_support() >= 0 && get_term_color_support() <= 3).toEqual(true);

	delete process.env.NO_COLOR;
	process.env.FORCE_COLOR = '0';
	expect(get_term_color_support()).toEqual(0);
	process.env.FORCE_COLOR = '1';
	expect(get_term_color_support()).toEqual(1);
	process.env.FORCE_COLOR = '2';
	expect(get_term_color_support()).toEqual(2);
	process.env.FORCE_COLOR = '3';
	expect(get_term_color_support()).toEqual(3);

	process.env.NO_COLOR = '';
	delete process.env.FORCE_COLOR;
	expect(get_term_color_support()).toEqual(0);
	process.env.FORCE_COLOR = '0';
	expect(get_term_color_support()).toEqual(0);
	process.env.FORCE_COLOR = '1';
	expect(get_term_color_support()).toEqual(1);
	process.env.FORCE_COLOR = '2';
	expect(get_term_color_support()).toEqual(2);
	process.env.FORCE_COLOR = '3';
	expect(get_term_color_support()).toEqual(3);

	process.env.FORCE_COLOR = '';
	expect(get_term_color_support()).toEqual(0);
	process.env.FORCE_COLOR = '3';
	delete process.env.NO_COLOR;
});

test(`Color - Return right object for colors`, () => {
	const output1 = {
		open: '\u001b[30m',
		close: '\u001b[39m',
	};

	const output2 = {
		open: '\u001b[33m',
		close: '\u001b[39m',
	};

	const output3 = {
		open: '\u001b[36m',
		close: '\u001b[39m',
	};

	expect(Color('black')).toEqual(output1);
	expect(Color('yellow')).toEqual(output2);
	expect(Color('cyan')).toEqual(output3);
});

test(`Color - Return empty object for system color`, () => {
	expect(Color('system')).toEqual({ open: '', close: '' });
});

test(`Color - Return no color when env hasn't been set yet`, () => {
	Options.reset();
	Options.set = { env: null };
	expect(Color('red')).toEqual({ open: '', close: '' });
	Options.set = { env: 'node' };
});

test(`Color - Return right object for keyword colors`, () => {
	expect(Color('redBright')).toEqual({ open: '\u001b[91m', close: '\u001b[39m' });
	expect(Color('magentaBright')).toEqual({ open: '\u001b[95m', close: '\u001b[39m' });
	expect(Color('blueBright', true)).toEqual({ open: '\u001b[104m', close: '\u001b[49m' });

	expect(inspect(Color('system').open)).toEqual(inspect(''));
	expect(inspect(Color('transparent').open)).toEqual(inspect('\u001b[49m'));
	expect(inspect(Color('black').open)).toEqual(inspect('\u001b[30m'));
	expect(inspect(Color('black', true).open)).toEqual(inspect('\u001b[40m'));
	expect(inspect(Color('red').open)).toEqual(inspect('\u001b[31m'));
	expect(inspect(Color('red', true).open)).toEqual(inspect('\u001b[41m'));
	expect(inspect(Color('green').open)).toEqual(inspect('\u001b[32m'));
	expect(inspect(Color('green', true).open)).toEqual(inspect('\u001b[42m'));
	expect(inspect(Color('yellow').open)).toEqual(inspect('\u001b[33m'));
	expect(inspect(Color('yellow', true).open)).toEqual(inspect('\u001b[43m'));
	expect(inspect(Color('blue').open)).toEqual(inspect('\u001b[34m'));
	expect(inspect(Color('blue', true).open)).toEqual(inspect('\u001b[44m'));
	expect(inspect(Color('magenta').open)).toEqual(inspect('\u001b[35m'));
	expect(inspect(Color('magenta', true).open)).toEqual(inspect('\u001b[45m'));
	expect(inspect(Color('cyan').open)).toEqual(inspect('\u001b[36m'));
	expect(inspect(Color('cyan', true).open)).toEqual(inspect('\u001b[46m'));
	expect(inspect(Color('white').open)).toEqual(inspect('\u001b[37m'));
	expect(inspect(Color('white', true).open)).toEqual(inspect('\u001b[47m'));
	expect(inspect(Color('gray').open)).toEqual(inspect('\u001b[90m'));
	expect(inspect(Color('gray', true).open)).toEqual(inspect('\u001b[100m'));
	expect(inspect(Color('redBright').open)).toEqual(inspect('\u001b[91m'));
	expect(inspect(Color('redBright', true).open)).toEqual(inspect('\u001b[101m'));
	expect(inspect(Color('greenBright').open)).toEqual(inspect('\u001b[92m'));
	expect(inspect(Color('greenBright', true).open)).toEqual(inspect('\u001b[102m'));
	expect(inspect(Color('yellowBright').open)).toEqual(inspect('\u001b[93m'));
	expect(inspect(Color('yellowBright', true).open)).toEqual(inspect('\u001b[103m'));
	expect(inspect(Color('blueBright').open)).toEqual(inspect('\u001b[94m'));
	expect(inspect(Color('blueBright', true).open)).toEqual(inspect('\u001b[104m'));
	expect(inspect(Color('magentaBright').open)).toEqual(inspect('\u001b[95m'));
	expect(inspect(Color('magentaBright', true).open)).toEqual(inspect('\u001b[105m'));
	expect(inspect(Color('cyanBright').open)).toEqual(inspect('\u001b[96m'));
	expect(inspect(Color('cyanBright', true).open)).toEqual(inspect('\u001b[106m'));
	expect(inspect(Color('whiteBright').open)).toEqual(inspect('\u001b[97m'));
	expect(inspect(Color('whiteBright', true).open)).toEqual(inspect('\u001b[107m'));
});

test(`Color - Return the right colors for the difference color support levels`, () => {
	process.env.FORCE_COLOR = '0';
	expect(Color('red')).toEqual({ open: '', close: '' });
	expect(Color('red', true)).toEqual({ open: '', close: '' });
	expect(Color('#ff8800')).toEqual({ open: '', close: '' });
	expect(Color('#ff8800', true)).toEqual({ open: '', close: '' });
	process.env.FORCE_COLOR = '1';
	expect(Color('red')).toEqual({ open: '\u001b[31m', close: '\u001b[39m' });
	expect(Color('red', true)).toEqual({ open: '\u001b[41m', close: '\u001b[49m' });
	expect(Color('#ff8800')).toEqual({ open: '\u001b[93m', close: '\u001b[39m' });
	expect(Color('#ff8800', true)).toEqual({ open: '\u001b[103m', close: '\u001b[49m' });

	process.env.FORCE_COLOR = '2';
	expect(Color('red')).toEqual({ open: '\u001b[31m', close: '\u001b[39m' });
	expect(Color('red', true)).toEqual({ open: '\u001b[41m', close: '\u001b[49m' });
	expect(Color('#ff8800')).toEqual({ open: '\x1B[38;5;214m', close: '\u001b[39m' });
	expect(Color('#ff8800', true)).toEqual({ open: '\x1B[48;5;214m', close: '\u001b[49m' });

	process.env.FORCE_COLOR = '3';
	expect(Color('red')).toEqual({ open: '\u001b[31m', close: '\u001b[39m' });
	expect(Color('red', true)).toEqual({ open: '\u001b[41m', close: '\u001b[49m' });
	expect(Color('#ff8800')).toEqual({ open: '\x1B[38;2;255;136;0m', close: '\u001b[39m' });
	expect(Color('#ff8800', true)).toEqual({ open: '\x1B[48;2;255;136;0m', close: '\u001b[49m' });

	delete process.env.FORCE_COLOR;
	process.env.NO_COLOR = '';
	expect(get_term_color_support()).toEqual(0);
	expect(Color('red')).toEqual({ open: '', close: '' });
	expect(Color('red', true)).toEqual({ open: '', close: '' });
	expect(Color('#ff8800')).toEqual({ open: '', close: '' });
	expect(Color('#ff8800', true)).toEqual({ open: '', close: '' });
	delete process.env.NO_COLOR;
	process.env.FORCE_COLOR = '3';
});

test(`Color - Candy color returns candy colors`, () => {
	const candy_color = [
		'\u001b[31m',
		'\u001b[32m',
		'\u001b[33m',
		'\u001b[35m',
		'\u001b[36m',
		'\u001b[91m',
		'\u001b[92m',
		'\u001b[93m',
		'\u001b[94m',
		'\u001b[95m',
		'\u001b[96m',
	];

	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
	expect(candy_color.includes(Color('candy').open)).toBe(true);
});

test(`Color - Return right object for hex colors`, () => {
	expect(Color('#ff8800')).toEqual({ open: '\u001b[38;2;255;136;0m', close: '\u001b[39m' });
	expect(Color('#0088ff')).toEqual({ open: '\u001b[38;2;0;136;255m', close: '\u001b[39m' });
});

test(`Color - Return right object for background colors`, () => {
	const output1 = {
		open: '\u001b[46m',
		close: '\u001b[49m',
	};

	const output2 = {
		open: '\u001b[103m',
		close: '\u001b[49m',
	};

	const output3 = {
		open: '\u001b[47m',
		close: '\u001b[49m',
	};

	expect(Color('cyan', true)).toEqual(output1);
	expect(Color('yellowBright', true)).toEqual(output2);
	expect(Color('white', true)).toEqual(output3);
});

test(`Color - Return empty object for unknown colors`, () => {
	const output = {
		open: '',
		close: '',
	};

	expect(Color('null')).toEqual(output);
	expect(Color('doesNotExists')).toEqual(output);
	expect(Color('nada')).toEqual(output);
});

test(`Color - Return empty object when FORCE_COLOR env var is set to 0`, () => {
	const output = {
		open: '',
		close: '',
	};

	process.env.FORCE_COLOR = '0';
	expect(Color('blue')).toEqual(output);
	expect(Color('redBright')).toEqual(output);
	expect(Color('yellow', true)).toEqual(output);
	process.env.FORCE_COLOR = '3';
});

test(`Color - Return named colors for browser environment`, () => {
	Options.reset();
	Options.set = { env: 'browser' };

	const output = {
		open: '<span style="color:#ea3223">',
		close: '</span>',
	};

	expect(Color('red')).toEqual(output);
});

test(`Color - Return hex colors for browser environment`, () => {
	Options.reset();
	Options.set = { env: 'browser' };

	const output = {
		open: '<span style="color:#ff0000">',
		close: '</span>',
	};

	expect(Color('#ff0000')).toEqual(output);
});

test(`Color - Return background colors for browser environment`, () => {
	Options.reset();
	Options.set = { env: 'browser' };

	const output = {
		open: '#ea3223',
		close: '',
	};

	expect(Color('red', true)).toEqual(output);
});
