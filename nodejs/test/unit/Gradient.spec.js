/***************************************************************************************************************************************************************
 *
 * Gradient unit tests
 *
 **************************************************************************************************************************************************************/

const {
	GetLinear,
	GetTheta,
	GetGradientColors,
	PaintLines,
	Color2hex,
	GetGaps,
	TransitionBetweenHex,
	Transition,
	PaintGradient,
} = require('../../src/Gradient.js');

test(`Gradient - GetLinear - Interpolate between points correctly`, () => {
	const start = 0;
	const end = 5;
	const steps = 5;

	expect(GetLinear(start, end, 0, steps)).toBe(start);
	expect(GetLinear(start, end, 1, steps)).toBe(1);
	expect(GetLinear(start, end, 2, steps)).toBe(2);
	expect(GetLinear(start, end, 3, steps)).toBe(3);
	expect(GetLinear(start, end, 4, steps)).toBe(4);
	expect(GetLinear(start, end, 5, steps)).toBe(end);
});

test(`Gradient - GetLinear - Return end point when 0 steps`, () => {
	const start = 0;
	const end = 5;
	const steps = 0;

	expect(GetLinear(start, end, 0, steps)).toBe(end);
});

test(`Gradient - GetTheta - Interpolate between points cylindrical correctly`, () => {
	const start1 = 0;
	const end1 = 5;
	const steps1 = 5;

	expect(GetTheta(start1, end1, 0, steps1)).toBe(start1);
	expect(GetTheta(start1, end1, 1, steps1)).toBe(1);
	expect(GetTheta(start1, end1, 2, steps1)).toBe(2);
	expect(GetTheta(start1, end1, 3, steps1)).toBe(3);
	expect(GetTheta(start1, end1, 4, steps1)).toBe(4);
	expect(GetTheta(start1, end1, 5, steps1)).toBe(end1);

	const start2 = 2;
	const end2 = 3;
	const steps2 = 3;

	expect(GetTheta(start2, end2, 0, steps2)).toBe(start2);
	expect(GetTheta(start2, end2, 1, steps2)).toBe(0.238938230940138);
	expect(GetTheta(start2, end2, 2, steps2)).toBe(4.761061769059863);
	expect(GetTheta(start2, end2, 3, steps2)).toBe(end2);

	const start3 = 3;
	const end3 = 2;
	const steps3 = 3;

	expect(GetTheta(start3, end3, 0, steps3)).toBe(start3);
	expect(GetTheta(start3, end3, 1, steps3)).toBe(4.761061769059862);
	expect(GetTheta(start3, end3, 2, steps3)).toBe(0.23893823094013733);
	expect(GetTheta(start3, end3, 3, steps3)).toBe(end3);

	const start4 = 5;
	const end4 = 1;
	const steps4 = 3;

	expect(GetTheta(start4, end4, 0, steps4)).toBe(start4);
	expect(GetTheta(start4, end4, 1, steps4)).toBe(3.666666666666667);
	expect(GetTheta(start4, end4, 2, steps4)).toBe(2.3333333333333335);
	expect(GetTheta(start4, end4, 3, steps4)).toBe(end4);
});

test(`Gradient - GetTheta - Return end point when 0 steps`, () => {
	const start1 = 0;
	const end1 = 5;
	const steps1 = 0;

	expect(GetTheta(start1, end1, 0, steps1)).toBe(end1);
});

test(`Gradient - GetGradientColors - Return a bunch of colors inbetween two given colors`, () => {
	expect(GetGradientColors('#ff8800', '#8899dd', 10)).toEqual([
		'#ff8800',
		'#fbe211',
		'#c0f721',
		'#7bf331',
		'#44ef41',
		'#50ec86',
		'#5fe8c0',
		'#6ddbe4',
		'#7ab4e0',
		'#8799dd',
	]);
});

test(`Gradient - PaintLines - Should color each character in the array`, () => {
	const colors = ['#ff0000', '#00ff00', '#0000ff'];
	const lines = ['xxx', 'xxx'];

	expect(PaintLines(lines, colors, 0)).toEqual([
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
	]);
});

test(`Gradient - Color2hex - Should return a hex color`, () => {
	expect(Color2hex('red')).toEqual('#ff0000');
	expect(Color2hex('#ff0000')).toEqual('#ff0000');
});

test(`Gradient - GetGaps - Should give us the correct gaps`, () => {
	expect(GetGaps([1], 1)).toEqual([]);
	expect(GetGaps([1, 1], 1)).toEqual([-1]);
	expect(GetGaps([1, 1], 2)).toEqual([0]);
	expect(GetGaps([1, 1], 3)).toEqual([1]);
	expect(GetGaps([1, 1], 4)).toEqual([2]);
	expect(GetGaps([1, 1], 5)).toEqual([3]);

	expect(GetGaps([1, 1, 1], 1)).toEqual([-1, -1]);
	expect(GetGaps([1, 1, 1], 2)).toEqual([-1, 0]);
	expect(GetGaps([1, 1, 1], 3)).toEqual([0, 0]);
	expect(GetGaps([1, 1, 1], 4)).toEqual([0, 1]);
	expect(GetGaps([1, 1, 1], 5)).toEqual([1, 1]);
});

test(`Gradient - TransitionBetweenHex - Should return colors between two`, () => {
	expect(TransitionBetweenHex('#ff0000', '#0000ff', -1)).toEqual([]);
	expect(TransitionBetweenHex('#ff0000', '#0000ff', 0)).toEqual([]);
	expect(TransitionBetweenHex('#ff0000', '#0000ff', 1)).toEqual(['#7f007f']);
	expect(TransitionBetweenHex('#ff0000', '#0000ff', 2)).toEqual(['#aa0055', '#5500aa']);
	expect(TransitionBetweenHex('#ff0000', '#0000ff', 5)).toEqual([
		'#d4002a',
		'#aa0055',
		'#7f007f',
		'#5500aa',
		'#2a00d4',
	]);
});

test(`Gradient - Transition - Should return colors between multiple`, () => {
	expect(Transition(['#ff0000', '#0000ff'], 1)).toEqual(['#0000ff']);
	expect(Transition(['#ff0000', '#0000ff'], 2)).toEqual(['#ff0000', '#0000ff']);
	expect(Transition(['#ff0000', '#0000ff'], 3)).toEqual(['#ff0000', '#7f007f', '#0000ff']);
	expect(Transition(['#ff0000', '#0000ff'], 4)).toEqual(['#ff0000', '#aa0055', '#5500aa', '#0000ff']);

	expect(Transition(['#ff0000', '#00ff00', '#0000ff'], 1)).toEqual(['#0000ff']);
	expect(Transition(['#ff0000', '#00ff00', '#0000ff'], 2)).toEqual(['#ff0000', '#0000ff']);
	expect(Transition(['#ff0000', '#00ff00', '#0000ff'], 3)).toEqual(['#ff0000', '#00ff00', '#0000ff']);
	expect(Transition(['#ff0000', '#00ff00', '#0000ff'], 4)).toEqual(['#ff0000', '#00ff00', '#007f7f', '#0000ff']);
	expect(Transition(['#ff0000', '#00ff00', '#0000ff'], 10)).toEqual([
		'#ff0000',
		'#bf3f00',
		'#7f7f00',
		'#3fbf00',
		'#00ff00',
		'#00cc33',
		'#009966',
		'#006699',
		'#0033cc',
		'#0000ff',
	]);

	expect(Transition(['set2'], 2, { set2: ['#ff0000', '#0000ff'] })).toEqual(['#ff0000', '#0000ff']);
	expect(Transition(['set2'], 4, { set2: ['#ff0000', '#0000ff'] })).toEqual([
		'#ff0000',
		'#aa0055',
		'#5500aa',
		'#0000ff',
	]);
});

test(`Gradient - PaintGradient - Should paint multi-line output`, () => {
	const output1 = PaintGradient({
		output: ['x', 'xxx', 'xxx', 'x'],
		gradient: ['red', 'blue'],
		lines: 4,
		lineHeight: 0,
		fontLines: 1,
		independentGradient: false,
		transitionGradient: false,
	});

	expect(output1).toEqual([
		'\u001b[38;2;255;0;0mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m',
	]);

	const output2 = PaintGradient({
		output: [' x', 'xxx', 'xxx', ' x'],
		gradient: ['red', 'blue'],
		lines: 4,
		lineHeight: 0,
		fontLines: 1,
		independentGradient: false,
		transitionGradient: false,
	});

	expect(output2).toEqual([
		'\u001b[38;2;255;0;0m \u001b[39m\u001b[38;2;0;255;0mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0m \u001b[39m\u001b[38;2;0;255;0mx\u001b[39m',
	]);

	const output3 = PaintGradient({
		output: [' x', 'xxx', 'xxx', ' x'],
		gradient: ['red', 'blue'],
		lines: 4,
		lineHeight: 0,
		fontLines: 1,
		independentGradient: true,
		transitionGradient: false,
	});

	expect(output3).toEqual([
		' \u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;0;255;0mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		' \u001b[38;2;0;0;255mx\u001b[39m',
	]);
});

test(`Gradient - PaintGradient - Should paint multi-line output for transitions`, () => {
	const output1 = PaintGradient({
		output: ['x', 'xxx', 'xxx', 'x'],
		gradient: ['red', 'blue'],
		lines: 4,
		lineHeight: 0,
		fontLines: 1,
		independentGradient: false,
		transitionGradient: true,
	});

	expect(output1).toEqual([
		'\u001b[38;2;255;0;0mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;127;0;127mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m\u001b[38;2;127;0;127mx\u001b[39m\u001b[38;2;0;0;255mx\u001b[39m',
		'\u001b[38;2;255;0;0mx\u001b[39m',
	]);

	const output2 = PaintGradient({
		output: [' x', 'xxxxxxxx', 'xxx', 'x'],
		gradient: ['pride'],
		lines: 4,
		lineHeight: 0,
		fontLines: 1,
		independentGradient: false,
		transitionGradient: true,
	});

	expect(output2).toEqual([
		'\u001b[38;2;117;7;135m \u001b[39m\u001b[38;2;0;77;255mx\u001b[39m',
		'\u001b[38;2;117;7;135mx\u001b[39m\u001b[38;2;0;77;255mx\u001b[39m\u001b[38;2;0;128;38mx\u001b[39m\u001b[38;2;255;237;0mx\u001b[39m\u001b[38;2;255;188;0mx\u001b[39m\u001b[38;2;255;140;0mx\u001b[39m\u001b[38;2;241;71;1mx\u001b[39m\u001b[38;2;228;3;3mx\u001b[39m',
		'\u001b[38;2;117;7;135mx\u001b[39m\u001b[38;2;0;77;255mx\u001b[39m\u001b[38;2;0;128;38mx\u001b[39m',
		'\u001b[38;2;117;7;135mx\u001b[39m',
	]);
});
