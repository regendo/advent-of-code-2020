pub fn solve_1() {
	let input = include_str!("input.txt");
	let values: Vec<_> = input
		.lines()
		.map(|line| u32::from_str_radix(line.trim(), 10).unwrap())
		.collect();

	let (a, b) = find_2020_combinations(&values);
	println!("{} + {} = 2020", a, b);
	println!("{} * {} = {}", a, b, a * b);
}

fn find_2020_combinations(values: &[u32]) -> (&u32, &u32) {
	values
		.iter()
		.enumerate()
		.find_map(|(idx, num)| {
			if let Some(other) = values
				.iter()
				.skip(idx + 1)
				.find(|other| *num + **other == 2020)
			{
				Some((num, other))
			} else {
				None
			}
		})
		.expect("No matching pair found!")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_1_works() {
		let input = r"1721
979
366
299
675
1456";
		let values: Vec<_> = input
			.lines()
			.map(|line| u32::from_str_radix(line.trim(), 10).unwrap())
			.collect();
		let (a, b) = find_2020_combinations(&values);
		assert_eq!(a + b, 2020);
		assert_eq!((*a, *b), (1721_u32, 299_u32));
		assert_eq!(a * b, 514579);
	}
}
