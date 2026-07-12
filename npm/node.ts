import { readFile } from "node:fs/promises";

import init from "../pkg/cfonts_wasm.js";

const file = await readFile(new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url));

await init({
	module_or_path: new Uint8Array(file),
});

export * from "./index.js";
