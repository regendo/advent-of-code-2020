pub fn solve_1() {
	let input = include_str!("input.txt");
	let values: Vec<_> = input
		.lines()
		.map(|line| u32::from_str_radix(line.trim(), 10).unwrap())
		.collect();

	let (a, b) = find_2020_combinations_of_2(&values);
	println!("{} + {} = 2020", a, b);
	println!("{} * {} = {}", a, b, a * b);
}

pub fn solve_2() {
	let input = include_str!("input.txt");
	let values: Vec<_> = input
		.lines()
		.map(|line| u32::from_str_radix(line.trim(), 10).unwrap())
		.collect();

	let (a, b, c) = find_2020_combinations_of_3(&values);
	println!("{} + {} + {} = 2020", a, b, c);
	println!("{} * {} * {} = {}", a, b, c, a * b * c);
}

fn find_2020_combinations_of_2(values: &[u32]) -> (&u32, &u32) {
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

fn find_2020_combinations_of_3(values: &[u32]) -> (&u32, &u32, &u32) {
	values
		.iter()
		.enumerate()
		.find_map(|(first_idx, first_num)| {
			if let Some((second_num, third_num)) =
				values
					.iter()
					.enumerate()
					.skip(first_idx)
					.find_map(|(second_idx, second_num)| {
						if let Some(third_num) = values
							.iter()
							.skip(second_idx + 1)
							.find(|third_num| *first_num + *second_num + **third_num == 2020)
						{
							Some((second_num, third_num))
						} else {
							None
						}
					}) {
				Some((first_num, second_num, third_num))
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
		let (a, b) = find_2020_combinations_of_2(&values);
		assert_eq!((*a, *b), (1721_u32, 299_u32));
		assert_eq!(a + b, 2020);
		assert_eq!(a * b, 514579);
	}

	#[test]
	fn example_2_works() {
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
		let (a, b, c) = find_2020_combinations_of_3(&values);
		assert_eq!((*a, *b, *c), (979_u32, 366_u32, 675_u32));
		assert_eq!(a + b + c, 2020);
		assert_eq!(a * b * c, 241_861_950);
	}
}
