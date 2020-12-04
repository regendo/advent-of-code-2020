mod passport;

pub fn solve_1() {
	let input = include_str!("input.txt");
	let parsed = passport::parse(input);

	println!(
		"{} valid passwords found (of {} total).",
		parsed.iter().filter(|p| p.is_ok()).count(),
		parsed.len()
	);
}

pub fn solve_2() {
	unimplemented!()
}
