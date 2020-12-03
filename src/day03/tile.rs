use crate::helpers;
use std::convert::TryFrom;

pub use helpers::Point2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
	Tree,
	Open,
}

impl TryFrom<char> for Tile {
	type Error = String;

	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'#' => Ok(Tile::Tree),
			'.' => Ok(Tile::Open),
			_ => Err(format!("Can't construct tile from {}.", c)),
		}
	}
}

pub fn grid(input: &str) -> Vec<Vec<Tile>> {
	helpers::grid::<Tile>(input)
}

pub fn traverse(grid: Vec<Vec<Tile>>, step: Point2D) -> Vec<Tile> {
	let mut pos = Point2D::new(0, 0);
	let mut yields = vec![];

	pos += step;
	while (pos.y as usize) < grid.len() {
		yields.push(grid[(pos.y as usize)][(pos.x as usize) % grid[pos.y as usize].len()]);
		pos += step;
	}

	yields
}
