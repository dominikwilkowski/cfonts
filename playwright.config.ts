import { defineConfig } from "@playwright/test";

// The three example pages the smoke test runs against, each served from the bundle `make bundle` built for it
const examples = [
	{ name: "browser", port: 4173 },
	{ name: "leptos", port: 4174 },
	{ name: "dioxus", port: 4175 },
];

export default defineConfig({
	testDir: "tests/examples",
	webServer: examples.map(({ name, port }) => ({
		command: `vite preview crates/cfonts/examples/${name} --outDir ../../../../target/${name}-example --port ${port} --strictPort --host 127.0.0.1`,
		url: `http://127.0.0.1:${port}`,
		reuseExistingServer: false,
	})),
	projects: examples.map(({ name, port }) => ({
		name,
		use: { baseURL: `http://127.0.0.1:${port}` },
	})),
});
