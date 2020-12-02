use once_cell::sync as once_cell;
use regex;
use std::iter;

pub fn solve_1() {
	let pairs = parse_lines(include_str!("input.txt"));
	let passing = pairs
		.iter()
		.filter(|(policy, password)| policy.check_count(password));

	println!("{} passwords pass their old policies.", passing.count());
}

pub fn solve_2() {
	let pairs = parse_lines(include_str!("input.txt"));
	let passing = pairs
		.iter()
		.filter(|(policy, password)| policy.check_pos(password));

	println!("{} passwords pass their new policies.", passing.count());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Policy {
	character: char,
	min: usize,
	max: usize,
}

impl Policy {
	pub fn check_count(&self, password: &str) -> bool {
		let occurrences = password.chars().filter(|c| *c == self.character).count();
		self.min <= occurrences && occurrences <= self.max
	}

	pub fn check_pos(&self, password: &str) -> bool {
		let chars = iter::once('\0').chain(password.chars());
		let one = chars.clone().nth(self.min).unwrap() == self.character;
		let two = chars.clone().nth(self.max).unwrap() == self.character;
		one != two
	}
}

static LINE_REGEX: once_cell::Lazy<regex::Regex> =
	once_cell::Lazy::new(|| regex::Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap());

fn parse_lines(input: &str) -> Vec<(Policy, &str)> {
	LINE_REGEX
		.captures_iter(input)
		.map(|cap| {
			(
				Policy {
					min: cap.get(1).unwrap().as_str().parse().unwrap(),
					max: cap.get(2).unwrap().as_str().parse().unwrap(),
					character: cap.get(3).unwrap().as_str().chars().next().unwrap(),
				},
				cap.get(4).unwrap().as_str(),
			)
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_parses() {
		let input = "1-3 a: abcde";
		let policy = Policy {
			character: 'a',
			min: 1,
			max: 3,
		};

		assert_eq!(vec![(policy, "abcde")], parse_lines(input));
	}

	#[test]
	fn example_1_works() {
		let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

		let expected = vec![true, false, true];

		assert_eq!(
			expected,
			parse_lines(input)
				.iter()
				.map(|(policy, password)| policy.check_count(password))
				.collect::<Vec<bool>>()
		)
	}

	#[test]
	fn example_2_works() {
		let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

		let expected = vec![true, false, false];

		assert_eq!(
			expected,
			parse_lines(input)
				.iter()
				.map(|(policy, password)| policy.check_pos(password))
				.collect::<Vec<bool>>()
		)
	}
}
