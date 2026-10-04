//! The environment boundary: JavaScript hosts pass their variables as
//! parallel name/value arrays, and one lookup reads them for every resolution

/// A lookup over the parallel name/value arrays a JavaScript host gathered
pub(crate) fn lookup<'a>(names: &'a [String], values: &'a [String]) -> impl Fn(&str) -> Option<String> + 'a {
	move |name: &str| names.iter().position(|candidate| candidate == name).and_then(|index| values.get(index).cloned())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_lookup_pairs_names_with_values_by_position() {
		let names = vec![String::from("FORCE_SIZE"), String::from("TERM")];
		let values = vec![String::from("12"), String::from("xterm")];
		let environment = lookup(&names, &values);

		assert_eq!(environment("FORCE_SIZE"), Some(String::from("12")));
		assert_eq!(environment("TERM"), Some(String::from("xterm")));
		assert_eq!(environment("NO_COLOR"), None);
	}

	#[test]
	fn a_name_without_a_value_reads_as_absent() {
		let names = vec![String::from("FORCE_SIZE")];
		let values = Vec::new();
		let environment = lookup(&names, &values);

		assert_eq!(environment("FORCE_SIZE"), None);
	}
}
