use crate::fonts::Font;

pub const FONT_BLOCK: Font<6> = Font {
	name: "block",
	version: "1.0.0",
	#[rustfmt::skip]
	buffer: [
		"",
		"",
		"",
		"",
		"",
		"",
	],
	#[rustfmt::skip]
	letterspace: [
		" ",
		" ",
		" ",
		" ",
		" ",
		" ",
	],
	letterspace_size: 1,
	colors: 2,
	homepage: "https://github.com/dominikwilkowski/cfonts",
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(&[
			r" <c1>█████</c1><c2>╗</c2> ",
			r"<c1>██</c1><c2>╔══</c2><c1>██</c1><c2>╗</c2>",
			r"<c1>███████</c1><c2>║</c2>",
			r"<c1>██</c1><c2>╔══</c2><c1>██</c1><c2>║</c2>",
			r"<c1>██</c1><c2>║</c2><c1>  ██</c1><c2>║</c2>",
			r"<c2>╚═╝  ╚═╝</c2>",
		]);
		table['B' as usize] = Some(&[
			r"<c1>██████</c1><c2>╗ </c2>",
			r"<c1>██</c1><c2>╔══</c2><c1>██</c1><c2>╗</c2>",
			r"<c1>██████</c1><c2>╔╝</c2>",
			r"<c1>██</c1><c2>╔══</c2><c1>██</c1><c2>╗</c2>",
			r"<c1>██████</c1><c2>╔╝</c2>",
			r"<c2>╚═════╝ </c2>",
		]);

		table
	},
};
