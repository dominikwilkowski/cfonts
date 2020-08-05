/**
 * Main method to get the ANSI output for a string
 *
 * @param  {string}  input       - The string you want to write out; use '|' character for line break
 * @param  {object}  settings    - Settings object
 * @param  {boolean} debug       - A flag to enable debug mode
 * @param  {number}  debugLevel  - The debug level we want to show
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {number}  size.width  - The width of the terminal
 * @param  {number}  size.height - The height of the terminal
 *
 * @typedef  {(object|boolean)} RenderOutput
 *   @property {string}  string  - The pure string for output with all line breaks
 *   @property {array}   array   - Each line of output in an array
 *   @property {number}  lines   - The number of lines
 *   @property {object}  options - All options used
 *
 * @return {RenderOutput}        - CLI output of INPUT to be consoled out
 */
export const render: (input: string, settings?: Partial<Options>, debug?: boolean, debugLevel?: number, size?: Partial<Size>) => RenderOutput;
/**
 * Print to console
 *
 * @param  {string}  input       - The string you want to write out; use '|' character for line break
 * @param  {object}  settings    - Settings object
 * @param  {boolean} debug       - A flag to enable debug mode
 * @param  {number}  debuglevel  - The debug level we want to show
 * @param  {object}  size        - The size of the terminal as an object, default: Size
 * @param  {number}  size.width  - The width of the terminal
 * @param  {number}  size.height - The height of the terminal
 */
export const say: (input: string, settings?: Partial<Options>, debug?: boolean, debugLevel?: number, size?: Partial<Size>) => void;

export interface Size {
	/** the width of the terminal */
	width: number;
	/** the height of the terminal */
	height: number;
}

export type Font = "3d" | "block" | "chrome" | "console" | "grid" | "huge" | "pallet" | "shade" | "simple" | "simpleBlock" | "simple3d" | "slick" | "tiny";
export type Alignment = "left" | "center" | "right";
export type Environment = "node" | "browser";

export interface Options {
	/**
	 * the font face
	 * @default 'block'
	 */
	font: Font;
	/**
	 * the text alignment
	 * @default 'left'
	 */
	align: Alignment;
	/**
	 * all colors to use
	 * @default ['system']
	 */
	colors: string[];
	/**
	 * the background color
	 * you can also use `backgroundColor` here as key
	 * @default 'transparent'
	 */
	background: string;
	/**
	 * letter spacing
	 * @default 1
	 */
	letterSpacing: number;
	/**
	 * the line height
	 * @default 1
	 */
	lineHeight: number;
	/**
	 * if the output text should have empty lines on top and on the bottom
	 * @default true
	 */
	space: boolean;
	/**
	 * how many character can be on one line
	 * 0 means no max width and the text will break at the edge of the terminal window
	 * @default 0
	 */
	maxLength: number;
	/**
	 * your two gradient colors
	 * @default false (disabled)
	 */
	gradient: string[] | false;
	/**
	 * if you want to recalculate the gradient for each new line
	 * @default false
	 */
	independentGradient: boolean;
	/**
	 * if this is a transition between colors directly
	 * @default false
	 */
	transitionGradient: boolean;
	/**
	 * the environment CFonts is being executed in
	 * @default 'node'
	 */
	env: Environment;
}

export interface RenderOutput {
	/** The pure string for output with all line breaks */
	string: string;
	/** Each line of output in an array */
	array: string[];
	/** The number of lines */
	lines: number;
	/** All options used */
	options: Options;
}
