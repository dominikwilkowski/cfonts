import { defineConfig } from "@playwright/test";

export default defineConfig({
	testDir: "tests/browser",
	webServer: {
		command:
			"vite preview crates/cfonts/examples/browser --outDir ../../../../target/browser-example --port 4173 --strictPort",
		port: 4173,
		reuseExistingServer: false,
	},
	use: {
		baseURL: "http://localhost:4173",
	},
});
