macro_rules! _nums {
	() => {
		include_str!("input.txt")
			.lines()
			.map(|line| {
				line.trim()
					.parse()
					.expect(&format!("unparsable number {}!", line.trim()))
			})
			.collect()
	};
}

// This hack appears to be the only way to properly export our macro at the proper scope.
pub(crate) use _nums as nums;
