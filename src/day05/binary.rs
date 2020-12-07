pub(crate) use helpers::Point2D as Seat;
use std::convert::TryFrom;

use crate::helpers;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SplitInstruction {
	Row(Row),
	Column(Column),
}

impl TryFrom<char> for SplitInstruction {
	type Error = String;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'F' => SplitInstruction::Row(Row::Front),
			'B' => SplitInstruction::Row(Row::Back),
			'L' => SplitInstruction::Column(Column::Left),
			'R' => SplitInstruction::Column(Column::Right),
			_ => Err(format!("Invalid direction {}.", value))?,
		})
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Row {
	Front,
	Back,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Column {
	Left,
	Right,
}

pub(crate) fn parse(input: &str) -> Vec<SplitInstruction> {
	input
		.chars()
		.map(|c| SplitInstruction::try_from(c).unwrap())
		.collect()
}

pub(crate) fn calculate_seat(instructions: &[SplitInstruction]) -> Seat {
	Seat {
		y: calculate_row(instructions.get(0..7).unwrap()),
		x: calculate_col(instructions.get(7..10).unwrap()),
	}
}

fn calculate_row(instructions: &[SplitInstruction]) -> i32 {
	let last = instructions
		.iter()
		.fold(0..=127, |rows, instr| match instr {
			SplitInstruction::Row(Row::Front) => {
				*rows.start()..=((rows.end() - rows.start()) / 2 + rows.start())
			}
			SplitInstruction::Row(Row::Back) => {
				((rows.end() - rows.start()) / 2 + rows.start() + 1)..=*rows.end()
			}
			SplitInstruction::Column(dir) => panic!(
				"Trying to split sideways towards the {:?} while determining rows!",
				dir
			),
		});
	if last.clone().count() != 1 {
		panic!(
			"Couldn't reduce space to just one row: left with {:?} for instructions {:?}.",
			last, instructions
		);
	} else {
		*last.start()
	}
}

fn calculate_col(instructions: &[SplitInstruction]) -> i32 {
	let last = instructions.iter().fold(0..=7, |cols, instr| match instr {
		SplitInstruction::Column(Column::Left) => {
			*cols.start()..=((cols.end() - cols.start()) / 2 + cols.start())
		}
		SplitInstruction::Column(Column::Right) => {
			((cols.end() - cols.start()) / 2 + cols.start() + 1)..=*cols.end()
		}
		SplitInstruction::Row(dir) => panic!(
			"Trying to split lengthwise towards the {:?} while determining columns!",
			dir
		),
	});
	if last.clone().count() != 1 {
		panic!(
			"Couldn't reduce space to just one column: left with {:?} for instructions {:?}.",
			last, instructions
		);
	} else {
		*last.start()
	}
}

pub(crate) fn seat_id(seat: Seat) -> i32 {
	seat.y * 8 + seat.x
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn seat_id_calc_works() {
		fn test(input: &str, expected: i32) {
			assert_eq!(expected, seat_id(calculate_seat(&parse(input))))
		}

		test("BFFFBBFRRR", 567);
		test("FFFBBBFRRR", 119);
		test("BBFFBBFRLL", 820);
	}
}
