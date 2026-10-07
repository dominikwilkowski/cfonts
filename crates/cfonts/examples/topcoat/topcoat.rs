//! A cfonts component for [topcoat](https://github.com/tokio-rs/topcoat), the server rendered web framework of the tokio org
//!
//! Topcoat renders a page on the server and ships HTML, so the core runs natively here and the component
//! hands the page the HTML the browser environment renders, the way the Leptos and Dioxus components do in
//! the browser. Topcoat is still early stage and not ready for a component yet, once it is the crate can ship one
//!
//! Run it with `make topcoat` from the repository root, or with the command behind it:
//!
//! ```sh
//! cargo run --manifest-path crates/cfonts/examples/topcoat/Cargo.toml
//! ```
//!
//! then open the address it prints, http://127.0.0.1:3000 unless `HOST` or `PORT` say otherwise. It builds once
//! and serves, so after a change stop it and run it again

use std::{env, io};

use cfonts::{BrowserEnv, BrowserHost, Cfonts, Color, Font, GradientOption, Host, Options, RenderOverrides};
use tokio::net::TcpListener;
use topcoat::{
	Result,
	router::{module_router, page},
	view::{Unescaped, View, component, view},
};

/// The cfonts topcoat component
///
/// The artifact paints in true color and never wraps unless the overrides say otherwise:
/// constrain and place it with your own page styles, or hand it a column count
///
/// Candy rolls a fresh pick per request through the host, a seed in the overrides pins the picks
#[component]
pub async fn cfonts_topcoat(options: &Options, #[default] overrides: RenderOverrides) -> Result<impl View> {
	let rendered = BrowserHost::from_overrides(overrides).render(&BrowserEnv, options);

	Ok(view! { (Unescaped::new_unchecked(rendered.text)) })
}

/// The page at `/`: one banner the page sizes and one the overrides wrap at forty columns
#[page]
async fn home() -> Result<impl View> {
	let banner: Options = Cfonts::text("topcoat")
		.font(Font::Block)
		.colors(GradientOption::TwoStop { start: Color::RED, end: Color::BLUE })
		.into();
	let wrapped: Options =
		Cfonts::text("server rendered").font(Font::Thin).colors(vec![Color::CANDY, Color::CANDY, Color::CANDY]).into();

	Ok(view! {
		<!DOCTYPE html>
		<html>
			<head>
				<meta charset="utf-8">
				<title>"cfonts in topcoat"</title>
			</head>
			<body style="background:#111318;color:#fff;margin:2rem">
				cfonts_topcoat(options: &banner)
				cfonts_topcoat(options: &wrapped, overrides: RenderOverrides::default().with_canvas_width(40))
			</body>
		</html>
	})
}

/// Binds the address topcoat would bind, `HOST` and `PORT` with their defaults, and says where it listens
#[tokio::main]
async fn main() -> io::Result<()> {
	let host = env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1"));
	let port = env::var("PORT").map_or(Ok(3000), |port| port.parse::<u16>()).map_err(io::Error::other)?;
	let listener = TcpListener::bind((host.as_str(), port)).await?;
	println!("cfonts topcoat example serves on http://{}", listener.local_addr()?);

	topcoat::serve(listener, module_router!().build()).await
}
