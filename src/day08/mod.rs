mod compile;
mod machine;
mod permutate;

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
	let mut permutations = permutate::Permuter::new(&compile::compile(input));
	let acc_after_terminated = permutations
		.find_map(|instr| machine::Machine::new(&instr).execute_until_end().ok())
		.unwrap();

	println!(
		"The accumulator immediately after the modified program terminated is {}.",
		acc_after_terminated
	);
}
