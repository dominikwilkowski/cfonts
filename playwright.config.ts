import { defineConfig } from "@playwright/test";

export default defineConfig({
	testDir: "tests/browser",
	webServer: [
		{
			command:
				"vite preview crates/cfonts/examples/browser --outDir ../../../../target/browser-example --port 4173 --strictPort --host 127.0.0.1",
			url: "http://127.0.0.1:4173",
			reuseExistingServer: false,
		},
	],
	use: {
		baseURL: "http://127.0.0.1:4173",
	},
});
