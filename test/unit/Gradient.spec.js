/***************************************************************************************************************************************************************
 *
 * Gradient unit tests
 *
 **************************************************************************************************************************************************************/


const {
	Rgb2hsv,
	Hsv2rgb,
	Rgb2hex,
	Hex2rgb,
	Hsv2hsvRad,
	HsvRad2hsv,
	Hex2hsvRad,
	HsvRad2hex,
	GetLinear,
	GetTheta,
	Gradient,
} = require('../../src/Gradient.js');


test(`Gradient - Rgb2hsv - Convert RGB to HSV correctly`, () => {
	// unit test each line
	expect( Rgb2hsv({ r: 0, g: 0, b: 0 }) ).toEqual([ 0, 0, 0 ]);
	expect( Rgb2hsv({ r: 155, g: 150, b: 100 }) ).toEqual([ 54.54545454545456, 35.48387096774193, 60.78431372549019 ]);
	expect( Rgb2hsv({ r: 166, g: 20, b: 100 }) ).toEqual([ 327.1232876712329, 87.95180722891565, 65.09803921568627 ]);
	expect( Rgb2hsv({ r: 250, g: 255, b: 245 }) ).toEqual([ 90.00000000000009, 3.9215686274509776, 100 ]);
	expect( Rgb2hsv({ r: 110, g: 100, b: 250 }) ).toEqual([ 244, 60, 98.0392156862745 ]);

	// some random tests
	expect( Rgb2hsv({ r: 114, g: 103, b: 130 }) ).toEqual([ 264.44444444444446, 20.769230769230763, 50.98039215686274 ]);
	expect( Rgb2hsv({ r: 255, g: 150, b: 0 }) ).toEqual([ 35.294117647058826, 100, 100 ]);
	expect( Rgb2hsv({ r: 5, g: 78, b: 55 }) ).toEqual([ 161.0958904109589, 93.58974358974359, 30.58823529411765 ]);
	expect( Rgb2hsv({ r: 35, g: 10, b: 170 }) ).toEqual([ 249.375, 94.11764705882352, 66.66666666666666 ]);
	expect( Rgb2hsv({ r: 1, g: 6, b: 3 }) ).toEqual([ 144, 83.33333333333334, 2.3529411764705883 ]);
	expect( Rgb2hsv({ r: 100, g: 100, b: 100 }) ).toEqual([ 0, 0, 39.21568627450981 ]);
	expect( Rgb2hsv({ r: 100, g: 20, b: 100 }) ).toEqual([ 300, 80, 39.21568627450981 ]);

	// reverse tests from Hsv2rgb
	expect( Rgb2hsv({ r: 0, g: 0, b: 0 }) ).toEqual([ 0, 0, 0 ]);
	expect( Rgb2hsv({ r: 51, g: 46, b: 41 }) ).toEqual([ 29.99999999999998, 19.607843137254903, 20 ]);
	expect( Rgb2hsv({ r: 48, g: 51, b: 41 }) ).toEqual([ 78.00000000000003, 19.607843137254903, 20 ]);
	expect( Rgb2hsv({ r: 41, g: 51, b: 41 }) ).toEqual([ 120, 19.607843137254903, 20 ]);
	expect( Rgb2hsv({ r: 41, g: 51, b: 51 }) ).toEqual([ 180, 19.607843137254903, 20 ]);
	expect( Rgb2hsv({ r: 41, g: 41, b: 51 }) ).toEqual([ 240, 19.607843137254903, 20 ]);
	expect( Rgb2hsv({ r: 51, g: 41, b: 51 }) ).toEqual([ 300, 19.607843137254903, 20 ]);
});

test(`Gradient - Hsv2rgb - Convert HSV to RGB correctly`, () => {
	// unit test each line
	expect( Hsv2rgb( 0, 0, 0 ) ).toEqual({ r: 0, g: 0, b: 0 });
	expect( Hsv2rgb( 30, 20, 20 ) ).toEqual({ r: 51, g: 46, b: 41 });
	expect( Hsv2rgb( 80, 20, 20 ) ).toEqual({ r: 48, g: 51, b: 41 });
	expect( Hsv2rgb( 120, 20, 20 ) ).toEqual({ r: 41, g: 51, b: 41 });
	expect( Hsv2rgb( 180, 20, 20 ) ).toEqual({ r: 41, g: 51, b: 51 });
	expect( Hsv2rgb( 240, 20, 20 ) ).toEqual({ r: 41, g: 41, b: 51 });
	expect( Hsv2rgb( 300, 20, 20 ) ).toEqual({ r: 51, g: 41, b: 51 });

	// Reverse tests from Rgb2hsv
	expect( Hsv2rgb( 0, 0, 0 ) ).toEqual({ r: 0, g: 0, b: 0 });
	expect( Hsv2rgb( 54.54545454545456, 35.48387096774193, 60.78431372549019 ) ).toEqual({ r: 155, g: 150, b: 100 });
	expect( Hsv2rgb( 327.1232876712329, 87.95180722891565, 65.09803921568627 ) ).toEqual({ r: 166, g: 20, b: 100 });
	expect( Hsv2rgb( 90.00000000000009, 3.9215686274509776, 100 ) ).toEqual({ r: 250, g: 255, b: 245 });
	expect( Hsv2rgb( 244, 60, 98.0392156862745 ) ).toEqual({ r: 110, g: 100, b: 250 });
	expect( Hsv2rgb( 264.44444444444446, 20.769230769230763, 50.98039215686274 ) ).toEqual({ r: 114, g: 103, b: 130 });
	expect( Hsv2rgb( 35.294117647058826, 100, 100 ) ).toEqual({ r: 255, g: 150, b: 0 });
	expect( Hsv2rgb( 161.0958904109589, 93.58974358974359, 30.58823529411765 ) ).toEqual({ r: 5, g: 78, b: 55 });
	expect( Hsv2rgb( 249.375, 94.11764705882352, 66.66666666666666 ) ).toEqual({ r: 35, g: 10, b: 170 });
	expect( Hsv2rgb( 144, 83.33333333333334, 2.3529411764705883 ) ).toEqual({ r: 1, g: 6, b: 3 });
	expect( Hsv2rgb( 0, 0, 39.21568627450981 ) ).toEqual({ r: 100, g: 100, b: 100 });
	expect( Hsv2rgb( 300, 80, 39.21568627450981 ) ).toEqual({ r: 100, g: 20, b: 100 });
});

test(`Gradient - Rgb2hex - Convert RGB to HEX correctly`, () => {
	expect( Rgb2hex( 0, 0, 0 ) ).toBe('#000000');
	expect( Rgb2hex( 255, 255, 255 ) ).toBe('#ffffff');
	expect( Rgb2hex( 0, 255, 255 ) ).toBe('#00ffff');
	expect( Rgb2hex( 255, 0, 255 ) ).toBe('#ff00ff');
	expect( Rgb2hex( 255, 255, 0 ) ).toBe('#ffff00');
	expect( Rgb2hex( 127, 127, 127 ) ).toBe('#7f7f7f');
	expect( Rgb2hex( 255, 136, 0 ) ).toBe('#ff8800');
});

test(`Gradient - Hex2rgb - Convert HEX to RGB correctly`, () => {
	expect( Hex2rgb( '#000000' ) ).toEqual([ 0, 0, 0 ]);
	expect( Hex2rgb( '#ffffff' ) ).toEqual([ 255, 255, 255 ]);
	expect( Hex2rgb( '#00ffff' ) ).toEqual([ 0, 255, 255 ]);
	expect( Hex2rgb( '#ff00ff' ) ).toEqual([ 255, 0, 255 ]);
	expect( Hex2rgb( '#ffff00' ) ).toEqual([ 255, 255, 0 ]);
	expect( Hex2rgb( '#ffffffff' ) ).toEqual([ 255, 255, 255 ]);
	expect( Hex2rgb( '#fff' ) ).toEqual([ 255, 255, 255 ]);
	expect( Hex2rgb( '#ffff' ) ).toEqual([ 255, 255, 255 ]);
	expect( Hex2rgb( '#7f7f7f' ) ).toEqual([ 127, 127, 127 ]);
	expect( Hex2rgb( '#ff8800' ) ).toEqual([ 255, 136, 0 ]);
});

test(`Gradient - Hsv2hsvRad - Convert HSV to HSVrad correctly`, () => {
	expect( Hsv2hsvRad([ 0, 0, 0 ]) ).toEqual([ 0, 0, 0 ]);
	expect( Hsv2hsvRad([ 360, 0, 0 ]) ).toEqual([ 6.283185307179586, 0, 0 ]);
	expect( Hsv2hsvRad([ 180, 0, 0 ]) ).toEqual([ 3.141592653589793, 0, 0 ]);
});

test(`Gradient - HsvRad2hsv - Convert HSVrad to HSV correctly`, () => {
	expect( HsvRad2hsv( 0, 0, 0 ) ).toEqual([ 0, 0, 0 ]);
	expect( HsvRad2hsv( 6.283185307179586, 0, 0 ) ).toEqual([ 360, 0, 0 ]);
	expect( HsvRad2hsv( 3.141592653589793, 0, 0 ) ).toEqual([ 180, 0, 0 ]);
});

test(`Gradient - Hex2hsvRad - Convert HEX to HSVrad correctly`, () => {
	expect( Hex2hsvRad('#000000') ).toEqual([ 0, 0, 0 ]);
	expect( Hex2hsvRad('#ffffff') ).toEqual([ 0, 0, 100 ]);
	expect( Hex2hsvRad('#00ffff') ).toEqual([ 3.141592653589793, 100, 100 ]);
	expect( Hex2hsvRad('#ff00ff') ).toEqual([ 5.235987755982989, 100, 100 ]);
	expect( Hex2hsvRad('#ffff00') ).toEqual([ 1.0471975511965976, 100, 100 ]);
});

test(`Gradient - HsvRad2hex - Convert HSVrad to HEX correctly`, () => {
	expect( HsvRad2hex( 0, 0, 0 ) ).toEqual('#000000');
	expect( HsvRad2hex( 0, 0, 100 ) ).toEqual('#ffffff');
	expect( HsvRad2hex( 3.141592653589793, 100, 100 ) ).toEqual('#00ffff');
	expect( HsvRad2hex( 5.235987755982989, 100, 100 ) ).toEqual('#ff00ff');
	expect( HsvRad2hex( 1.0471975511965976, 100, 100 ) ).toEqual('#ffff00');
});

test(`Gradient - GetLinear - Interpolate between points correctly`, () => {
	const start = 0;
	const end = 5;
	const steps = 5;

	expect( GetLinear( start, end, 0, steps ) ).toBe( 0 );
	expect( GetLinear( start, end, 1, steps ) ).toBe( 1 );
	expect( GetLinear( start, end, 2, steps ) ).toBe( 2 );
	expect( GetLinear( start, end, 3, steps ) ).toBe( 3 );
	expect( GetLinear( start, end, 4, steps ) ).toBe( 4 );
	expect( GetLinear( start, end, 5, steps ) ).toBe( 5 );
});

test(`Gradient - GetTheta - Interpolate between points straight correctly`, () => {
	const start1 = 0;
	const end1 = 5;
	const steps1 = 5;

	expect( GetTheta( start1, end1, 0, steps1 ) ).toBe( 0 );
	expect( GetTheta( start1, end1, 1, steps1 ) ).toBe( 1 );
	expect( GetTheta( start1, end1, 2, steps1 ) ).toBe( 2 );
	expect( GetTheta( start1, end1, 3, steps1 ) ).toBe( 3 );
	expect( GetTheta( start1, end1, 4, steps1 ) ).toBe( 4 );
	expect( GetTheta( start1, end1, 5, steps1 ) ).toBe( 5 );

	const start2 = 2;
	const end2 = 3;
	const steps2 = 3;

	expect( GetTheta( start2, end2, 0, steps2 ) ).toBe( 2 );
	expect( GetTheta( start2, end2, 1, steps2 ) ).toBe( 0.238938230940138 );
	expect( GetTheta( start2, end2, 2, steps2 ) ).toBe( 4.761061769059863 );
	expect( GetTheta( start2, end2, 3, steps2 ) ).toBe( 3 );

	const start3 = 3;
	const end3 = 2;
	const steps3 = 3;

	expect( GetTheta( start3, end3, 0, steps3 ) ).toBe( 3 );
	expect( GetTheta( start3, end3, 1, steps3 ) ).toBe( 4.761061769059862 );
	expect( GetTheta( start3, end3, 2, steps3 ) ).toBe( 0.23893823094013733 );
	expect( GetTheta( start3, end3, 3, steps3 ) ).toBe( 2 );

	const start4 = 5;
	const end4 = 1;
	const steps4 = 3;

	expect( GetTheta( start4, end4, 0, steps4 ) ).toBe( 5 );
	expect( GetTheta( start4, end4, 1, steps4 ) ).toBe( 3.666666666666667 );
	expect( GetTheta( start4, end4, 2, steps4 ) ).toBe( 2.3333333333333335 );
	expect( GetTheta( start4, end4, 3, steps4 ) ).toBe( 1 );
});

test(`Gradient - Return a bunch of colors inbetween two given colors`, () => {
	expect( Gradient( '#ff8800', '#8899dd', 10 ) ).toEqual([
		"#ff8800",
		"#fbe311",
		"#c0f722",
		"#7bf432",
		"#44f042",
		"#51ec87",
		"#5fe8c1",
		"#6ddce5",
		"#7bb4e1",
		"#8899dd",
	]);
});
