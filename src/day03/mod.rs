mod tile;

pub fn solve_1() {
	let grid = tile::grid(include_str!("input.txt"));
	let step = tile::Point2D::new(3, 1);
	let tree_count = tile::traverse(grid, step)
		.iter()
		.filter(|&&t| t == tile::Tile::Tree)
		.count();

	println!(
		"Encountered {} trees starting from the top left corner.",
		tree_count
	);
}

pub fn solve_2() {
	unimplemented!()
}

#[cfg(test)]
mod tests {
	use super::tile;

	#[test]
	fn example_1() {
		let grid = tile::grid(
			r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
		);
		let step = tile::Point2D::new(3, 1);

		let traversed = tile::traverse(grid, step);
		assert_eq!(
			7,
			traversed.iter().filter(|&&t| t == tile::Tile::Tree).count()
		)
	}
}
