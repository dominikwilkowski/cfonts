export const U32_MAX = 0xffff_ffff;

export function expectString(value: unknown, method: string): string {
	if (typeof value !== "string") {
		throw new TypeError(`\`${method}()\` expects a string`);
	}

	return value;
}

export function expectU32(value: unknown, method: string): number {
	if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > U32_MAX) {
		throw new TypeError(`\`${method}()\` expects an unsigned 32-bit integer`);
	}

	return value;
}

export function expectU8(value: unknown, method: string): number {
	if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > 255) {
		throw new TypeError(`\`${method}()\` expects RGB channel values as integers between 0 and 255`);
	}

	return value;
}

export function expectBoolean(value: unknown, method: string): boolean {
	if (typeof value !== "boolean") {
		throw new TypeError(`\`${method}()\` expects a boolean`);
	}

	return value;
}

export function expectEnum<T extends number>(value: unknown, enumeration: object, method: string): T {
	if (typeof value !== "number" || !Number.isInteger(value) || !Object.hasOwn(enumeration, value)) {
		throw new TypeError(`\`${method}()\` expects a supported enum value`);
	}

	return value as T;
}

export function parseU32(value: string | undefined): number | undefined {
	if (value === undefined || !/^\d+$/.test(value)) {
		return undefined;
	}

	const parsed = Number(value);

	if (!Number.isInteger(parsed) || parsed < 0 || parsed > U32_MAX) {
		return undefined;
	}

	return parsed;
}
