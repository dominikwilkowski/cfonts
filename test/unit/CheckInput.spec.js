/***************************************************************************************************************************************************************
 *
 * CheckInput unit tests
 *
 **************************************************************************************************************************************************************/


const { CheckInput } = require('../../src/CheckInput.js');


test(`CheckInput - Should pass with correct input`, () => {
	const FONTFACES = { font1: 'font1', font2: 'font2', font3: 'font3' };
	const COLORS = { color1: 'color1', color2: 'color2', color3: 'color3' };
	const BGCOLORS = { bgcolor1: 'bgcolor1', bgcolor2: 'bgcolor2', bgcolor3: 'bgcolor3' };
	const ALIGNMENT = [ 'left', 'center', 'right' ];

	expect( CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color1','color2'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font2', ['color2', 'candy'], 'bgcolor2', 'center', false, false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['color3'], 'bgcolor3', 'right', ['#ff8800','color3'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['candy'], 'bgcolor3', 'right', ['color1','#ff8800'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['color1'], 'bgcolor2', 'right', ['color1','#ff8800', 'color1', 'color2', 'color3'], true, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
});


test(`CheckInput - Should be able to work out casing automatically`, () => {
	const FONTFACES = { font1: 'FONT1', font2: 'FONT2', font3: 'FONT3' };
	const COLORS = { color1: 'coLOr1', color2: 'coLOr2', color3: 'cOLor3' };
	const BGCOLORS = { bgcolor1: 'bGCOlor1', bgcolor2: 'bGCOlor2', bgcolor3: 'bGCOlor3' };
	const ALIGNMENT = [ 'left', 'center', 'right' ];

	expect( CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color1','color2'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font2', ['color2', 'candy'], 'bgcolor2', 'center', false, false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['color3'], 'bgcolor3', 'right', ['#ff8800','color3'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['candy'], 'bgcolor3', 'right', ['color1','#ff8800'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT ).pass ).toEqual( true );
});


test(`CheckInput - Should fail with wrong input`, () => {
	const FONTFACES = { font1: 'font1', font2: 'font2', font3: 'font3' };
	const COLORS = { color1: 'color1', color2: 'color2', color3: 'color3' };
	const BGCOLORS = { bgcolor1: 'bgcolor1', bgcolor2: 'bgcolor2', bgcolor3: 'bgcolor3' };
	const ALIGNMENT = [ 'left', 'center', 'right' ];

	const fail = CheckInput( undefined, 'font1', ['color1'], 'bgcolor1', 'left', ['color1','color2'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail.pass ).toEqual( false );
	expect( fail.message.length > 0 ).toEqual( true );

	const fail0 = CheckInput( '', 'font1', ['color1'], 'bgcolor1', 'left', false, false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail0.pass ).toEqual( false );
	expect( fail0.message.length > 0 ).toEqual( true );

	const fail1 = CheckInput( 'INPUT', 'notfound', ['color1'], 'bgcolor1', 'left', ['#ff8800','color3'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail1.pass ).toEqual( false );
	expect( fail1.message.length > 0 ).toEqual( true );

	const fail2 = CheckInput( 'INPUT', 'font1', ['notfound'], 'bgcolor1', 'left', ['color1','#ff8800'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail2.pass ).toEqual( false );
	expect( fail2.message.length > 0 ).toEqual( true );

	const fail3 = CheckInput( 'INPUT', 'font1', ['color1'], 'notfound', 'left', false, false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail3.pass ).toEqual( false );
	expect( fail3.message.length > 0 ).toEqual( true );

	const fail4 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'notfound', false, false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail4.pass ).toEqual( false );
	expect( fail4.message.length > 0 ).toEqual( true );

	const fail5 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color1'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail5.pass ).toEqual( false );
	expect( fail5.message.length > 0 ).toEqual( true );

	const fail6 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['notfound','color1'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail6.pass ).toEqual( false );
	expect( fail6.message.length > 0 ).toEqual( true );

	const fail7 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color2','#egz'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail7.pass ).toEqual( false );
	expect( fail7.message.length > 0 ).toEqual( true );

	const fail8 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color1','color2','color3'], false, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail8.pass ).toEqual( false );
	expect( fail8.message.length > 0 ).toEqual( true );

	const fail9 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', ['color1'], true, FONTFACES, COLORS, BGCOLORS, COLORS, ALIGNMENT );
	expect( fail9.pass ).toEqual( false );
	expect( fail9.message.length > 0 ).toEqual( true );
});
