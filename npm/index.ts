import { Align, Cfonts as WasmCfonts, Env, Font, Valign, type Rendered } from "../pkg/cfonts_wasm.js";

export { Align, Env, Font, Valign };

export type { Rendered };

export class Cfonts {
	readonly #inner: WasmCfonts;

	private constructor(inner: WasmCfonts) {
		this.#inner = inner;
	}

	static text(input: string): Cfonts {
		return new Cfonts(WasmCfonts.text(input));
	}

	newText(input: string): this {
		this.#inner.newText(input);
		return this;
	}

	font(font: Font): this {
		this.#inner.font(font);
		return this;
	}

	letterSpacing(letterSpacing: number): this {
		this.#inner.letterSpacing(letterSpacing);
		return this;
	}

	wordWrap(): this {
		this.#inner.wordWrap();
		return this;
	}

	lineHeight(lineHeight: number): this {
		this.#inner.lineHeight(lineHeight);
		return this;
	}

	env(env: Env): this {
		this.#inner.env(env);
		return this;
	}

	align(align: Align): this {
		this.#inner.align(align);
		return this;
	}

	valign(valign: Valign): this {
		this.#inner.valign(valign);
		return this;
	}

	spaceless(): this {
		this.#inner.spaceless();
		return this;
	}

	maxLength(maxLength: number): this {
		this.#inner.maxLength(maxLength);
		return this;
	}

	render(): Rendered {
		return this.#inner.render();
	}

	say(): void {
		console.log(this.render().text);
	}
}
