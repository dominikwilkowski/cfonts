import { Align, Cfonts as WasmCfonts, Env, Font, Valign, type Rendered } from "../pkg/cfonts_wasm.js";

export { Align, Env, Font, Valign };

export type { Rendered };

const U32_MAX = 0xffff_ffff;

function expectString(value: unknown, method: string): string {
	if (typeof value !== "string") {
		throw new TypeError(`\`${method}()\` expects a string`);
	}

	return value;
}

function expectU32(value: unknown, method: string): number {
	if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > U32_MAX) {
		throw new TypeError(`\`${method}()\` expects an unsigned 32-bit integer`);
	}

	return value;
}

function expectEnum<T extends number>(value: unknown, enumeration: object, method: string): T {
	if (typeof value !== "number" || !Number.isInteger(value) || !Object.hasOwn(enumeration, value)) {
		throw new TypeError(`\`${method}()\` expects a supported enum value`);
	}

	return value as T;
}

export class Cfonts {
	readonly #inner: WasmCfonts;

	private constructor(inner: WasmCfonts) {
		this.#inner = inner;
	}

	static text(input: string): Cfonts {
		return new Cfonts(WasmCfonts.text(expectString(input, "text")));
	}

	newText(input: string): this {
		this.#inner.newText(expectString(input, "newText"));
		return this;
	}

	font(font: Font): this {
		this.#inner.font(expectEnum<Font>(font, Font, "font"));
		return this;
	}

	letterSpacing(letterSpacing: number): this {
		this.#inner.letterSpacing(expectU32(letterSpacing, "letterSpacing"));
		return this;
	}

	lineHeight(lineHeight: number): this {
		this.#inner.lineHeight(expectU32(lineHeight, "lineHeight"));
		return this;
	}

	env(env: Env): this {
		this.#inner.env(expectEnum<Env>(env, Env, "env"));
		return this;
	}

	align(align: Align): this {
		this.#inner.align(expectEnum<Align>(align, Align, "align"));
		return this;
	}

	valign(valign: Valign): this {
		this.#inner.valign(expectEnum<Valign>(valign, Valign, "valign"));
		return this;
	}

	maxLength(maxLength: number): this {
		this.#inner.maxLength(expectU32(maxLength, "maxLength"));
		return this;
	}

	wordWrap(): this {
		this.#inner.wordWrap();
		return this;
	}

	spaceless(): this {
		this.#inner.spaceless();
		return this;
	}

	render(): Rendered {
		return this.#inner.render();
	}

	say(): void {
		console.log(this.render().text);
	}
}
