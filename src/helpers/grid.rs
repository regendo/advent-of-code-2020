use std::{convert::TryFrom, fmt::Debug};

pub fn grid<T>(input: &str) -> Vec<Vec<T>>
where
	T: TryFrom<char>,
	<T as TryFrom<char>>::Error: Debug,
{
	input
		.trim()
		.lines()
		.map(|line| {
			line
				.trim()
				.chars()
				.map(|c| T::try_from(c).unwrap())
				.collect()
		})
		.collect()
}
