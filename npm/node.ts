import { readFileSync } from "node:fs";

import { initSync } from "../pkg/cfonts_wasm.js";

initSync({ module: readFileSync(new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url)) });

export * from "./index.js";
