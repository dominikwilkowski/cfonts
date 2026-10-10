# The examples, each run with the tools its own platform needs
#
# The Rust examples need cargo alone
# the two framework examples add trunk and the wasm32 target
# the topcoat site is a standalone package, cargo alone on Rust 1.98
# the JavaScript examples build the npm package first and need node and pnpm for that

.DEFAULT_GOAL := help
.PHONY: help cli ratatui leptos dioxus topcoat node browser bundle check test perf perf-save compare

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
	@echo "make check     compiles the ratatui, leptos and dioxus examples"
	@echo "make test      the whole gate, every check and every test"
	@echo "make perf      v4 on speed, allocations and memory, prints what changed against perf/README.md"
	@echo "make perf-save v4 on speed, allocations and memory, the results go into perf/README.md"
	@echo "make compare   v4 against the published v3 1.3.0, the comparison goes into perf/README.md"

cli:
	cargo run --locked -p cfonts --example cli

node: package
	node crates/cfonts/examples/node.js

ratatui:
	cargo run --locked --manifest-path crates/cfonts/examples/ratatui/Cargo.toml

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
	cargo check --locked --manifest-path crates/cfonts/examples/ratatui/Cargo.toml
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

# The perf package measures this v4 and compares it with the published v3 on request, its lib.rs lists the scenarios
# and how they run, its README.md holds the hand written part and the tables perf-save and compare write into it,
# perf reads the results back and prints what changed
# the v3 binary installs once into perf/target/v3 with a build directory of its own, so it never lands on the v4 binary,
# the other paths follow CARGO_TARGET_DIR the way cargo does
PERF_V3 = $(CURDIR)/perf/target/v3/bin/cfonts
PERF_V4 = $(abspath $(or $(CARGO_TARGET_DIR),target))/release/cfonts
PERF_TARGET = $(abspath $(or $(CARGO_TARGET_DIR),perf/target))/release
PERF_INTERPOSE = $(PERF_TARGET)/libcfonts_perf_interpose.$(if $(filter Darwin,$(shell uname -s)),dylib,so)
PERF_RUN = CFONTS_PERF_V4=$(PERF_V4) CFONTS_PERF_INTERPOSE=$(PERF_INTERPOSE) $(PERF_TARGET)/cfonts-perf

perf: perf-build
	$(PERF_RUN) perf

perf-save: perf-build
	$(PERF_RUN) save

compare: perf-build
	test -x $(PERF_V3) || cargo install cfonts --version =1.3.0 --locked --root perf/target/v3 --target-dir perf/target/v3/build
	CFONTS_PERF_V3=$(PERF_V3) $(PERF_RUN) compare

# the v4 binary, then the runner and the interpose library in one build of the perf workspace
.PHONY: perf-build
perf-build:
	cargo build --locked --release -p cfonts --bin cfonts
	cargo build --locked --release --manifest-path perf/Cargo.toml --workspace

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
