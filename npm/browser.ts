import init from "../pkg/cfonts_wasm.js";

await init({
	module_or_path: new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url),
});

export * from "./index.js";
