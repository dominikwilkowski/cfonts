//! Common utilities for tests

use cfonts::Rendered;

pub const ALL_FONTS: &[cfonts::Font] = &[
	cfonts::Font::Block,
	cfonts::Font::Chrome,
	cfonts::Font::Console,
	cfonts::Font::Font3D,
	cfonts::Font::Grid,
	cfonts::Font::Huge,
	cfonts::Font::Pallet,
	cfonts::Font::Shade,
	cfonts::Font::Simple3D,
	cfonts::Font::SimpleBlock,
	cfonts::Font::Slick,
	cfonts::Font::Tiny,
];

/// Runs a test without the `FORCE_SIZE` environment variable set
pub fn without_force_size<T>(test: impl FnOnce() -> T) -> T {
	temp_env::with_var("FORCE_SIZE", None::<&str>, test)
}

/// Runs a test with the `FORCE_SIZE` environment variable set to `size`
pub fn with_force_size<T>(size: usize, test: impl FnOnce() -> T) -> T {
	temp_env::with_var("FORCE_SIZE", Some(&size.to_string()), test)
}

/// The inner content of a browser render, without the wrapping div
pub fn browser_content(rendered: &Rendered) -> &str {
	let start = rendered.text.find('>').expect("wrapper div present") + 1;
	let end = rendered.text.rfind("</div>").expect("wrapper div closes");

	&rendered.text[start..end]
}
