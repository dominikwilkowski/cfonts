use crate::fonts::{FontFile, Glyph, GlyphRow, Segment};
use cfonts_macros::glyph;

pub static FONT_BOARD: FontFile<7> = FontFile {
	name: "board",
	#[rustfmt::skip]
	buffer_start: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	#[rustfmt::skip]
	buffer_end: &[
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
		GlyphRow { segments: &[Segment::Plain("")] },
	],
	buffer_size: 0,
	#[rustfmt::skip]
	letter_space: glyph!(
		r" ",
		r"<c3>═</c3>",
		r"<c4>░</c4>",
		r"<c4>▒</c4>",
		r"<c4>▓</c4>",
		r"<c3>═</c3>",
		r" ",
	),
	colors: 4,
	line_height: 1,
	#[rustfmt::skip]
	glyphs: {
		let mut table = [None; 128];
		table['A' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘ │</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕ │</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['B' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘╒╛</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕└┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['C' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>╘═╛</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>┌─┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['D' as usize] = Some(glyph!(
			r"<c1> ┌──────┐ </c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕└┐</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙    ∙</c2><c1>╒╛</c1>",
			r"<c1> ╘══════╛ </c1>",
		));
		table['E' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └┐╘═╛</c1>",
			r"<c4>▒</c4><c1>│   ╒╛┌─┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['F' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └┐╘═╛</c1>",
			r"<c4>▒</c4><c1>│   ╒╛</c1><c4>▒▒▒</c4>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓▓▓▓</c4>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>════</c3>",
			r"<c1> ╘═══╛    </c1>",
		));
		table['G' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═══╛</c1>",
			r"<c4>░</c4><c1>│   │┌──┐</c1>",
			r"<c4>▒</c4><c1>│   │╘╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['H' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘ │</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕ │</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['I' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙</c2><c1>  ╒═╛</c1>",
			r"<c4>░░░</c4><c1>│   │</c1><c4>░░</c4>",
			r"<c4>▒▒▒</c4><c1>│   │</c1><c4>▒▒</c4>",
			r"<c4>▓▓▓</c4><c1>│   │</c1><c4>▓▓</c4>",
			r"<c3>═</c3><c1>┌─┘  </c1><c2>∙</c2><c1>└─┐</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['J' as usize] = Some(glyph!(
			r"<c1>       ┌─┐</c1>",
			r"<c3>═══════</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░░░░░░░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>┌───┐</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│</c1><c2>∙</c2><c1>  └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['K' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘╒╛</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕└┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['L' as usize] = Some(glyph!(
			r"<c1> ┌───┐    </c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>════</c3>",
			r"<c4>░</c4><c1>│   │</c1><c4>░░░░</c4>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>┌─┐</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['M' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1> ╒╕╒╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│  │╘╛│ │</c1>",
			r"<c4>▒</c4><c1>│  │</c1><c4>▒▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│  │</c1><c4>▓▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1> │</c1><c3>══</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘══╛  ╘═╛</c1>",
		));
		table['N' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['O' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['P' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘ │</c1>",
			r"<c4>▒</c4><c1>│   ╒═══╛</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓▓▓▓</c4>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>════</c3>",
			r"<c1> ╘═══╛    </c1>",
		));
		table['Q' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>    ┌└┐</c1>",
			r"<c1> ╘═════╛═╛</c1>",
		));
		table['R' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘╒╛</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕└┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['S' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═════╛</c1>",
			r"<c4>░</c4><c1>│ └─────┐</c1>",
			r"<c4>▒</c4><c1>╘═══╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>▓</c4><c1>┌───┘   │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['T' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙ ∙</c2><c1>╒═╛</c1>",
			r"<c4>░░░</c4><c1>│   │</c1><c4>░░</c4>",
			r"<c4>▒▒▒</c4><c1>│   │</c1><c4>▒▒</c4>",
			r"<c4>▓▓▓</c4><c1>│   │</c1><c4>▓▓</c4>",
			r"<c3>═══</c3><c1>│</c1><c2>∙ ∙</c2><c1>│</c1><c3>══</c3>",
			r"<c1>   ╘═══╛  </c1>",
		));
		table['U' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['V' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>╘╕  </c1><c2>∙</c2><c1>  ╒╛</c1>",
			r"<c1>  ╘═════╛ </c1>",
		));
		table['W' as usize] = Some(glyph!(
			r"<c1> ┌──┐  ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1> │</c1><c3>══</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│  │</c1><c4>░░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│  │┌┐│ │</c1>",
			r"<c4>▓</c4><c1>│  └┘└┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['X' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>╘╕  └─┘╒╛</c1>",
			r"<c4>▒</c4><c1>┌┘  ╒═╕└┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══╛ ╘═╛</c1>",
		));
		table['Y' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘ │</c1>",
			r"<c4>▒</c4><c1>╘═════╕ │</c1>",
			r"<c4>▓</c4><c1>┌─────┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['Z' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘═════╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>┌─────┘ │</c1>",
			r"<c4>▒</c4><c1>│</c1><c2>∙</c2><c1>  ╒═══╛</c1>",
			r"<c4>▓</c4><c1>│   └───┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['0' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░</c4><c1>│ │</c1>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒</c4><c1>│ │</c1>",
			r"<c4>▓</c4><c1>│   └─┘ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['1' as usize] = Some(glyph!(
			r"<c1> ┌─────┐  </c1>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙</c2><c1>  │</c1><c3>══</c3>",
			r"<c4>░░░</c4><c1>│   │</c1><c4>░░</c4>",
			r"<c4>▒▒▒</c4><c1>│   │</c1><c4>▒▒</c4>",
			r"<c4>▓▓▓</c4><c1>│   │</c1><c4>▓▓</c4>",
			r"<c3>═</c3><c1>┌─┘</c1><c2>∙</c2><c1>  └─┐</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['2' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘═════╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>┌─────┘ │</c1>",
			r"<c4>▒</c4><c1>│</c1><c2>∙</c2><c1>  ╒═══╛</c1>",
			r"<c4>▓</c4><c1>│   └───┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['3' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>╘═╛┌┘   │</c1>",
			r"<c4>▒</c4><c1>┌─┐╘╕   │</c1>",
			r"<c4>▓</c4><c1>│ │</c1><c4>▓</c4><c1>│   │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>└─┘  </c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['4' as usize] = Some(glyph!(
			r"<c1> ┌───┐ ┌─┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  │</c1><c3>═</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   └─┘ │</c1>",
			r"<c4>▒</c4><c1>╘═════╕ │</c1>",
			r"<c4>▓▓▓▓▓▓▓</c4><c1>│ │</c1>",
			r"<c3>═══════</c3><c1>│</c1><c2>∙</c2><c1>│</c1>",
			r"<c1>       ╘═╛</c1>",
		));
		table['5' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═════╛</c1>",
			r"<c4>░</c4><c1>│ └─────┐</c1>",
			r"<c4>▒</c4><c1>╘═══╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>▓</c4><c1>┌───┘   │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙     ∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['6' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═══╛</c1>",
			r"<c4>░</c4><c1>│   └───┐</c1>",
			r"<c4>▒</c4><c1>│   ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['7' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>╘═╛┌┘   │</c1>",
			r"<c4>▒▒▒▒</c4><c1>╘╕   │</c1>",
			r"<c4>▓▓▓▓▓</c4><c1>│   │</c1>",
			r"<c3>═════</c3><c1>│  </c1><c2>∙</c2><c1>│</c1>",
			r"<c1>     ╘═══╛</c1>",
		));
		table['8' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>╘╕  └─┘╒╛</c1>",
			r"<c4>▒</c4><c1>┌┘  ╒═╕└┐</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓</c4><c1>│ │</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─┘</c1><c2>∙</c2><c1>│</c1>",
			r"<c1> ╘═══════╛</c1>",
		));
		table['9' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│ └─┘   │</c1>",
			r"<c4>▒</c4><c1>╘═══╕   │</c1>",
			r"<c4>▓▓▓▓▓</c4><c1>│   │</c1>",
			r"<c3>═════</c3><c1>│  </c1><c2>∙</c2><c1>│</c1>",
			r"<c1>     ╘═══╛</c1>",
		));
		table['!' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙</c2><c1>  ╒═╛</c1>",
			r"<c4>░░░</c4><c1>│   │</c1><c4>░░</c4>",
			r"<c4>▒▒▒</c4><c1>│  </c1><c2>∙</c2><c1>│</c1><c4>▒▒</c4>",
			r"<c4>▓▓▓</c4><c1>╘═══╛</c1><c4>▓▓</c4>",
			r"<c3>═══</c3><c1>┌───┐</c1><c3>══</c3>",
			r"<c1>   ╘═══╛  </c1>",
		));
		table['?' as usize] = Some(glyph!(
			r"<c1> ┌───────┐</c1>",
			r"<c3>═</c3><c1>╘══╕   </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>┌──┘ ╒══╛</c1>",
			r"<c4>▒</c4><c1>│</c1><c2>∙</c2><c1>   └──┐</c1>",
			r"<c4>▓</c4><c1>╘═══════╛</c1>",
			r"<c3>════</c3><c1>┌──┐</c1><c3>══</c3>",
			r"<c1>    ╘══╛  </c1>",
		));
		table['.' as usize] = Some(glyph!(
			r"      ",
			r"<c3>══════</c3>",
			r"<c4>░░░░░░</c4>",
			r"<c4>▒▒▒▒▒▒</c4>",
			r"<c4>▓▓▓▓▓▓</c4>",
			r"<c3>═</c3><c1>┌──┐</c1><c3>═</c3>",
			r"<c1> ╘══╛ </c1>",
		));
		table['+' as usize] = Some(glyph!(
			r"          ",
			r"<c3>═══</c3><c1>┌──┐</c1><c3>═══</c3>",
			r"<c4>░</c4><c1>┌─┘</c1><c2>∙</c2><c1> └─┐</c1><c4>░</c4>",
			r"<c4>▒</c4><c1>╘═╕ </c1><c2>∙</c2><c1>╒═╛</c1><c4>▒</c4>",
			r"<c4>▓▓▓</c4><c1>╘══╛</c1><c4>▓▓▓</c4>",
			r"<c3>══════════</c3>",
			r"          ",
		));
		table['-' as usize] = Some(glyph!(
			r"        ",
			r"<c3>════════</c3>",
			r"<c4>░</c4><c1>┌─────┐</c1>",
			r"<c4>▒</c4><c1>╘═════╛</c1>",
			r"<c4>▓▓▓▓▓▓▓▓</c4>",
			r"<c3>════════</c3>",
			r"        ",
		));
		table['_' as usize] = Some(glyph!(
			r"        ",
			r"<c3>════════</c3>",
			r"<c4>░░░░░░░░</c4>",
			r"<c4>▒▒▒▒▒▒▒▒</c4>",
			r"<c4>▓▓▓▓▓▓▓▓</c4>",
			r"<c3>═</c3><c1>┌─────┐</c1>",
			r"<c1> ╘═════╛</c1>",
		));
		table['=' as usize] = Some(glyph!(
			r"         ",
			r"<c3>═════════</c3>",
			r"<c4>░</c4><c1>┌──────┐</c1>",
			r"<c4>▒</c4><c1>╘══════╛</c1>",
			r"<c4>▓</c4><c1>┌──────┐</c1>",
			r"<c3>═</c3><c1>╘══════╛</c1>",
			r"         ",
		));
		table['@' as usize] = Some(glyph!(
			r"<c1> ┌─────────┐</c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  ╒══╕ </c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░</c4><c1>│   │┌─┘  │</c1>",
			r"<c4>▒</c4><c1>│   │╘════╛</c1>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓▓▓▓▓▓</c4>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>  └─────┐</c1>",
			r"<c1> ╘═════════╛</c1>",
		));
		table['#' as usize] = Some(glyph!(
			r"<c1>  ┌─┐┌─┐  </c1>",
			r"<c1>┌─┘</c1><c2>∙</c2><c1>└┘</c1><c2>∙</c2><c1>└─┐</c1>",
			r"<c1>╘═╕ ╒╕ ╒═╛</c1>",
			r"<c1>┌─┘ └┘ └─┐</c1>",
			r"<c1>╘═╕</c1><c2>∙</c2><c1>╒╕</c1><c2>∙</c2><c1>╒═╛</c1>",
			r"<c3>══</c3><c1>╘═╛╘═╛</c1><c3>══</c3>",
			r"          ",
		));
		table['$' as usize] = Some(glyph!(
			r"<c1>    ┌─┐   </c1>",
			r"<c3>═</c3><c1>┌──┘ └──┐</c1>",
			r"<c4>░</c4><c1>│</c1><c2>∙</c2><c1>╒═════╛</c1>",
			r"<c4>▒</c4><c1>│ └─────┐</c1>",
			r"<c4>▓</c4><c1>╘═══╕  </c1><c2>∙</c2><c1>│</c1>",
			r"<c3>═</c3><c1>┌───┘</c1><c2>∙</c2><c1>  │</c1>",
			r"<c1> ╘══╕_╒══╛</c1>",
		));
		table['%' as usize] = Some(glyph!(
			r"<c1> ┌─┐   ┌─┐ </c1>",
			r"<c3>═</c3><c1>╘═╛</c1><c3>══</c3><c1>┌┘</c1><c2>∙</c2><c1>│</c1><c3>═</c3>",
			r"<c4>░░░░</c4><c1>┌┘ ╒╛</c1><c4>░░</c4>",
			r"<c4>▒▒▒</c4><c1>┌┘ ╒╛</c1><c4>▒▒▒</c4>",
			r"<c4>▓▓</c4><c1>┌┘ ╒╛</c1><c4>▓▓▓▓</c4>",
			r"<c1>┌─┘</c1><c2>∙</c2><c1>╒╛</c1><c3>═</c3><c1>┌─┐</c1><c3>═</c3>",
			r"<c1>╘═══╛  ╘═╛ </c1>",
		));
		table['&' as usize] = Some(glyph!(
			r"<c1>     ┌──┐  </c1>",
			r"<c3>═════</c3><c1>│ </c1><c2>∙</c2><c1>│</c1><c3>══</c3>",
			r"<c4>░</c4><c1>┌───────┐</c1><c4>░</c4>",
			r"<c4>▒</c4><c1>│</c1><c2>∙</c2><c1> ╒════╛</c1><c4>▒</c4>",
			r"<c4>▓</c4><c1>│  └┘  │</c1><c4>▓▓</c4>",
			r"<c3>═</c3><c1>│</c1><c2>∙    ∙</c2><c1>│</c1><c3>══</c3>",
			r"<c1> ╘══════╛  </c1>",
		));
		table['(' as usize] = Some(glyph!(
			r"<c1>  ┌──────┐</c1>",
			r"<c3>═</c3><c1>┌┘</c1><c2>∙</c2><c1> ╒═══╛</c1>",
			r"<c4>░</c4><c1>│   │</c1><c4>░░░░</c4>",
			r"<c4>▒</c4><c1>│   │</c1><c4>▒▒▒▒</c4>",
			r"<c4>▓</c4><c1>│   │</c1><c4>▓▓▓▓</c4>",
			r"<c3>═</c3><c1>╘╕ </c1><c2>∙</c2><c1>└───┐</c1>",
			r"<c1>  ╘══════╛</c1>",
		));
		table[')' as usize] = Some(glyph!(
			r"<c1> ┌──────┐ </c1>",
			r"<c3>═</c3><c1>╘═══╕</c1><c2>∙</c2><c1> └┐</c1>",
			r"<c4>░░░░░</c4><c1>│   │</c1>",
			r"<c4>▒▒▒▒▒</c4><c1>│   │</c1>",
			r"<c4>▓▓▓▓▓</c4><c1>│   │</c1>",
			r"<c3>═</c3><c1>┌───┘ </c1><c2>∙</c2><c1>╒╛</c1>",
			r"<c1> ╘══════╛ </c1>",
		));
		table['/' as usize] = Some(glyph!(
			r"<c1>      ┌─┐ </c1>",
			r"<c3>═════</c3><c1>┌┘</c1><c2>∙</c2><c1>│</c1><c3>═</c3>",
			r"<c4>░░░░</c4><c1>┌┘ ╒╛</c1><c4>░</c4>",
			r"<c4>▒▒▒</c4><c1>┌┘ ╒╛</c1><c4>▒▒</c4>",
			r"<c4>▓▓</c4><c1>┌┘ ╒╛</c1><c4>▓▓▓</c4>",
			r"<c1>┌─┘</c1><c2>∙</c2><c1>╒╛</c1><c3>════</c3>",
			r"<c1>╘═══╛     </c1>",
		));
		table[':' as usize] = Some(glyph!(
			r"       ",
			r"<c3>═══════</c3>",
			r"<c4>░</c4><c1>┌───┐</c1><c4>░</c4>",
			r"<c4>▒</c4><c1>╘═══╛</c1><c4>▒</c4>",
			r"<c4>▓</c4><c1>┌───┐</c1><c4>▓</c4>",
			r"<c3>═</c3><c1>╘═══╛</c1><c3>═</c3>",
			r"       ",
		));
		table[';' as usize] = Some(glyph!(
			r"       ",
			r"<c3>═══════</c3>",
			r"<c4>░</c4><c1>┌───┐</c1><c4>░</c4>",
			r"<c4>▒</c4><c1>╘═══╛</c1><c4>▒</c4>",
			r"<c4>▓</c4><c1>┌───┐</c1><c4>▓</c4>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙</c2><c1>│</c1><c3>═</c3>",
			r"<c1>   ╘═╛ </c1>",
		));
		table[',' as usize] = Some(glyph!(
			r"       ",
			r"<c3>═══════</c3>",
			r"<c4>░░░░░░░</c4>",
			r"<c4>▒▒▒▒▒▒▒</c4>",
			r"<c4>▓</c4><c1>┌───┐</c1><c4>▓</c4>",
			r"<c3>═</c3><c1>╘═╕</c1><c2>∙</c2><c1>│</c1><c3>═</c3>",
			r"<c1>   ╘═╛ </c1>",
		));
		table['\'' as usize] = Some(glyph!(
			r"<c1> ┌───┐ </c1>",
			r"<c3>═</c3><c1>│</c1><c2>∙</c2><c1>╒═╛</c1><c3>═</c3>",
			r"<c4>░</c4><c1>╘═╛</c1><c4>░░░</c4>",
			r"<c4>▒▒▒▒▒▒▒</c4>",
			r"<c4>▓▓▓▓▓▓▓</c4>",
			r"<c3>═══════</c3>",
			r"       ",
		));
		table['"' as usize] = Some(glyph!(
			r"<c1>┌───┐┌───┐</c1>",
			r"<c1>╘═╕</c1><c2>∙</c2><c1>│╘═╕</c1><c2>∙</c2><c1>│</c1>",
			r"<c4>░░</c4><c1>╘═╛</c1><c4>░░</c4><c1>╘═╛</c1>",
			r"<c4>▒▒▒▒▒▒▒▒▒▒</c4>",
			r"<c4>▓▓▓▓▓▓▓▓▓▓</c4>",
			r"<c3>══════════</c3>",
			r"          ",
		));
		table[' ' as usize] = Some(glyph!(
			r"  ",
			r"<c3>══</c3>",
			r"<c4>░░</c4>",
			r"<c4>▒▒</c4>",
			r"<c4>▓▓</c4>",
			r"<c3>══</c3>",
			r"  ",
		));

		table
	},
};

#[cfg(test)]
mod tests {
	use crate::fonts::{
		Segment,
		tests::{
			assert_buffer_end_size, assert_buffer_start_size, assert_buffers_complementary, assert_buffers_plain,
			assert_colors_all_used, assert_plain_segments_are_spaces, assert_supported, glyph_name,
		},
	};

	#[test]
	fn all_supported_glyphs_defined() {
		assert_supported(&super::FONT_BOARD);
	}

	#[test]
	fn colors_all_used() {
		assert_colors_all_used(&super::FONT_BOARD);
	}

	#[test]
	fn plain_segments_are_spaces() {
		assert_plain_segments_are_spaces(&super::FONT_BOARD);
	}

	#[test]
	fn buffer_start_size() {
		assert_buffer_start_size(&super::FONT_BOARD);
	}

	#[test]
	fn buffer_end_size() {
		assert_buffer_end_size(&super::FONT_BOARD);
	}

	#[test]
	fn buffers_plain() {
		assert_buffers_plain(&super::FONT_BOARD);
	}

	#[test]
	fn buffers_complementary() {
		assert_buffers_complementary(&super::FONT_BOARD);
	}

	#[test]
	fn pieces_keep_their_colors() {
		// dots paint <c2>, shades <c4>, the line piece <c3> or a box frame
		// <c1>, and every other character the base <c1> — in both directions:
		// a slot painting a foreign character fails the same row
		let font = &super::FONT_BOARD;

		for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate() {
			let Some(glyph) = glyph else {
				continue;
			};

			for (line, row) in glyph.rows.iter().enumerate() {
				for segment in row.segments {
					let Segment::Colored { slot, text } = segment else {
						continue;
					};

					for character in text.chars().filter(|character| *character != ' ') {
						let allowed: &[usize] = match character {
							'∙' => &[1],
							'░' | '▒' | '▓' => &[3],
							'═' => &[0, 2],
							_ => &[0],
						};

						assert!(
							allowed.contains(slot),
							"glyph {} (line {}) paints {:?} with <c{}>",
							glyph_name(font, code_point),
							line,
							character,
							slot + 1,
						);
					}
				}
			}
		}
	}

	#[test]
	fn line_pieces_never_hide_in_the_base_color() {
		// a lone ═ in the base color is almost always a line piece missing its
		// <c3>: box frames always carry a corner in the same segment
		let font = &super::FONT_BOARD;

		for (code_point, glyph) in font.glyphs.iter().copied().chain(std::iter::once(Some(font.letter_space))).enumerate() {
			let Some(glyph) = glyph else {
				continue;
			};

			for (line, row) in glyph.rows.iter().enumerate() {
				for segment in row.segments {
					if let Segment::Colored { slot: 0, text } = segment
						&& text.contains('═')
					{
						assert!(
							text.chars().any(|character| matches!(character, '╒' | '╕' | '╘' | '╛')),
							"glyph {} (line {}) paints \u{2550} with <c1> but no frame corner sits beside it; a lone line is <c3>",
							glyph_name(font, code_point),
							line,
						);
					}
				}
			}
		}
	}
}
