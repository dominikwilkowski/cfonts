import { readFileSync } from "node:fs";

// window-size's main export is a load-time snapshot that is undefined in pipes;
// its get() in utils always detects fresh, per render
import windowSize from "window-size/utils.js";

import { initSync } from "../pkg/cfonts_wasm.js";
import { entry } from "./entry.js";

initSync({ module: readFileSync(new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url)) });

entry.width = () => windowSize.get()?.width;
entry.sayRender = (banner) => banner.renderCli();

export * from "./index.js";
