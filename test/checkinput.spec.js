/***************************************************************************************************************************************************************
 *
 * CheckInput unit tests
 *
 **************************************************************************************************************************************************************/


const CFonts = require('../src/lib.js')
const CheckInput = CFonts.__test__.CheckInput;


test(`CheckInput - Should pass with correct input`, () => {
	const FONTFACES = [ 'font1', 'font2', 'font3' ];
	const COLORS = [ 'color1', 'color2', 'color3' ];
	const BGCOLORS = [ 'bgcolor1', 'bgcolor2', 'bgcolor3' ];
	const ALIGNMENT = [ 'left', 'center', 'right' ];

	expect( CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font2', ['color2', 'candy'], 'bgcolor2', 'center', FONTFACES, COLORS, BGCOLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['color3'], 'bgcolor3', 'right', FONTFACES, COLORS, BGCOLORS, ALIGNMENT ).pass ).toEqual( true );
	expect( CheckInput( 'INPUT', 'font3', ['candy'], 'bgcolor3', 'right', FONTFACES, COLORS, BGCOLORS, ALIGNMENT ).pass ).toEqual( true );
});

test(`CheckInput - Should fail with wrong input`, () => {
	const FONTFACES = [ 'font1', 'font2', 'font3' ];
	const COLORS = [ 'color1', 'color2', 'color3' ];
	const BGCOLORS = [ 'bgcolor1', 'bgcolor2', 'bgcolor3' ];
	const ALIGNMENT = [ 'left', 'center', 'right' ];

	const fail = CheckInput( undefined, 'font1', ['color1'], 'bgcolor1', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail.pass ).toEqual( false );
	expect( fail.message.length > 0 ).toEqual( true );

	const fail0 = CheckInput( '', 'font1', ['color1'], 'bgcolor1', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail0.pass ).toEqual( false );
	expect( fail0.message.length > 0 ).toEqual( true );

	const fail1 = CheckInput( 'INPUT', 'notfound', ['color1'], 'bgcolor1', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail1.pass ).toEqual( false );
	expect( fail1.message.length > 0 ).toEqual( true );

	const fail2 = CheckInput( 'INPUT', 'font1', ['notfound'], 'bgcolor1', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail2.pass ).toEqual( false );
	expect( fail2.message.length > 0 ).toEqual( true );

	const fail3 = CheckInput( 'INPUT', 'font1', ['color1'], 'notfound', 'left', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail3.pass ).toEqual( false );
	expect( fail3.message.length > 0 ).toEqual( true );

	const fail4 = CheckInput( 'INPUT', 'font1', ['color1'], 'bgcolor1', 'notfound', FONTFACES, COLORS, BGCOLORS, ALIGNMENT );
	expect( fail4.pass ).toEqual( false );
	expect( fail4.message.length > 0 ).toEqual( true );
});
