mod binary;

use std::cmp::Ordering;

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
	let mut seats: Vec<_> = include_str!("input.txt")
		.trim()
		.lines()
		.map(binary::parse)
		.map(|instructions| binary::calculate_seat(&instructions))
		.collect();

	seats.sort_by(|a, b| match a.y.cmp(&b.y) {
		Ordering::Equal => a.x.cmp(&b.x),
		other => other,
	});

	let found = (|seats: Vec<binary::Seat>| {
		let mut iter = seats.iter().peekable();
		while let (Some(&seat), Some(&&next)) = (iter.next(), iter.peek()) {
			if seat.y == next.y {
				if seat.x + 1 == next.x {
					// Continuous
				} else {
					return binary::Seat {
						x: seat.x + 1,
						y: seat.y,
					};
				}
			} else if seat.y + 1 == next.y {
				if seat.x == 8 && next.x == 0 {
					// Continuous
				} else if seat.x == 6 && next.x == 0 {
					return binary::Seat { x: 7, y: seat.y };
				} else if seat.x == 7 && next.x == 2 {
					return binary::Seat { x: 0, y: next.y };
				}
			} else {
				panic!("Weird ordering: {:?}, {:?}", seat, next);
			}
		}
		panic!("Really should have found a seat.");
	})(seats);

	println!(
		"Found my seat: {:?} with ID {}.",
		found,
		binary::seat_id(found)
	);
}
