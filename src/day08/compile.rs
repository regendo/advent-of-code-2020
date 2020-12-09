use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum Instruction {
	Accumulate(i32),
	Jump(i32),
	Noop(i32),
}

impl TryFrom<(&str, i32)> for Instruction {
	type Error = String;

	fn try_from(value: (&str, i32)) -> Result<Self, Self::Error> {
		Ok(match value {
			("acc", val) => Instruction::Accumulate(val),
			("jmp", val) => Instruction::Jump(val),
			("nop", val) => Instruction::Noop(val),
			_ => Err(format!("Can't parse {:?} as an instruction!", value))?,
		})
	}
}

fn split_line(line: &str) -> (&str, i32) {
	let mut split = line.trim().split_whitespace();
	if let (Some(left), Some(right), None) = (split.next(), split.next(), split.next()) {
		(left, right.parse::<i32>().unwrap())
	} else {
		panic!("Uh-oh! Line {} can't be split properly.", line)
	}
}

pub(crate) fn compile(input: &str) -> Vec<Instruction> {
	input
		.trim()
		.lines()
		.map(split_line)
		.map(|i| Instruction::try_from(i).unwrap())
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_1_compiles() {
		use super::Instruction::*;

		let input = include_str!("example_1_input.txt");

		let expected = vec![
			Noop(0),
			Accumulate(1),
			Jump(4),
			Accumulate(3),
			Jump(-3),
			Accumulate(-99),
			Accumulate(1),
			Jump(-4),
			Accumulate(6),
		];
		let actual = compile(input);
		assert_eq!(expected, actual);
	}
}
