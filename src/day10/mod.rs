use std::iter;

use itertools::Itertools;

use crate::helpers;

pub fn solve_1() {
	let input: Vec<u16> = helpers::input::nums!();
	// our wall outlet is treated as having a value of 0, so we add that to our input for convenience
	// also, the correct order of adapters is just sorted in ascending order.
	let sequence = input.iter().chain(iter::once(&0)).sorted();
	let jolt_differences = sequence.tuple_windows().map(|(a, b)| b - a);
	let (ones, _twos, threes) =
	// threes start at 1 because the difference between the last adapter's output and our device's input is always 3
		jolt_differences.fold((0, 0, 1), |(ones, twos, threes), diff| match diff {
			1 => (ones + 1, twos, threes),
			2 => (ones, twos + 1, threes),
			3 => (ones, twos, threes + 1),
			_ => panic!("Invalid jolt difference of {}.", diff),
		});
	println!("The magic number is {}.", ones * threes);
}

pub fn solve_2() {
	unimplemented!()
}
