/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/released/LICENSE  GNU GPL-3.0-or-later
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * GetLinear            - Interpolate a linear path from a number to another number
 * GetTheta             - Interpolate a radial path from a number to another number
 * GetGradientColors    - Generate the most colorful delta between two colors
 * PaintLines           - Take a bunch of lines and color them in the colors provided
 * Color2hex            - Make sure a color is hex
 * GetGaps              - Calculate the gaps between an array of points
 * TransitionBetweenHex - Generate colors between two given colors
 * Transition           - Generate n colors between x colors
 * PaintGradient        - Paint finished output in a gradient
 *
 **************************************************************************************************************************************************************/

'use strict';

const { GetFirstCharacterPosition } = require('./GetFirstCharacterPosition.js');
const { Color, Hex2rgb, Hex2hsvRad, HsvRad2hex, Rgb2hex } = require('./Color.js');
const { GetLongestLine } = require('./GetLongestLine.js');
const { GRADIENTS } = require('./constants.js');
const { Debugging } = require('./Debugging.js');

/**
 * Interpolate a linear path from a number to another number
 *
 * @param  {number}  pointA - The number from which to start
 * @param  {number}  pointB - The number to go to
 * @param  {number}  n      - The current step
 * @param  {number}  steps  - The amount of steps
 *
 * @return {number}         - The number at step n
 */
function GetLinear(pointA, pointB, n, steps) {
	if (steps === 0) {
		return pointB;
	}

	return pointA + n * ((pointB - pointA) / steps);
}

/**
 * Interpolate a radial path from a number to another number
 *
 * @param  {number}  fromTheta - The radian from which to start
 * @param  {number}  toTheta   - The radian to go to
 * @param  {number}  n         - The current step
 * @param  {number}  steps     - The amount of steps
 *
 * @return {number}            - The radian at step n
 */
function GetTheta(fromTheta, toTheta, n, steps) {
	const TAU = 2 * Math.PI;
	let longDistance;

	if (steps === 0) {
		return toTheta;
	}

	if (fromTheta > toTheta) {
		if (fromTheta - toTheta < Math.PI) {
			longDistance = TAU - (fromTheta - toTheta);
		} else {
			longDistance = toTheta - fromTheta;
		}
	} else {
		if (toTheta - fromTheta < Math.PI) {
			longDistance = toTheta - fromTheta - TAU;
		} else {
			longDistance = -1 * (fromTheta - toTheta);
		}
	}

	let result = fromTheta + n * (longDistance / steps);

	if (result < 0) {
		result += TAU;
	}

	if (result > TAU) {
		result -= TAU;
	}

	return result;
}

/**
 * Generate the most colorful delta between two colors
 *
 * @param  {string}  fromColor - The color from which to start
 * @param  {string}  toColor   - The color to go to
 * @param  {number}  steps     - The amount of colors of the gradient
 *
 * @return {array}             - An array of colors
 */
function GetGradientColors(fromColor, toColor, steps) {
	const [fromHRad, fromS, fromV] = Hex2hsvRad(fromColor);
	const [toHRad, toS, toV] = Hex2hsvRad(toColor);

	const hexColors = [];

	for (let n = 0; n < steps; n++) {
		const hRad = GetTheta(fromHRad, toHRad, n, steps - 1);
		const s = GetLinear(fromS, toS, n, steps - 1);
		const v = GetLinear(fromV, toV, n, steps - 1);

		hexColors.push(HsvRad2hex(hRad, s, v));
	}

	return hexColors;
}

/**
 * Take a bunch of lines and color them in the colors provided
 *
 * @param  {array}   lines                  - The lines to be colored
 * @param  {array}   colors                 - The colors in an array
 * @param  {number}  firstCharacterPosition - We may have to cut something off from the start when text is aligned center, right
 *
 * @return {array}                          - The lines in color
 */
function PaintLines(lines, colors, firstCharacterPosition) {
	Debugging.report(`Running PaintLines`, 1);

	Debugging.report(colors, 2);

	const space = ' '.repeat(firstCharacterPosition);

	return lines.map((line) => {
		const coloredLine = line
			.slice(firstCharacterPosition)
			.split('')
			.map((char, i) => {
				const { open, close } = Color(colors[i]);
				return `${open}${char}${close}`;
			})
			.join('');

		return `${space}${coloredLine}`;
	});
}

/**
 * Make sure a color is hex
 *
 * @param  {string} color - The color
 *
 * @return {string}       - The hex color
 */
function Color2hex(color) {
	const colorMap = {
		black: '#000000',
		red: '#ff0000',
		green: '#00ff00',
		yellow: '#ffff00',
		blue: '#0000ff',
		magenta: '#ff00ff',
		cyan: '#00ffff',
		white: '#ffffff',
		gray: '#808080',
		grey: '#808080',
	};

	return colorMap[color] || color;
}

/**
 * Calculate the gaps between an array of points
 *
 * @param  {array}  points - An array of points, it's not important what's in the array for this function
 * @param  {number} steps  - The amount of steps we have to distribute between the above points
 *
 * @return {array}         - An array of steps per gap
 */
function GetGaps(points, steps) {
	// steps per gap
	const gapSteps = Math.floor((steps - points.length) / (points.length - 1));
	// steps left over to be distributed
	const rest = steps - (points.length + gapSteps * (points.length - 1));
	// the gaps array has one less items than our points (cause it's gaps between each of the points)
	const gaps = Array(points.length - 1).fill(gapSteps);

	// let's fill in the rest from the right
	for (let i = 0; i < rest; i++) {
		gaps[gaps.length - 1 - i]++;
	}

	return gaps;
}

/**
 * Generate colors between two given colors
 *
 * @param  {string} fromHex - The color we start from in hex
 * @param  {string} toHex   - The color we end up at in hex
 * @param  {number} steps   - How many colors should be returned
 *
 * @return {array}          - An array for colors
 */
function TransitionBetweenHex(fromHex, toHex, steps) {
	const fromRgb = Hex2rgb(fromHex);
	const toRgb = Hex2rgb(toHex);
	const hexColors = [];
	steps++;

	for (let n = 1; n < steps; n++) {
		const red = GetLinear(fromRgb[0], toRgb[0], n, steps);
		const green = GetLinear(fromRgb[1], toRgb[1], n, steps);
		const blue = GetLinear(fromRgb[2], toRgb[2], n, steps);

		hexColors.push(Rgb2hex(red, green, blue));
	}

	return hexColors;
}

/**
 * Generate n colors between x colors
 *
 * @param  {array}  colors    - An array of colors in hex
 * @param  {number} steps     - The amount of colors to generate
 * @param  {object} gradients - An object of pre-packaged gradient colors
 *
 * @return {array}            - An array of colors
 */
function Transition(colors, steps, gradients = GRADIENTS) {
	let hexColors = [];
	if (colors.length === 1) {
		colors = gradients[colors[0].toLowerCase()];
	} else {
		colors = colors.map((color) => Color2hex(color));
	}
	const gaps = GetGaps(colors, steps);

	if (steps <= 1) {
		return [colors[colors.length - 1]];
	}

	for (let i = 0; i < colors.length; i++) {
		const gap = gaps[i - 1];

		if (colors[i - 1]) {
			const gapColors = TransitionBetweenHex(colors[i - 1], colors[i], gap);
			hexColors = [...hexColors, ...gapColors];
		}

		if (gap !== -1) {
			hexColors.push(colors[i]);
		}
	}

	return hexColors;
}

/**
 * Paint finished output in a gradient
 *
 * @param  {object}  options                     - Arguments
 * @param  {array}   options.output              - The output to be painted
 * @param  {array}   options.gradient            - An array of two colors for start and end of gradient
 * @param  {number}  options.lines               - How many lines the output contains
 * @param  {number}  options.lineHeight          - The line height between lines
 * @param  {number}  options.fontLines           - The line height (line breaks) of a single font line
 * @param  {boolean} options.independentGradient - A switch to calculate gradient per line or not
 * @param  {boolean} options.transitionGradient  - A switch for transition gradients
 *
 * @return {array}                               - The output array painted in ANSI colors
 */
function PaintGradient({ output, gradient, lines, lineHeight, fontLines, independentGradient, transitionGradient }) {
	Debugging.report(`Running PaintGradient`, 1);
	let newOutput = [];

	if (transitionGradient) {
		Debugging.report(`Gradient transition with colors: ${JSON.stringify(gradient)}`, 2);
	} else {
		Debugging.report(`Gradient start: ${gradient[0]} | Gradient end: ${gradient[1]}`, 2);
	}

	let firstCharacterPosition = GetFirstCharacterPosition(output);
	let longestLine = GetLongestLine(output).length;

	for (let i = 0; i < lines; i++) {
		const start = i * (fontLines + lineHeight);
		const end = fontLines + start;
		const thisLine = output.slice(start, end);

		if (independentGradient) {
			firstCharacterPosition = GetFirstCharacterPosition(thisLine);
			longestLine = GetLongestLine(thisLine).length;
		}

		const colorsNeeded = longestLine - firstCharacterPosition;
		const linesInbetween = i === 0 ? [] : Array(lineHeight).fill('');

		Debugging.report(`longestLine: ${longestLine} | firstCharacterPosition: ${firstCharacterPosition}`, 2);

		const colors = transitionGradient
			? Transition(gradient, colorsNeeded)
			: GetGradientColors(Color2hex(gradient[0]), Color2hex(gradient[1]), colorsNeeded);

		newOutput = [...newOutput, ...linesInbetween, ...PaintLines(thisLine, colors, firstCharacterPosition)];
	}

	return newOutput;
}

module.exports = exports = {
	GetLinear,
	GetTheta,
	GetGradientColors,
	PaintLines,
	Color2hex,
	GetGaps,
	TransitionBetweenHex,
	Transition,
	PaintGradient,
};
