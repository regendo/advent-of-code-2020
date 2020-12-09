use super::compile::Instruction;
use std::convert::TryFrom;

pub(crate) struct Machine {
	accumulator: i32,
	instruction_pointer: usize,
	instructions: Vec<Instruction>,
}

impl Machine {
	pub fn new(instructions: &[Instruction]) -> Self {
		Self {
			accumulator: 0,
			instruction_pointer: 0,
			instructions: instructions.to_owned(),
		}
	}

	fn execute_instruction(&mut self) {
		match self.instructions.get(self.instruction_pointer) {
			Some(Instruction::Noop) => self.instruction_pointer += 1,
			Some(Instruction::Accumulate(val)) => {
				self.accumulator += val;
				self.instruction_pointer += 1;
			}
			Some(Instruction::Jump(offset)) => {
				self.instruction_pointer =
					usize::try_from(i32::try_from(self.instruction_pointer).unwrap() + offset)
						.unwrap()
			}
			None => panic!("Out of bounds!"),
		}
	}

	pub(crate) fn execute_until_first_repeat(&mut self) -> i32 {
		let mut visited = vec![false; self.instructions.len()];
		while let Some(false) = visited.get(self.instruction_pointer) {
			*visited.get_mut(self.instruction_pointer).unwrap() = true;
			self.execute_instruction();
		}
		self.accumulator
	}
}
