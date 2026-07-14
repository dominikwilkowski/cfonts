import { readFileSync } from "node:fs";

// window-size's main export is a load-time snapshot that is undefined in pipes;
// its get() in utils always detects fresh, per render
import windowSize from "window-size/utils.js";

import { initSync } from "../pkg/cfonts_wasm.js";
import { detection } from "./detection.js";

initSync({ module: readFileSync(new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url)) });

detection.width = () => windowSize.get()?.width;

export * from "./index.js";
