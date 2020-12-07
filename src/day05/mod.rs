mod binary;

pub fn solve_1() {
	let max_seat_id = include_str!("input.txt")
		.trim()
		.lines()
		.map(binary::parse)
		.map(|instructions| binary::seat_id(binary::calculate_seat(&instructions)))
		.max()
		.unwrap();

	println!("Max seat ID of all boarding passes: {}", max_seat_id);
}

pub fn solve_2() {
	todo!()
}
