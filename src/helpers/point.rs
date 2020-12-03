use std::ops::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point2D {
	pub x: i32,
	pub y: i32,
}

impl Point2D {
	pub fn new(x: i32, y: i32) -> Self {
		Self { x, y }
	}
}

impl Add<Point2D> for Point2D {
	type Output = Point2D;

	fn add(self, rhs: Point2D) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl AddAssign<Point2D> for Point2D {
	fn add_assign(&mut self, rhs: Point2D) {
		self.x = (*self + rhs).x;
		self.y = (*self + rhs).y;
	}
}

impl Sub<Point2D> for Point2D {
	type Output = Point2D;

	fn sub(self, rhs: Point2D) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}
