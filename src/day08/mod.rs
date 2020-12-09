mod compile;
mod machine;

pub fn solve_1() {
	let input = include_str!("input.txt");
	let instructions = compile::compile(input);
	let mut machine = machine::Machine::new(&instructions);
	let acc_before_repetition = machine.execute_until_first_repeat().unwrap();

	println!(
		"The accumulator immediately before an instruction is repeated is {}.",
		acc_before_repetition
	);
}

pub fn solve_2() {
	let input = include_str!("input.txt");
	let instructions = compile::compile(input);
	todo!()
}
