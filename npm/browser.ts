import init from "../pkg/cfonts_wasm.js";

// Browsers let wasm-bindgen fetch the module URL asynchronously
await init({
	module_or_path: new URL("../pkg/cfonts_wasm_bg.wasm", import.meta.url),
});

export { BrowserHost } from "./hosts/browser.js";
export * from "./index.js";
