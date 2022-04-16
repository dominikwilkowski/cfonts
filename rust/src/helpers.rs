pub fn lowercase_first_letter(s: &str) -> String {
	let mut c = s.chars();
	match c.next() {
		None => String::new(),
		Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
	}
}

#[test]
fn lowercase_first_letter_works() {
	assert_eq!(lowercase_first_letter("test"), "test");
	assert_eq!(lowercase_first_letter("TEST"), "tEST");
	assert_eq!(lowercase_first_letter("Test"), "test");
	assert_eq!(lowercase_first_letter("!not a latter"), "!not a latter");
	assert_eq!(lowercase_first_letter("1234"), "1234");
	assert_eq!(lowercase_first_letter(""), "");
}
