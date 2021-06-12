use itertools::{Itertools, MinMaxResult::MinMax};

mod solver;

static PREAMBLE_LENGTH: usize = 25;

pub fn solve_1() {
	let nums = read_input();
	if let Some(idx) = (PREAMBLE_LENGTH..nums.len()).find(|idx| !solver::does_some_up(&nums, *idx))
	{
		println!(
			"{} is the first number that doesn't sum up from the previous 25. (At index {}.)",
			nums[idx], idx
		)
	} else {
		panic!("All numbers accepted! (This should not happen.)")
	}
}

pub fn solve_2() {
	let nums = read_input();
	let wrong_at = (PREAMBLE_LENGTH..nums.len())
		.find(|idx| !solver::does_some_up(&nums, *idx))
		.expect("Failed in part 1");
	let seq = solver::find_sum_sequence(&nums, wrong_at).expect("Failed in part 2");
	if let MinMax(min, max) = seq.iter().minmax() {
		println!("Min {} + Max {} add up to {}", min, max, min + max);
	} else {
		panic!("Unexpected minmax result for sequence {:?}.", seq);
	}
}

fn read_input() -> Vec<u64> {
	include_str!("input.txt")
		.lines()
		.map(|line| u64::from_str_radix(line.trim(), 10).expect("Unparsable number!"))
		.collect()
}
