/***************************************************************************************************************************************************************
 *
 * cfonts
 *
 * Sexy fonts for the console. (CLI output)
 *
 * @license     https://github.com/dominikwilkowski/cfonts/blob/master/LICENSE  GNU GPLv2
 * @author      Dominik Wilkowski  hi@dominik-wilkowski.com
 * @repository  https://github.com/dominikwilkowski/cfonts
 *
 * Gradient
 *   Generate the most colorful delta between two colors
 *
 **************************************************************************************************************************************************************/

'use strict';


const { GetFirstCharacterPosition } = require('./GetFirstCharacterPosition.js');
const { GetLongestLine } = require('./GetLongestLine.js');
const { Debugging } = require('./Debugging.js');
const { Color } = require('./Color.js');

/**
 * Converts an RGB color value to HSV.
 *
 * @author https://github.com/Gavin-YYC/colorconvert
 *
 * @param   {object} options   - Arguments
 * @param   {number} options.r - The red color value
 * @param   {number} options.g - The green color value
 * @param   {number} options.b - The blue color value
 *
 * @return  {array}            - The HSV representation
 */
function Rgb2hsv({ r, g, b }) {
	r /= 255;
	g /= 255;
	b /= 255;

	const max = Math.max( r, g, b );
	const min = Math.min( r, g, b );
	const diff = max - min;

	let h = 0;
	let v = max;
	let s = max === 0 ? 0 : diff / max;

	// h
	if( max === min ) {
		h = 0;
	}
	else if( max === r && g >= b ) {
		h = 60 * ( ( g - b ) / diff );
	}
	else if( max === r && g < b ) {
		h = 60 * ( ( g - b ) / diff ) + 360;
	}
	else if( max === g ) {
		h = 60 * ( ( b - r ) / diff ) + 120;
	}
	else { // if( max === b ) {
		h = 60 * ( ( r - g ) / diff ) + 240;
	};

	return [ h, ( s * 100 ), ( v * 100 ) ];
}

/**
 * Converts an HSV color value to RGB.
 *
 * @author https://github.com/Gavin-YYC/colorconvert
 *
 * @param   {number}  h - The hue
 * @param   {number}  s - The saturation
 * @param   {number}  v - The value
 *
 * @typedef  {object} ReturnObject
 *   @property {number}  r  - The red value
 *   @property {number}  g  - The green value
 *   @property {number}  b  - The blue value
 *
 * @return  {ReturnObject}  - The RGB representation
 */
function Hsv2rgb( h, s, v ) {
	h /= 60;
	s /= 100;
	v /= 100;
	const hi = Math.floor( h ) % 6;

	const f = h - Math.floor( h );
	const p = 255 * v * ( 1 - s );
	const q = 255 * v * ( 1 - ( s * f ) );
	const t = 255 * v * ( 1 - ( s * ( 1 - f ) ) );
	v *= 255;

	switch( hi ) {
		case 0:
			return { r: v, g: t, b: p };
		case 1:
			return { r: q, g: v, b: p };
		case 2:
			return { r: p, g: v, b: t };
		case 3:
			return { r: p, g: q, b: v };
		case 4:
			return { r: t, g: p, b: v };
		case 5:
			return { r: v, g: p, b: q };
	}
}

/**
 * Converts RGB to HEX
 *
 * @param  {number} r - The Red value
 * @param  {number} g - The Green value
 * @param  {number} b - The Blue value
 *
 * @return {string}   - A HEX color
 */
function Rgb2hex( r, g, b ) {
	const val = ( ( b | g << 8 | r << 16) | 1 << 24 ).toString( 16 ).slice( 1 );
	return '#' + val.toLowerCase();
}

/**
 * Convert HEX to RGB
 *
 * @param  {string} hex - The HEX color
 *
 * @return {array}      - An object with RGB values
 */
function Hex2rgb( hex ) {
	hex = hex.replace(/^#/, '');

	if( hex.length > 6 ) {
		hex = hex.slice( 0, 6 );
	}

	if( hex.length === 4 ) {
		hex = hex.slice( 0, 3 );
	}

	if( hex.length === 3 ) {
		hex = hex[ 0 ] + hex[ 0 ] + hex[ 1 ] + hex[ 1 ] + hex[ 2 ] + hex[ 2 ];
	}

	const num = parseInt( hex, 16 );
	const r = num >> 16;
	const g = ( num >> 8 ) & 255;
	const b = num & 255;
	const rgb = [ r, g, b ];

	return rgb;
}

/**
 * Convert HSV coordinate to HSVrad (degree to radian)
 *
 * @param  {array}  argument  - The HSV representation of a color
 *
 * @return {array}            - The HSVrad color
 */
function Hsv2hsvRad([ h, s, v ]) {
	return [ ( h * Math.PI ) / 180, s, v ];
}

/**
 * Convert HSVrad color to HSV (radian to degree)
 *
 * @param {number} hRad - H in rad
 * @param {number} s    - S
 * @param {number} v    - V
 *
 * @return {array}    - The HSV color
 */
function HsvRad2hsv( hRad, s, v ) {
	return [ ( hRad * 180 ) / Math.PI, s, v ];
}

/**
 * Convert HEX to HSVrad
 *
 * @param  {string} hex - A HEX color
 *
 * @return {array}      - The HSVrad color
 */
function Hex2hsvRad( hex ) {
	const [ r, g, b, ] = Hex2rgb( hex );
	const hsv = Rgb2hsv({ r, g, b, });
	const hsvRad = Hsv2hsvRad( hsv );

	return hsvRad;
}

/**
 * Convert HSVrad to HEX
 *
 * @param  {number} hRad - The hue in rad
 * @param  {number} s    - The saturation
 * @param  {number} v    - The value
 *
 * @return {string}      - The HEX color
 */
function HsvRad2hex( hRad, s, v ) {
	const [ h ] = HsvRad2hsv( hRad, s, v );
	const { r, g, b } = Hsv2rgb( h, s, v );
	const hex = Rgb2hex( r, g, b );

	return hex;
}

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
function GetLinear( pointA, pointB, n, steps ) {
	if( steps === 0 ) {
		return pointB;
	}

	return pointA + n * ( ( pointB - pointA ) / steps );
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
function GetTheta( fromTheta, toTheta, n, steps ) {
	const TAU = 2 * Math.PI;
	let longDistance;

	if( steps === 0 ) {
		return toTheta;
	}

	if( fromTheta > toTheta ) {
		if( fromTheta - toTheta < Math.PI ) {
			longDistance = TAU - ( fromTheta - toTheta );
		}
		else {
			longDistance = toTheta - fromTheta;
		}
	}
	else {
		if( toTheta - fromTheta < Math.PI ) {
			longDistance = ( toTheta - fromTheta ) - TAU;
		}
		else {
			longDistance = -1 * ( fromTheta - toTheta );
		}
	}

	let result = fromTheta + ( n * ( longDistance / steps ) );

	if( result < 0 ) {
		result += TAU;
	}

	if( result > TAU ) {
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
function GetGradientColors( fromColor, toColor, steps ) {
	const [ fromHRad, fromS, fromV ] = Hex2hsvRad( fromColor );
	const [ toHRad, toS, toV ] = Hex2hsvRad( toColor );

	const hexColors = [];

	for( let n = 0; n < steps; n++ ) {
		const hRad = GetTheta( fromHRad, toHRad, n, ( steps - 1 ) );
		const s = GetLinear( fromS, toS, n, ( steps - 1 ) );
		const v = GetLinear( fromV, toV, n, ( steps - 1 ) );

		hexColors.push( HsvRad2hex( hRad, s, v ) );
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
function PaintLines( lines, colors, firstCharacterPosition ) {
	Debugging.report(`Running PaintLines`, 1);

	Debugging.report( colors, 2 );

	const output = [];
	const space = ' '.repeat( firstCharacterPosition );

	return lines
		.map( line => {
			const coloredLine = line
				.slice( firstCharacterPosition )
				.split('')
				.map( ( char, i ) => {
					const { open, close } = Color( colors[ i ] );
					return `${ open }${ char }${ close }`;
				})
				.join('');

			return `${ space }${ coloredLine }`;
		});
}


/**
 * Make sure a color is hex
 *
 * @param  {string} color - The color
 *
 * @return {string}       - The hex color
 */
function Color2hex( color ) {
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

	return colorMap[ color ] || color;
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
 *
 * @return {array}                               - The output array painted in ANSI colors
 */
function PaintGradient({ output, gradient, lines, lineHeight, fontLines, independentGradient }) {
	Debugging.report(`Running PaintGradient`, 1);
	const gradientStart = Color2hex( gradient[ 0 ] );
	const gradientEnd = Color2hex( gradient[ 1 ] );
	let newOutput = [];

	Debugging.report(`Gradient start: ${ gradientStart } | Gradient end: ${ gradientEnd }`, 2);

	let firstCharacterPosition;
	let longestLine;

	if( !independentGradient ) {
		firstCharacterPosition = GetFirstCharacterPosition( output );
		longestLine = GetLongestLine( output ).length;
	}

	for( let i = 0; i < lines; i++ ) {
		const start = ( i * ( fontLines + lineHeight ) );
		const end = fontLines + start;
		const thisLine = output.slice( start, end );

		if( independentGradient ) {
			firstCharacterPosition = GetFirstCharacterPosition( thisLine );
			longestLine = GetLongestLine( thisLine ).length;
		}

		const colorsNeeded = longestLine - firstCharacterPosition;
		const linesInbetween = i === 0
			? []
			: Array( lineHeight ).fill('\n');

		Debugging.report(`longestLine: ${ longestLine } | firstCharacterPosition: ${ firstCharacterPosition }`, 2);

		const colors = GetGradientColors( gradientStart, gradientEnd, colorsNeeded );
		newOutput = [ ...newOutput, ...linesInbetween, ...PaintLines( thisLine, colors, firstCharacterPosition ) ];
	}

	return newOutput;
}


module.exports = exports = {
	Rgb2hsv,
	Hsv2rgb,
	Rgb2hex,
	Hex2rgb,
	Hsv2hsvRad,
	HsvRad2hsv,
	Hex2hsvRad,
	HsvRad2hex,
	GetLinear,
	GetTheta,
	GetGradientColors,
	PaintLines,
	Color2hex,
	PaintGradient,
};
