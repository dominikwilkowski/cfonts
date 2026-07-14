import { Align, Cfonts as WasmCfonts, Font, Valign, type Rendered } from "../pkg/cfonts_wasm.js";
import { detection } from "./detection.js";

export { Align, Font, Valign };

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

function forcedSize(): number | undefined {
	// mirrors the core FORCE_SIZE parsing: unsigned integers only, garbage is ignored
	const raw = globalThis.process?.env?.FORCE_SIZE ?? "";

	if (!/^\d+$/.test(raw)) {
		return undefined;
	}

	const size = Number.parseInt(raw, 10);

	return size <= U32_MAX ? size : undefined;
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

	renderCli(): Rendered {
		// FORCE_SIZE takes precedence over whatever the entry point's width detection finds;
		// neither exists inside the WASM so the width crosses the boundary here
		return this.#inner.renderCli(forcedSize() ?? detection.width());
	}

	renderBrowser(): Rendered {
		return this.#inner.renderBrowser();
	}

	renderBrowserConsole(): Rendered {
		return this.#inner.renderBrowserConsole();
	}

	say(): void {
		console.log(this.renderCli().text);
	}
}
