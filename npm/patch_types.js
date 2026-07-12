// Strips the `[Symbol.dispose]()` member from the generated wasm types:
// it requires `lib: esnext` and would break type checking for consumers
// on any stable target (verified failing on es2022 through es2024)
import { readFileSync, writeFileSync } from "node:fs";

const path = "pkg/cfonts_wasm.d.ts";
const lines = readFileSync(path, "utf8").split("\n");
const patched = lines.filter((line) => !line.includes("[Symbol.dispose]()"));

if (patched.length === lines.length) {
	console.warn(`patch_types: no [Symbol.dispose]() member found in ${path}`);
}

writeFileSync(path, patched.join("\n"));
console.info("Files patched successfully");
