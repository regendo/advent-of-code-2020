use itertools::Itertools;

pub fn solve_1() {
	let input = include_str!("input.txt");
	let confirms: usize = split_into_groups(input)
		.iter()
		.map(|group| questions_confirmed(group))
		.map(|chars| chars.len())
		.sum();

	println!("The groups confirmed a total of {} questions.", confirms);
}

pub fn solve_2() {
	let input = include_str!("input.txt");
	let confirms: usize = split_into_groups_2(input)
		.iter()
		.map(|group| questions_confirmed_2(group))
		.map(|chars| chars.len())
		.sum();

	println!(
		"The groups confirmed a total of {} questions unanimously.",
		confirms
	);
}

pub(crate) fn split_into_groups(input: &str) -> Vec<String> {
	input
		.lines()
		.map(|s| s.to_owned())
		.coalesce(|a, b| {
			if b.trim().is_empty() {
				Err((a, b))
			} else {
				Ok(format!("{}{}", a, b))
			}
		})
		.collect()
}

pub(crate) fn questions_confirmed(group: &str) -> Vec<char> {
	group
		.chars()
		.unique()
		.filter(|c| c.is_ascii_alphabetic())
		.collect()
}

pub(crate) fn split_into_groups_2(input: &str) -> Vec<&str> {
	#[cfg(unix)]
	let blankline = "\n\n";
	#[cfg(windows)]
	let blankline = "\r\n\r\n";

	input.split(blankline).collect()
}

pub(crate) fn questions_confirmed_2(input: &str) -> Vec<char> {
	let mut lines = input.trim().lines();
	let (first, remainder) = (lines.next().unwrap(), lines);
	first
		.chars()
		.filter(|c| c.is_ascii_alphabetic())
		.filter(|&c| remainder.clone().all(|line| line.contains(c)))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_1_works() {
		let input = include_str!("example_input.txt");
		let yes_s: Vec<usize> = split_into_groups(input)
			.iter()
			.map(|group| questions_confirmed(group))
			.map(|chars| chars.len())
			.collect();

		assert_eq!(vec![3, 3, 3, 1, 1], yes_s);
	}

	#[test]
	fn example_2_works() {
		let input = include_str!("example_input.txt");
		let yes_s: Vec<usize> = split_into_groups_2(input)
			.iter()
			.map(|group| questions_confirmed_2(group))
			.map(|chars| chars.len())
			.collect();

		assert_eq!(vec![3, 0, 1, 1, 1], yes_s);
	}
}
