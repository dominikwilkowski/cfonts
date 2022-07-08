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
 * HEXTEST                - Regex to see if a string is a hex color
 * Rgb2hsv                - Converts an RGB color value to HSV
 * Hsv2rgb                - Converts an HSV color value to RGB
 * Rgb2hex                - Converts RGB to HEX
 * Hex2rgb                - Convert HEX to RGB
 * Hsv2hsvRad             - Convert HSV coordinate to HSVrad (degree to radian)
 * HsvRad2hsv             - Convert HSVrad color to HSV (radian to degree)
 * Hex2hsvRad             - Convert HEX to HSVrad
 * HsvRad2hex             - Convert HSVrad to HEX
 * rgb2ansi_16m -         - Convert RGB values to ANSI16 million colors - truecolor
 * rgb2ansi256Code        - Convert RGB values to ANSI256 escape code
 * rgb2ansi_256           - Convert RGB values to ANSI256
 * ansi_2562ansi_16       - Convert ANSI256 code values to ANSI16
 * get_term_color_support - Detect the ANSI support for the current terminal taking into account env vars NO_COLOR and FORCE_COLOR
 * Color                  - Abstraction for coloring hex-, keyword- and background-colors
 *
 **************************************************************************************************************************************************************/

'use strict';

const { supportsColor } = require('supports-color');

const { Options } = require('./Options.js');

/**
 * Regex to see if a string is a hex color
 *
 * @type {RegExp}
 */
const HEXTEST = RegExp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$');

/**
 * Converts an RGB color value to HSV
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

	const max = Math.max(r, g, b);
	const min = Math.min(r, g, b);
	const diff = max - min;

	let h = 0;
	let v = max;
	let s = max === 0 ? 0 : diff / max;

	// h
	if (max === min) {
		h = 0;
	} else if (max === r && g >= b) {
		h = 60 * ((g - b) / diff);
	} else if (max === r && g < b) {
		h = 60 * ((g - b) / diff) + 360;
	} else if (max === g) {
		h = 60 * ((b - r) / diff) + 120;
	} else {
		// if( max === b ) {
		h = 60 * ((r - g) / diff) + 240;
	}

	return [h, s * 100, v * 100];
}

/**
 * Converts an HSV color value to RGB
 *
 * @author https://github.com/Gavin-YYC/colorconvert
 *
 * @param   {number}  h - The hue
 * @param   {number}  s - The saturation
 * @param   {number}  v - The value
 *
 * @typedef  {object} Hsv2rgbReturnObject
 *   @property {number}  r  - The red value
 *   @property {number}  g  - The green value
 *   @property {number}  b  - The blue value
 *
 * @return  {Hsv2rgbReturnObject}  - The RGB representation
 */
function Hsv2rgb(h, s, v) {
	h /= 60;
	s /= 100;
	v /= 100;
	const hi = Math.floor(h) % 6;

	const f = h - Math.floor(h);
	const p = 255 * v * (1 - s);
	const q = 255 * v * (1 - s * f);
	const t = 255 * v * (1 - s * (1 - f));
	v *= 255;

	switch (hi) {
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
function Rgb2hex(r, g, b) {
	const val = (b | (g << 8) | (r << 16) | (1 << 24)).toString(16).slice(1);
	return '#' + val.toLowerCase();
}

/**
 * Convert HEX to RGB
 *
 * @param  {string} hex - The HEX color
 *
 * @return {array}      - An object with RGB values
 */
function Hex2rgb(hex) {
	hex = hex.replace(/^#/, '');

	if (hex.length > 6) {
		hex = hex.slice(0, 6);
	}

	if (hex.length === 4) {
		hex = hex.slice(0, 3);
	}

	if (hex.length === 3) {
		hex = hex[0] + hex[0] + hex[1] + hex[1] + hex[2] + hex[2];
	}

	const num = parseInt(hex, 16);
	const r = num >> 16;
	const g = (num >> 8) & 255;
	const b = num & 255;
	const rgb = [r, g, b];

	return rgb;
}

/**
 * Convert HSV coordinate to HSVrad (degree to radian)
 *
 * @param  {array}  argument  - The HSV representation of a color
 *
 * @return {array}            - The HSVrad color
 */
function Hsv2hsvRad([h, s, v]) {
	return [(h * Math.PI) / 180, s, v];
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
function HsvRad2hsv(hRad, s, v) {
	const precision = 1000000000000;
	const h = Math.round(((hRad * 180) / Math.PI) * precision) / precision;
	return [h, s, v];
}

/**
 * Convert HEX to HSVrad
 *
 * @param  {string} hex - A HEX color
 *
 * @return {array}      - The HSVrad color
 */
function Hex2hsvRad(hex) {
	const [r, g, b] = Hex2rgb(hex);
	const hsv = Rgb2hsv({ r, g, b });
	const hsvRad = Hsv2hsvRad(hsv);

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
function HsvRad2hex(hRad, s, v) {
	const [h] = HsvRad2hsv(hRad, s, v);
	const { r, g, b } = Hsv2rgb(h, s, v);
	const hex = Rgb2hex(r, g, b);

	return hex;
}

/**
 * Convert RGB values to ANSI16 million colors - truecolor
 *
 * @param  {number}   r  - Red value
 * @param  {number}   g  - Green value
 * @param  {number}   b  - Blue value
 * @param  {boolean} bg  - Is this a background color; default: false
 *
 * @return {string}      - The opening ANSI escape sequence for the given color
 */
function rgb2ansi_16m(r, g, b, bg = false) {
	const layer_code = bg ? 48 : 38;
	return `\u001b[${layer_code};2;${r};${g};${b}m`;
}

/**
 * Convert RGB values to ANSI256 escape code
 *
 * @param  {number}  red     - Red value
 * @param  {number}  green   - Green value
 * @param  {number}  blue    - Blue value
 *
 * @return {number}          - The ANSI escape code for the given color
 */
function rgb2ansi256Code(red, green, blue) {
	if (red === green && green === blue) {
		if (red < 8) {
			return 16;
		}

		if (red > 248) {
			return 231;
		}

		return Math.round(((red - 8) / 247) * 24) + 232;
	}

	return 16 + 36 * Math.round((red / 255) * 5) + 6 * Math.round((green / 255) * 5) + Math.round((blue / 255) * 5);
}

/**
 * Convert RGB values to ANSI256
 *
 * @param  {number}   r  - Red value
 * @param  {number}   g  - Green value
 * @param  {number}   b  - Blue value
 * @param  {boolean} bg  - Is this a background color; default: false
 *
 * @return {string}      - The opening ANSI escape sequence for the given color
 */
function rgb2ansi_256(r, g, b, bg = false) {
	const layer_code = bg ? 48 : 38;
	const code = rgb2ansi256Code(r, g, b);
	return `\u001b[${layer_code};5;${code}m`;
}

/**
 * Convert ANSI256 code values to ANSI16
 *
 * @param  {number}   code  - The code of the ANSI256 color
 * @param  {boolean} bg     - Is this a background color; default: false
 *
 * @return {string}         - The opening ANSI escape sequence for the given color
 */
function ansi_2562ansi_16(code, bg = false) {
	let ansi_16_code;
	if (code <= 7) {
		ansi_16_code = code + 10;
	}
	if (code >= 8 && code <= 15) {
		ansi_16_code = code + 82;
	}
	if (code === 16) {
		ansi_16_code = 0;
	}
	if (code >= 17 && code <= 19) {
		ansi_16_code = 34;
	}
	if ((code >= 20 && code <= 21) || (code >= 25 && code <= 27)) {
		ansi_16_code = 94;
	}
	if (
		(code >= 22 && code <= 24) ||
		(code >= 58 && code <= 60) ||
		(code >= 64 && code <= 66) ||
		(code >= 94 && code <= 95) ||
		(code >= 100 && code <= 102) ||
		(code >= 106 && code <= 108) ||
		(code >= 130 && code <= 131) ||
		(code >= 136 && code <= 138) ||
		(code >= 142 && code <= 144) ||
		(code >= 148 && code <= 151) ||
		(code >= 172 && code <= 174) ||
		(code >= 178 && code <= 181) ||
		(code >= 184 && code <= 189)
	) {
		ansi_16_code = 33;
	}
	if (
		(code >= 28 && code <= 30) ||
		(code >= 34 && code <= 36) ||
		(code >= 70 && code <= 72) ||
		(code >= 76 && code <= 79) ||
		(code >= 112 && code <= 114)
	) {
		ansi_16_code = 32;
	}
	if (
		(code >= 31 && code <= 33) ||
		(code >= 37 && code <= 39) ||
		(code >= 44 && code <= 45) ||
		(code >= 61 && code <= 63) ||
		(code >= 67 && code <= 69) ||
		(code >= 73 && code <= 75) ||
		(code >= 80 && code <= 81) ||
		(code >= 103 && code <= 111) ||
		(code >= 115 && code <= 117) ||
		(code >= 152 && code <= 153)
	) {
		ansi_16_code = 36;
	}
	if (
		(code >= 40 && code <= 43) ||
		(code >= 46 && code <= 49) ||
		(code >= 82 && code <= 85) ||
		(code >= 118 && code <= 120) ||
		(code >= 154 && code <= 157)
	) {
		ansi_16_code = 92;
	}
	if (
		(code >= 50 && code <= 51) ||
		(code >= 86 && code <= 87) ||
		(code >= 121 && code <= 123) ||
		(code >= 158 && code <= 159)
	) {
		ansi_16_code = 96;
	}
	if (
		(code >= 52 && code <= 54) ||
		(code >= 88 && code <= 90) ||
		(code >= 124 && code <= 126) ||
		(code >= 166 && code <= 168)
	) {
		ansi_16_code = 31;
	}
	if (
		(code >= 55 && code <= 57) ||
		(code >= 91 && code <= 93) ||
		(code >= 96 && code <= 99) ||
		(code >= 127 && code <= 129) ||
		(code >= 132 && code <= 135) ||
		(code >= 139 && code <= 141) ||
		(code >= 145 && code <= 147) ||
		(code >= 169 && code <= 171) ||
		(code >= 175 && code <= 177)
	) {
		ansi_16_code = 35;
	}
	if ((code >= 160 && code <= 163) || (code >= 196 && code <= 199) || (code >= 202 && code <= 213)) {
		ansi_16_code = 91;
	}
	if (
		(code >= 164 && code <= 165) ||
		(code >= 182 && code <= 183) ||
		(code >= 200 && code <= 201) ||
		(code >= 218 && code <= 219)
	) {
		ansi_16_code = 95;
	}
	if ((code >= 190 && code <= 193) || (code >= 214 && code <= 217) || (code >= 220 && code <= 228)) {
		ansi_16_code = 93;
	}
	if ((code >= 194 && code <= 195) || (code >= 229 && code <= 231) || (code >= 253 && code <= 255)) {
		ansi_16_code = 97;
	}
	if (code >= 232 && code <= 239) {
		ansi_16_code = 30;
	}
	if (code >= 240 && code <= 246) {
		ansi_16_code = 90;
	}
	if (code >= 247 && code <= 252) {
		ansi_16_code = 37;
	}

	if (bg) {
		ansi_16_code = ansi_16_code + 10;
	}

	return `\u001b[${ansi_16_code}m`;
}

/**
 * Detect the ANSI support for the current terminal taking into account env vars NO_COLOR and FORCE_COLOR
 *
 * @return {number}  - 0 = no color support; 1 = 16 colors support; 2 = 256 colors support; 3 = 16 million colors support
 */
function get_term_color_support() {
	let term_support = supportsColor().level || 3;

	if ('NO_COLOR' in process.env) {
		term_support = 0;
	}

	if (process.env['FORCE_COLOR'] === '0') {
		term_support = 0;
	}

	if (process.env['FORCE_COLOR'] === '1') {
		term_support = 1;
	}

	if (process.env['FORCE_COLOR'] === '2') {
		term_support = 2;
	}

	if (process.env['FORCE_COLOR'] === '3') {
		term_support = 3;
	}

	return term_support;
}

/**
 * Abstraction for coloring hex-, keyword- and background-colors
 *
 * @param  {string}  color    - The color to be used
 * @param  {boolean} bg       - Whether this is a background or not
 *
 * @typedef  {object} ColorReturnObject
 *   @property {string} open  - The open ansi code
 *   @property {string} close - The close ansi code
 *
 * @return {ColorReturnObject}     - An object with open and close ansi codes
 */
const Color = (color, bg = false) => {
	const COLORS = {
		black: '#000',
		red: '#ea3223',
		green: '#377d22',
		yellow: '#fffd54',
		blue: '#0020f5',
		magenta: '#ea3df7',
		cyan: '#74fbfd',
		white: '#fff',
		gray: '#808080',
		redbright: '#ee776d',
		greenbright: '#8cf57b',
		yellowbright: '#fffb7f',
		bluebright: '#6974f6',
		magentabright: '#ee82f8',
		cyanbright: '#8dfafd',
		whitebright: '#fff',
	};

	const support = get_term_color_support();

	// bail early if we use system color
	if (color === 'system' || support === 0) {
		return { open: '', close: '' };
	}

	const OPTIONS = Options.get;

	if (OPTIONS.env === 'node') {
		let open;
		let close = bg ? '\u001b[49m' : '\u001b[39m';

		switch (color.toLowerCase()) {
			case 'transparent':
				open = '\u001b[49m';
				break;
			case 'black':
				open = bg ? '\u001b[40m' : '\u001b[30m';
				break;
			case 'red':
				open = bg ? '\u001b[41m' : '\u001b[31m';
				break;
			case 'green':
				open = bg ? '\u001b[42m' : '\u001b[32m';
				break;
			case 'yellow':
				open = bg ? '\u001b[43m' : '\u001b[33m';
				break;
			case 'blue':
				open = bg ? '\u001b[44m' : '\u001b[34m';
				break;
			case 'magenta':
				open = bg ? '\u001b[45m' : '\u001b[35m';
				break;
			case 'cyan':
				open = bg ? '\u001b[46m' : '\u001b[36m';
				break;
			case 'white':
				open = bg ? '\u001b[47m' : '\u001b[37m';
				break;
			case 'gray':
				open = bg ? '\u001b[100m' : '\u001b[90m';
				break;
			case 'redbright':
				open = bg ? '\u001b[101m' : '\u001b[91m';
				break;
			case 'greenbright':
				open = bg ? '\u001b[102m' : '\u001b[92m';
				break;
			case 'yellowbright':
				open = bg ? '\u001b[103m' : '\u001b[93m';
				break;
			case 'bluebright':
				open = bg ? '\u001b[104m' : '\u001b[94m';
				break;
			case 'magentabright':
				open = bg ? '\u001b[105m' : '\u001b[95m';
				break;
			case 'cyanbright':
				open = bg ? '\u001b[106m' : '\u001b[96m';
				break;
			case 'whitebright':
				open = bg ? '\u001b[107m' : '\u001b[97m';
				break;
			case 'candy':
				open = [
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
				][Math.floor(Math.random() * 11)];
				break;
			default: {
				let hex = color;
				if (!HEXTEST.test(color)) {
					return { open: '', close: '' };
				}
				const rgb = Hex2rgb(hex);

				if (support === 1) {
					open = ansi_2562ansi_16(rgb2ansi256Code(rgb[0], rgb[1], rgb[2]), bg);
				}
				if (support === 2) {
					open = rgb2ansi_256(rgb[0], rgb[1], rgb[2], bg);
				}
				if (support === 3) {
					open = rgb2ansi_16m(rgb[0], rgb[1], rgb[2], bg);
				}
			}
		}
		return { open, close };
	} else if (!OPTIONS.env) {
		return { open: '', close: '' };
	} else {
		if (!HEXTEST.test(color)) {
			color = COLORS[color.toLowerCase()];
			if (!color) {
				return { open: '', close: '' };
			}
		}

		if (bg) {
			return {
				open: color,
				close: '',
			};
		}

		return {
			open: `<span style="color:${color}">`,
			close: '</span>',
		};
	}
};

module.exports = exports = {
	HEXTEST,
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
};
