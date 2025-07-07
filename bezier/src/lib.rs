use std::ops::{Add, Index, IndexMut, Sub};

#[derive(Debug, Clone, Default)]
pub struct BezierCurve {
	points: Vec<BezierPoint>,
}

impl BezierCurve {
	pub fn new() -> Self {
		Self { points: Vec::new() }
	}

	pub fn add_point(&mut self, p: BezierPoint) {
		self.points.push(p);
	}

	pub fn get(&self, idx: usize) -> Option<&BezierPoint> {
		self.points.get(idx)
	}

	pub fn get_mut(&mut self, idx: usize) -> Option<&mut BezierPoint> {
		self.points.get_mut(idx)
	}

	pub fn mesh(&self) -> BezierCurveMesh {
		BezierCurveMesh {
			curve: self,
			idx: 0,
		}
	}
}

impl Index<usize> for BezierCurve {
	type Output = BezierPoint;
	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl IndexMut<usize> for BezierCurve {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

pub struct BezierCurveMesh<'a> {
	curve: &'a BezierCurve,
	idx: usize,
}

impl Iterator for BezierCurveMesh<'_> {
	type Item = ((f32, f32), (f32, f32));
	fn next(&mut self) -> Option<Self::Item> {
		// TODO: Temp Impl repalce with actual code

		let v = Some((
			(200.0 + (self.idx * 200) as f32, 200.0),
			(200.0 + (self.idx * 200) as f32, 400.0),
		));
		self.idx += 1;

		v
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}
}

impl Add for Point {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub for Point {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl From<(f32, f32)> for Point {
	fn from(value: (f32, f32)) -> Self {
		Self {
			x: value.0,
			y: value.1,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct BezierPoint {
	origin: Point,
	handle_a: Point,
	handle_b: Point,
}

impl From<((f32, f32), (f32, f32))> for BezierPoint {
	fn from(value: ((f32, f32), (f32, f32))) -> Self {
		let origin = value.0.into();
		let p = value.1.into();

		Self {
			origin,
			handle_a: p,
			handle_b: origin + (p - origin),
		}
	}
}

impl From<((f32, f32), (f32, f32), (f32, f32))> for BezierPoint {
	fn from(value: ((f32, f32), (f32, f32), (f32, f32))) -> Self {
		Self {
			origin: value.0.into(),
			handle_a: value.1.into(),
			handle_b: value.2.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn foo() {}
}
