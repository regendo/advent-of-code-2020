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
	let grid = tile::grid(include_str!("input.txt"));
	let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	let mult: usize = steps
		.iter()
		.map(|(x, y)| tile::Point2D::new(*x, *y))
		.map(|step| {
			tile::traverse(grid.clone(), step)
				.iter()
				.filter(|&&t| t == tile::Tile::Tree)
				.count()
		})
		.product();

	println!(
		"Multiplying all trees in all slopes results in {} Tree^5.",
		mult
	);
}

#[cfg(test)]
mod tests {
	use super::tile;

	#[test]
	fn example_1_works() {
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

	#[test]
	fn example_2_works() {
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
		let expected: Vec<usize> = vec![2, 7, 3, 4, 2];
		let found: Vec<usize> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
			.iter()
			.map(|(x, y)| tile::Point2D::new(*x, *y))
			.map(|step| {
				tile::traverse(grid.clone(), step)
					.iter()
					.filter(|&&t| t == tile::Tile::Tree)
					.count()
			})
			.collect();

		assert_eq!(expected, found)
	}
}
