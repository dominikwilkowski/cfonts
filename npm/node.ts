import { readFileSync } from "node:fs";

import { initSync } from "../pkg/cfonts_wasm.js";

// The web-target WASM loader cannot read file URLs in Node so initialize it from bytes
initSync({
	module: readFileSync(new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url)),
});

export { NodeHost } from "./hosts/node.js";
export * from "./index.js";
