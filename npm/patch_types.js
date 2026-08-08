// Patches the generated wasm declaration so every consumer target can check it:
// - strips the `[Symbol.dispose]()` member: it requires `lib: esnext` and would
//   break type checking for consumers on any stable target (verified failing on
//   es2022 through es2024)
// - rewrites the loader types that name DOM-lib globals (RequestInfo, BufferSource,
//   WebAssembly) into lib-free shapes, so the Node entry never injects DOM globals
//   into a consumer's typecheck
import { readFileSync, writeFileSync } from "node:fs";

const path = "pkg/cfonts_wasm.d.ts";
const DOM_REFERENCE = '/// <reference lib="dom" />';

const lines = readFileSync(path, "utf8").split("\n");

const withoutDispose = lines.filter((line) => !line.includes("[Symbol.dispose]()"));

if (withoutDispose.length === lines.length) {
	console.warn(`patch_types: no [Symbol.dispose]() member found in ${path}`);
}

// earlier patched declarations carried a DOM lib reference instead of the rewrites
let content = withoutDispose.filter((line) => line.trim() !== DOM_REFERENCE).join("\n");

// The raw loader is internal transport: the broad `object` shapes still accept
// Node Buffers and browser URLs while naming no DOM-lib globals
// Each rewrite accepts the raw generated spelling or the already patched one, and
// throws otherwise so a wasm-bindgen output change cannot silently publish
// declarations that need the DOM lib again
const rewrites = [
	[
		"export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;",
		"export type InitInput = string | object;",
	],
	["readonly memory: WebAssembly.Memory;", "readonly memory: object;"],
	["readonly __wbindgen_externrefs: WebAssembly.Table;", "readonly __wbindgen_externrefs: object;"],
	["export type SyncInitInput = BufferSource | WebAssembly.Module;", "export type SyncInitInput = object;"],
];

for (const [raw, patched] of rewrites) {
	if (content.includes(patched)) {
		continue;
	}

	if (!content.includes(raw)) {
		throw new Error(`patch_types: expected \`${raw}\` in ${path}; the wasm-bindgen output changed`);
	}

	content = content.replace(raw, patched);
}

writeFileSync(path, content);
console.info("Files patched successfully");
