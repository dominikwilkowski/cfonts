extern crate cfonts;

use cfonts::config::Options;
use cfonts::gradient::{hex2rgb, hsv2rgb, hsv2rsv, rgb2hsv, rsv2hsv, Hsv, Rgb, Rsv};

#[cfg(test)]
mod tests {
	extern crate temp_env;
	use super::*;

	#[test]
	fn rgb2hsv_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(rgb2hsv(Rgb::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(
			rgb2hsv(Rgb::Val(155.0, 150.0, 100.0), &options),
			Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019)
		);
		assert_eq!(
			rgb2hsv(Rgb::Val(166.0, 20.0, 100.0), &options),
			Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627)
		);
		assert_eq!(
			rgb2hsv(Rgb::Val(250.0, 255.0, 245.0), &options),
			Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0)
		);
		assert_eq!(rgb2hsv(Rgb::Val(110.0, 100.0, 250.0), &options), Hsv::Val(244.0, 60.0, 98.0392156862745));
		assert_eq!(rgb2hsv(Rgb::Val(255.0, 0.0, 0.0), &options), Hsv::Val(0.0, 100.0, 100.0));

		// some random tests
		assert_eq!(
			rgb2hsv(Rgb::Val(114.0, 103.0, 130.0), &options),
			Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274)
		);
		assert_eq!(rgb2hsv(Rgb::Val(255.0, 150.0, 0.0), &options), Hsv::Val(35.294117647058826, 100.0, 100.0));
		assert_eq!(
			rgb2hsv(Rgb::Val(5.0, 78.0, 55.0), &options),
			Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765)
		);
		assert_eq!(rgb2hsv(Rgb::Val(35.0, 10.0, 170.0), &options), Hsv::Val(249.375, 94.11764705882352, 66.66666666666666));
		assert_eq!(rgb2hsv(Rgb::Val(1.0, 6.0, 3.0), &options), Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883));
		assert_eq!(rgb2hsv(Rgb::Val(100.0, 100.0, 100.0), &options), Hsv::Val(0.0, 0.0, 39.21568627450981));
		assert_eq!(rgb2hsv(Rgb::Val(100.0, 20.0, 100.0), &options), Hsv::Val(300.0, 80.0, 39.21568627450981));

		// reverse tests from Hsv2rgb
		assert_eq!(rgb2hsv(Rgb::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rgb2hsv(Rgb::Val(51.0, 46.0, 41.0), &options), Hsv::Val(29.99999999999998, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(Rgb::Val(48.0, 51.0, 41.0), &options), Hsv::Val(78.00000000000003, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(Rgb::Val(41.0, 51.0, 41.0), &options), Hsv::Val(120.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(Rgb::Val(41.0, 51.0, 51.0), &options), Hsv::Val(180.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(Rgb::Val(41.0, 41.0, 51.0), &options), Hsv::Val(240.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(Rgb::Val(51.0, 41.0, 51.0), &options), Hsv::Val(300.0, 19.607843137254903, 20.0));
	}

	#[test]
	fn hsv2rgb_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(hsv2rgb(Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0.0, 0.0, 0.0));

		assert_eq!(hsv2rgb(Hsv::Val(30.0, 20.0, 20.0), &options), Rgb::Val(51.0, 45.9, 40.800000000000004));
		assert_eq!(hsv2rgb(Hsv::Val(80.0, 20.0, 20.0), &options), Rgb::Val(47.6, 51.0, 40.800000000000004));
		assert_eq!(hsv2rgb(Hsv::Val(120.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 51.0, 40.800000000000004));
		assert_eq!(hsv2rgb(Hsv::Val(180.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 51.0, 51.0));
		assert_eq!(hsv2rgb(Hsv::Val(240.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 40.800000000000004, 51.0));
		assert_eq!(hsv2rgb(Hsv::Val(300.0, 20.0, 20.0), &options), Rgb::Val(51.0, 40.800000000000004, 51.0));
		assert_eq!(hsv2rgb(Hsv::Val(360.0, 100.0, 100.0), &options), Rgb::Val(255.0, 0.0, 0.0));

		// Reverse tests from Rgb2hsv
		assert_eq!(hsv2rgb(Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0.0, 0.0, 0.0));
		assert_eq!(
			hsv2rgb(Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019), &options),
			Rgb::Val(155.0, 150.00000000000003, 100.00000000000001)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627), &options),
			Rgb::Val(165.99999999999997, 20.000000000000007, 99.99999999999991)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0), &options),
			Rgb::Val(250.0, 255.0, 245.0)
		);
		assert_eq!(hsv2rgb(Hsv::Val(244.0, 60.0, 98.0392156862745), &options), Rgb::Val(109.99999999999996, 100.0, 250.0));
		assert_eq!(
			hsv2rgb(Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274), &options),
			Rgb::Val(114.0, 103.00000000000001, 130.0)
		);
		assert_eq!(hsv2rgb(Hsv::Val(35.294117647058826, 100.0, 100.0), &options), Rgb::Val(255.0, 150.0, 0.0));
		assert_eq!(
			hsv2rgb(Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765), &options),
			Rgb::Val(5.0, 78.0, 55.00000000000001)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(249.375, 94.11764705882352, 66.66666666666666), &options),
			Rgb::Val(35.00000000000002, 10.00000000000002, 169.99999999999997)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883), &options),
			Rgb::Val(0.9999999999999991, 6.0, 2.999999999999999)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(0.0, 0.0, 39.21568627450981), &options),
			Rgb::Val(100.00000000000001, 100.00000000000001, 100.00000000000001)
		);
		assert_eq!(
			hsv2rgb(Hsv::Val(300.0, 80.0, 39.21568627450981), &options),
			Rgb::Val(100.00000000000001, 20.0, 100.00000000000001)
		);
	}

	#[test]
	fn hex2rgb_works() {
		let options = Options::default();
		assert_eq!(hex2rgb("#000000", &options), Rgb::Val(0.0, 0.0, 0.0));
		assert_eq!(hex2rgb("#ffffff", &options), Rgb::Val(255.0, 255.0, 255.0));
		assert_eq!(hex2rgb("#00ffff", &options), Rgb::Val(0.0, 255.0, 255.0));
		assert_eq!(hex2rgb("#ff00ff", &options), Rgb::Val(255.0, 0.0, 255.0));
		assert_eq!(hex2rgb("#ffff00", &options), Rgb::Val(255.0, 255.0, 0.0));
		assert_eq!(hex2rgb("#ffffffff", &options), Rgb::Val(255.0, 255.0, 255.0));
		assert_eq!(hex2rgb("#fff", &options), Rgb::Val(255.0, 255.0, 255.0));
		assert_eq!(hex2rgb("#ffff", &options), Rgb::Val(255.0, 255.0, 255.0));
		assert_eq!(hex2rgb("#7f7f7f", &options), Rgb::Val(127.0, 127.0, 127.0));
		assert_eq!(hex2rgb("#ff8800", &options), Rgb::Val(255.0, 136.0, 0.0));
	}

	#[test]
	fn hsv2rsv_works() {
		let options = Options::default();
		assert_eq!(hsv2rsv(Hsv::Val(0.0, 0.0, 0.0), &options), Rsv::Val(0.0, 0.0, 0.0));
		assert_eq!(hsv2rsv(Hsv::Val(360.0, 0.0, 0.0), &options), Rsv::Val(6.283185307179586, 0.0, 0.0));
		assert_eq!(hsv2rsv(Hsv::Val(180.0, 0.0, 0.0), &options), Rsv::Val(3.141592653589793, 0.0, 0.0));
	}

	#[test]
	fn rsv2hsv_works() {
		let options = Options::default();
		assert_eq!(rsv2hsv(Rsv::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(Rsv::Val(6.283185307179586, 0.0, 0.0), &options), Hsv::Val(360.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(Rsv::Val(3.141592653589793, 0.0, 0.0), &options), Hsv::Val(180.0, 0.0, 0.0));
	}
}
