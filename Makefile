# The examples, each run with the tools its own platform needs
#
# The Rust examples need cargo alone
# the two framework examples add trunk and the wasm32 target
# the topcoat site is a standalone package, cargo alone on Rust 1.98
# the JavaScript examples build the npm package first and need node and pnpm for that

.DEFAULT_GOAL := help
.PHONY: help cli ratatui leptos dioxus topcoat node browser bundle check test

help:
	@echo "make cli       the Rust API tour, printed to the terminal"
	@echo "make node      the node API tour, printed to the terminal"
	@echo "               or: pnpm run example:node"
	@echo "make ratatui   the ratatui widget, an interactive terminal app"
	@echo "make leptos    the leptos site, served by trunk"
	@echo "make dioxus    the dioxus site, served by trunk"
	@echo "make topcoat   the topcoat site, served by its own binary"
	@echo "make browser   the browser example, served by vite"
	@echo "               or: pnpm run example:browser"
	@echo "make bundle    the browser, leptos and dioxus pages the smoke test serves, after pnpm run build"
	@echo "make check     compiles the leptos and dioxus examples"
	@echo "make test      the whole gate, every check and every test"

cli:
	cargo run --locked -p cfonts --example cli

node: package
	node crates/cfonts/examples/node.js

ratatui:
	cargo run --locked -p cfonts --example ratatui --features ratatui

leptos: trunk
	cd crates/cfonts/examples/leptos && trunk serve

dioxus: trunk
	cd crates/cfonts/examples/dioxus && trunk serve

topcoat:
	cargo run --locked --manifest-path crates/cfonts/examples/topcoat/Cargo.toml

browser: package
	pnpm exec vite crates/cfonts/examples/browser

# The browser page imports the npm package, which `pnpm run build` leaves in pkg and dist,
# and test:package builds it right before the built tests run, so bundle takes it as is instead of building it again
bundle: trunk
	pnpm exec vite build crates/cfonts/examples/browser --base ./ --outDir ../../../../target/browser-example --emptyOutDir
	cd crates/cfonts/examples/leptos && trunk build --locked --dist ../../../../target/leptos-example
	cd crates/cfonts/examples/dioxus && trunk build --locked --dist ../../../../target/dioxus-example

check: wasm32
	cargo check --locked --manifest-path crates/cfonts/examples/leptos/Cargo.toml --target wasm32-unknown-unknown
	cargo check --locked --manifest-path crates/cfonts/examples/dioxus/Cargo.toml --target wasm32-unknown-unknown

# A cfg gated on one feature hides behind --all-features, so every feature compiles alone, on both targets,
# and pnpm test runs the cargo tests, make check, the wasm tests and the package suite with the smoke test of the three example pages
test: wasm32
	cargo fmt --all -- --check
	cargo clippy --locked --workspace --all-features --tests --all-targets -- -D warnings
	cargo check --locked --workspace --all-targets --all-features --release
	cargo check --locked -p cfonts --features web
	cargo check --locked -p cfonts --features ratatui
	cargo check --locked -p cfonts --features leptos
	cargo check --locked -p cfonts --features dioxus
	cargo check --locked -p cfonts --features wasm
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features --features web
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features --features ratatui
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features --features leptos
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features --features dioxus
	cargo check --locked -p cfonts --lib --target wasm32-unknown-unknown --no-default-features --features wasm
	pnpm test

# The framework examples compile for the browser, add the target with `rustup target add wasm32-unknown-unknown`
# and install trunk once with `cargo install trunk` to serve them
.PHONY: wasm32 trunk package
wasm32:
	@rustup target list --installed | grep -q wasm32-unknown-unknown || { echo "the wasm32 target is missing, run: rustup target add wasm32-unknown-unknown"; exit 1; }

trunk: wasm32
	@command -v trunk > /dev/null || { echo "trunk is not installed, run: cargo install trunk"; exit 1; }

# The JavaScript examples import the npm package, which is built from the wasm crate
package:
	pnpm install --frozen-lockfile
	pnpm run build
