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
