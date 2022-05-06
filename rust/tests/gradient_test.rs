extern crate cfonts;

use cfonts::config::Options;
use cfonts::gradient::{
	get_gradient_colors, get_linear, get_theta, get_transition_colors, get_transition_steps, hex2rgb, hex2rsv, hsv2rgb,
	hsv2rsv, paint_lines, rgb2hex, rgb2hsv, rsv2hex, rsv2hsv, transition, Hsv, Rgb, Rsv,
};

#[cfg(test)]
mod tests {
	extern crate temp_env;
	use super::*;

	#[test]
	fn rgb2hsv_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(rgb2hsv(&Rgb::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(
			rgb2hsv(&Rgb::Val(155.0, 150.0, 100.0), &options),
			Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019)
		);
		assert_eq!(
			rgb2hsv(&Rgb::Val(166.0, 20.0, 100.0), &options),
			Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627)
		);
		assert_eq!(
			rgb2hsv(&Rgb::Val(250.0, 255.0, 245.0), &options),
			Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(110.0, 100.0, 250.0), &options), Hsv::Val(244.0, 60.0, 98.0392156862745));
		assert_eq!(rgb2hsv(&Rgb::Val(255.0, 0.0, 0.0), &options), Hsv::Val(0.0, 100.0, 100.0));

		// some random tests
		assert_eq!(
			rgb2hsv(&Rgb::Val(114.0, 103.0, 130.0), &options),
			Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(255.0, 150.0, 0.0), &options), Hsv::Val(35.294117647058826, 100.0, 100.0));
		assert_eq!(
			rgb2hsv(&Rgb::Val(5.0, 78.0, 55.0), &options),
			Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765)
		);
		assert_eq!(
			rgb2hsv(&Rgb::Val(35.0, 10.0, 170.0), &options),
			Hsv::Val(249.375, 94.11764705882352, 66.66666666666666)
		);
		assert_eq!(rgb2hsv(&Rgb::Val(1.0, 6.0, 3.0), &options), Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883));
		assert_eq!(rgb2hsv(&Rgb::Val(100.0, 100.0, 100.0), &options), Hsv::Val(0.0, 0.0, 39.21568627450981));
		assert_eq!(rgb2hsv(&Rgb::Val(100.0, 20.0, 100.0), &options), Hsv::Val(300.0, 80.0, 39.21568627450981));

		// reverse tests from Hsv2rgb
		assert_eq!(rgb2hsv(&Rgb::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rgb2hsv(&Rgb::Val(51.0, 46.0, 41.0), &options), Hsv::Val(29.99999999999998, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(48.0, 51.0, 41.0), &options), Hsv::Val(78.00000000000003, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41.0, 51.0, 41.0), &options), Hsv::Val(120.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41.0, 51.0, 51.0), &options), Hsv::Val(180.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(41.0, 41.0, 51.0), &options), Hsv::Val(240.0, 19.607843137254903, 20.0));
		assert_eq!(rgb2hsv(&Rgb::Val(51.0, 41.0, 51.0), &options), Hsv::Val(300.0, 19.607843137254903, 20.0));
	}

	#[test]
	fn hsv2rgb_works() {
		let options = Options::default();
		// unit test each line
		assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0.0, 0.0, 0.0));

		assert_eq!(hsv2rgb(&Hsv::Val(30.0, 20.0, 20.0), &options), Rgb::Val(51.0, 45.9, 40.800000000000004));
		assert_eq!(hsv2rgb(&Hsv::Val(80.0, 20.0, 20.0), &options), Rgb::Val(47.6, 51.0, 40.800000000000004));
		assert_eq!(hsv2rgb(&Hsv::Val(120.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 51.0, 40.800000000000004));
		assert_eq!(hsv2rgb(&Hsv::Val(180.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 51.0, 51.0));
		assert_eq!(hsv2rgb(&Hsv::Val(240.0, 20.0, 20.0), &options), Rgb::Val(40.800000000000004, 40.800000000000004, 51.0));
		assert_eq!(hsv2rgb(&Hsv::Val(300.0, 20.0, 20.0), &options), Rgb::Val(51.0, 40.800000000000004, 51.0));
		assert_eq!(hsv2rgb(&Hsv::Val(360.0, 100.0, 100.0), &options), Rgb::Val(255.0, 0.0, 0.0));

		// Reverse tests from Rgb2hsv
		assert_eq!(hsv2rgb(&Hsv::Val(0.0, 0.0, 0.0), &options), Rgb::Val(0.0, 0.0, 0.0));
		assert_eq!(
			hsv2rgb(&Hsv::Val(54.54545454545456, 35.48387096774193, 60.78431372549019), &options),
			Rgb::Val(155.0, 150.00000000000003, 100.00000000000001)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(327.1232876712329, 87.95180722891565, 65.09803921568627), &options),
			Rgb::Val(165.99999999999997, 20.000000000000007, 99.99999999999991)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(90.00000000000009, 3.9215686274509776, 100.0), &options),
			Rgb::Val(250.0, 255.0, 245.0)
		);
		assert_eq!(hsv2rgb(&Hsv::Val(244.0, 60.0, 98.0392156862745), &options), Rgb::Val(109.99999999999996, 100.0, 250.0));
		assert_eq!(
			hsv2rgb(&Hsv::Val(264.44444444444446, 20.769230769230763, 50.98039215686274), &options),
			Rgb::Val(114.0, 103.00000000000001, 130.0)
		);
		assert_eq!(hsv2rgb(&Hsv::Val(35.294117647058826, 100.0, 100.0), &options), Rgb::Val(255.0, 150.0, 0.0));
		assert_eq!(
			hsv2rgb(&Hsv::Val(161.0958904109589, 93.58974358974359, 30.58823529411765), &options),
			Rgb::Val(5.0, 78.0, 55.00000000000001)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(249.375, 94.11764705882352, 66.66666666666666), &options),
			Rgb::Val(35.00000000000002, 10.00000000000002, 169.99999999999997)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(144.0, 83.33333333333334, 2.3529411764705883), &options),
			Rgb::Val(0.9999999999999991, 6.0, 2.999999999999999)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(0.0, 0.0, 39.21568627450981), &options),
			Rgb::Val(100.00000000000001, 100.00000000000001, 100.00000000000001)
		);
		assert_eq!(
			hsv2rgb(&Hsv::Val(300.0, 80.0, 39.21568627450981), &options),
			Rgb::Val(100.00000000000001, 20.0, 100.00000000000001)
		);
	}

	#[test]
	fn rgb2hex_works() {
		let options = Options::default();
		assert_eq!(rgb2hex(&Rgb::Val(0.0, 0.0, 0.0), &options), "#000000");
		assert_eq!(rgb2hex(&Rgb::Val(255.0, 255.0, 255.0), &options), "#ffffff");
		assert_eq!(rgb2hex(&Rgb::Val(0.0, 255.0, 255.0), &options), "#00ffff");
		assert_eq!(rgb2hex(&Rgb::Val(255.0, 0.0, 255.0), &options), "#ff00ff");
		assert_eq!(rgb2hex(&Rgb::Val(255.0, 255.0, 0.0), &options), "#ffff00");
		assert_eq!(rgb2hex(&Rgb::Val(127.0, 127.0, 127.0), &options), "#7f7f7f");
		assert_eq!(rgb2hex(&Rgb::Val(255.0, 136.0, 0.0), &options), "#ff8800");
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
		assert_eq!(hsv2rsv(&Hsv::Val(0.0, 0.0, 0.0), &options), Rsv::Val(0.0, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(360.0, 0.0, 0.0), &options), Rsv::Val(6.283185307179586, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(180.0, 0.0, 0.0), &options), Rsv::Val(3.141592653589793, 0.0, 0.0));
		assert_eq!(hsv2rsv(&Hsv::Val(300.0, 0.0, 0.0), &options), Rsv::Val(5.235987755982989, 0.0, 0.0));
	}

	#[test]
	fn rsv2hsv_works() {
		let options = Options::default();
		assert_eq!(rsv2hsv(&Rsv::Val(0.0, 0.0, 0.0), &options), Hsv::Val(0.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(6.283185307179586, 0.0, 0.0), &options), Hsv::Val(360.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(3.141592653589793, 0.0, 0.0), &options), Hsv::Val(180.0, 0.0, 0.0));
		assert_eq!(rsv2hsv(&Rsv::Val(5.235987755982989, 0.0, 0.0), &options), Hsv::Val(300.0, 0.0, 0.0));
	}

	#[test]
	fn hex2rsv_works() {
		let options = Options::default();
		assert_eq!(hex2rsv("#000000", &options), Rsv::Val(0.0, 0.0, 0.0));
		assert_eq!(hex2rsv("#ffffff", &options), Rsv::Val(0.0, 0.0, 100.0));
		assert_eq!(hex2rsv("#00ffff", &options), Rsv::Val(3.141592653589793, 100.0, 100.0));
		assert_eq!(hex2rsv("#ff00ff", &options), Rsv::Val(5.235987755982989, 100.0, 100.0));
		assert_eq!(hex2rsv("#ffff00", &options), Rsv::Val(1.0471975511965976, 100.0, 100.0));
	}

	#[test]
	fn rsv2hex_works() {
		let options = Options::default();
		assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 0.0), &options), "#000000".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(0.0, 0.0, 100.0), &options), "#ffffff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(3.141592653589793, 100.0, 100.0), &options), "#00ffff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(5.235987755982989, 100.0, 100.0), &options), "#ff00ff".to_string());
		assert_eq!(rsv2hex(&Rsv::Val(1.0471975511965976, 100.0, 100.0), &options), "#ffff00".to_string());
	}

	#[test]
	fn get_linear_works() {
		let options = Options::default();

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 5;
		assert_eq!(get_linear(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_linear(point_a, point_b, 1, steps, &options), 1.0);
		assert_eq!(get_linear(point_a, point_b, 2, steps, &options), 2.0);
		assert_eq!(get_linear(point_a, point_b, 3, steps, &options), 3.0);
		assert_eq!(get_linear(point_a, point_b, 4, steps, &options), 4.0);
		assert_eq!(get_linear(point_a, point_b, 5, steps, &options), point_b);

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 0;
		assert_eq!(get_linear(point_a, point_b, 0, steps, &options), 5.0);
	}

	#[test]
	fn get_theta_works() {
		let options = Options::default();

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 5;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 1.0);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 2.0);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), 3.0);
		assert_eq!(get_theta(point_a, point_b, 4, steps, &options), 4.0);
		assert_eq!(get_theta(point_a, point_b, 5, steps, &options), point_b);

		let point_a = 2.0;
		let point_b = 3.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 0.238938230940138);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 4.761061769059863);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 3.0;
		let point_b = 2.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 4.761061769059862);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 0.23893823094013733);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 5.0;
		let point_b = 1.0;
		let steps = 3;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_a);
		assert_eq!(get_theta(point_a, point_b, 1, steps, &options), 3.666666666666667);
		assert_eq!(get_theta(point_a, point_b, 2, steps, &options), 2.3333333333333335);
		assert_eq!(get_theta(point_a, point_b, 3, steps, &options), point_b);

		let point_a = 0.0;
		let point_b = 5.0;
		let steps = 0;
		assert_eq!(get_theta(point_a, point_b, 0, steps, &options), point_b);
	}

	#[test]
	fn get_gradient_colors_works() {
		let options = Options::default();
		assert_eq!(
			get_gradient_colors("#ff8800", "#8899dd", 10, &options),
			vec![
				"#ff8800", "#fbe211", "#c0f721", "#7bf331", "#44ef41", "#50ec86", "#5fe8c0", "#6ddbe4", "#7ab4e0", "#8799dd",
			]
		);
	}

	#[test]
	fn get_transition_colors_works() {
		let options = Options::default();
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", -1, &options).len(), 0);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 0, &options).len(), 0);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 1, &options), vec!["#7f007f"]);
		assert_eq!(get_transition_colors("#ff0000", "#0000ff", 2, &options), vec!["#aa0055", "#5500aa"]);
		assert_eq!(
			get_transition_colors("#ff0000", "#0000ff", 5, &options),
			vec!["#d4002a", "#aa0055", "#7f007f", "#5500aa", "#2a00d4"]
		);
	}

	#[test]
	fn paint_lines_works() {
		let options = Options::default();
		let lines = vec!["XXX".to_string(), "XXX".to_string()];
		let colors = vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()];

		assert_eq!(
			paint_lines(&lines, &colors, 0, &options),
			vec![
				"\x1b[38;2;255;0;0mX\x1b[39m\x1b[38;2;0;255;0mX\x1b[39m\x1b[38;2;0;0;255mX\x1b[39m".to_string(),
				"\x1b[38;2;255;0;0mX\x1b[39m\x1b[38;2;0;255;0mX\x1b[39m\x1b[38;2;0;0;255mX\x1b[39m".to_string(),
			]
		);

		let lines = vec!["     XXX".to_string(), "     XXX".to_string()];
		assert_eq!(
			paint_lines(&lines, &colors, 5, &options),
			vec![
				"     \x1b[38;2;255;0;0mX\x1b[39m\x1b[38;2;0;255;0mX\x1b[39m\x1b[38;2;0;0;255mX\x1b[39m".to_string(),
				"     \x1b[38;2;255;0;0mX\x1b[39m\x1b[38;2;0;255;0mX\x1b[39m\x1b[38;2;0;0;255mX\x1b[39m".to_string(),
			]
		);
	}

	#[test]
	fn get_transition_steps_works() {
		let options = Options::default();

		let result = get_transition_steps(&vec!["color1".to_string()], 1, &options);
		assert_eq!(result.len(), 0);

		let colors = vec!["color1".to_string(), "color2".to_string()];
		assert_eq!(get_transition_steps(&colors, 1, &options), vec![-1]);
		assert_eq!(get_transition_steps(&colors, 2, &options), vec![0]);
		assert_eq!(get_transition_steps(&colors, 3, &options), vec![1]);
		assert_eq!(get_transition_steps(&colors, 4, &options), vec![2]);
		assert_eq!(get_transition_steps(&colors, 5, &options), vec![3]);

		let colors = vec!["color1".to_string(), "color2".to_string(), "color3".to_string()];
		assert_eq!(get_transition_steps(&colors, 1, &options), vec![-1, -1]);
		assert_eq!(get_transition_steps(&colors, 2, &options), vec![-1, 0]);
		assert_eq!(get_transition_steps(&colors, 3, &options), vec![0, 0]);
		assert_eq!(get_transition_steps(&colors, 4, &options), vec![0, 1]);
		assert_eq!(get_transition_steps(&colors, 5, &options), vec![1, 1]);
	}

	#[test]
	fn transition_works() {
		let options = Options::default();
		let colors = vec!["#ff0000".to_string(), "#0000ff".to_string()];
		assert_eq!(transition(&colors, 1, &options), vec!["#0000ff".to_string()]);
		assert_eq!(transition(&colors, 2, &options), vec!["#ff0000".to_string(), "#0000ff".to_string()]);
		assert_eq!(
			transition(&colors, 3, &options),
			vec!["#ff0000".to_string(), "#7f007f".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			transition(&colors, 4, &options),
			vec![
				"#ff0000".to_string(),
				"#aa0055".to_string(),
				"#5500aa".to_string(),
				"#0000ff".to_string()
			]
		);

		let colors = vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()];
		assert_eq!(transition(&colors, 1, &options), vec!["#0000ff".to_string()]);
		assert_eq!(transition(&colors, 2, &options), vec!["#ff0000".to_string(), "#0000ff".to_string()]);
		assert_eq!(
			transition(&colors, 3, &options),
			vec!["#ff0000".to_string(), "#00ff00".to_string(), "#0000ff".to_string()]
		);
		assert_eq!(
			transition(&colors, 4, &options),
			vec![
				"#ff0000".to_string(),
				"#00ff00".to_string(),
				"#007f7f".to_string(),
				"#0000ff".to_string(),
			]
		);
		assert_eq!(
			transition(&colors, 10, &options),
			vec![
				"#ff0000".to_string(),
				"#bf3f00".to_string(),
				"#7f7f00".to_string(),
				"#3fbf00".to_string(),
				"#00ff00".to_string(),
				"#00cc33".to_string(),
				"#009966".to_string(),
				"#006699".to_string(),
				"#0033cc".to_string(),
				"#0000ff".to_string(),
			]
		);
	}
}
