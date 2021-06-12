use std::cmp::Ordering::{Equal, Greater, Less};

use itertools::Itertools;

use super::PREAMBLE_LENGTH;

pub fn does_some_up(data: &[u64], target_idx: usize) -> bool {
	let target = data[target_idx];
	let search_range = (target_idx - PREAMBLE_LENGTH)..target_idx;
	data.get(search_range)
		.expect("Invalid search range!")
		.iter()
		.combinations(2)
		.any(|combination| combination.into_iter().sum::<u64>() == target)
}

pub fn find_sum_sequence(data: &[u64], target_idx: usize) -> Option<&[u64]> {
	let target = data[target_idx];
	for start in 0..target_idx {
		for end in start + 1..target_idx {
			match data
				.get(start..=end)
				.unwrap()
				.into_iter()
				.sum::<u64>()
				.cmp(&target)
			{
				Less => (),
				Equal => return data.get(start..=end),
				Greater => break,
			}
		}
	}
	None
}
