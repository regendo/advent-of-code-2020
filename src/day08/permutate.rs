use super::compile::Instruction;

pub(crate) struct Permuter {
	prev_changed: Option<usize>,
	source_instructions: Vec<Instruction>,
}

impl Permuter {
	pub fn new(instructions: &[Instruction]) -> Self {
		Self {
			prev_changed: None,
			source_instructions: instructions.to_owned(),
		}
	}
}

impl Iterator for Permuter {
	type Item = Vec<Instruction>;

	fn next(&mut self) -> Option<Self::Item> {
		let off = match self.prev_changed {
			Some(idx) => idx + 1,
			None => 0,
		};
		match self
			.source_instructions
			.iter()
			.enumerate()
			.skip(off)
			.find(|(_, &instr)| is_noop_or_jump(instr))
		{
			Some((idx, &instr)) => {
				self.prev_changed = Some(idx);
				let mut perm = self.source_instructions.clone();
				*perm.get_mut(idx).unwrap() = swap_instruction(instr);
				Some(perm)
			}
			None => None,
		}
	}
}

fn is_noop_or_jump(instruction: Instruction) -> bool {
	matches!(instruction, Instruction::Noop(_) | Instruction::Jump(_))
}

fn swap_instruction(instruction: Instruction) -> Instruction {
	match instruction {
		Instruction::Noop(val) => Instruction::Jump(val),
		Instruction::Jump(val) => Instruction::Noop(val),
		Instruction::Accumulate(_) => unimplemented!(),
	}
}
