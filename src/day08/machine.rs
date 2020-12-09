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

	fn execute_instruction(&mut self) -> Result<(), String> {
		match self.instructions.get(self.instruction_pointer) {
			Some(Instruction::Noop(_)) => {
				self.instruction_pointer += 1;
				Ok(())
			}
			Some(Instruction::Accumulate(val)) => {
				self.accumulator += val;
				self.instruction_pointer += 1;
				Ok(())
			}
			Some(Instruction::Jump(offset)) => {
				self.instruction_pointer =
					usize::try_from(i32::try_from(self.instruction_pointer).unwrap() + offset)
						.unwrap();
				Ok(())
			}
			None => Err("Out of bounds!".to_owned()),
		}
	}

	pub(crate) fn execute_until_first_repeat(&mut self) -> Result<i32, String> {
		let mut visited = vec![false; self.instructions.len()];

		loop {
			match visited.get_mut(self.instruction_pointer) {
				Some(false) => {
					*visited.get_mut(self.instruction_pointer).unwrap() = true;
					self.execute_instruction()?;
				}
				Some(true) => return Ok(self.accumulator),
				None => return Err("Out of bounds!".to_owned()),
			}
		}
	}

	pub(crate) fn execute_until_end(&mut self) -> Result<i32, String> {
		match self.execute_until_first_repeat() {
			Ok(_) => {
				// Repetition detected, we don't want that.
				Err("Repeats".to_owned())
			}
			Err(_) => {
				// It didn't loop! But did it terminate correctly?
				if self.instruction_pointer == self.instructions.len() {
					Ok(self.accumulator)
				} else {
					Err(
						"Program stopped at instruction {} (should have been {}).",
						self.instruction_pointer,
						self.instructions.len(),
					)
				}
			}
		}
	}
}
