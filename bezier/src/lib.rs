use std::{
	num::FpCategory,
	ops::{Add, Index, IndexMut, Mul, Sub},
};

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

	pub fn as_slice(&self) -> &[BezierPoint] {
		self.points.as_slice()
	}

	pub fn as_mut_slice(&mut self) -> &mut [BezierPoint] {
		self.points.as_mut_slice()
	}

	pub fn mesh(&self) -> BezierCurveMesh {
		BezierCurveMesh {
			curve: self,
			idx: 0,
		}
	}

	/// Get the point allong the bezier curve for t
	///
	pub fn curve(&self, t: f32) -> Option<Point> {
		match t.classify() {
			FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => {
				if t.is_sign_negative() {
					return None;
				}
				let segment = t.trunc() as usize;
				if segment >= self.points.len() - 1 {
					return None;
				}

				let s = BezierSegment {
					point_a: &self.points[segment],
					point_b: &self.points[segment + 1],
				};

				let n = t.fract();
				Some(s.curve(n))
			}
			_ => None,
		}
	}

	pub fn derivative(&self, t: f32) -> Option<Point> {
		match t.classify() {
			FpCategory::Zero | FpCategory::Normal | FpCategory::Subnormal => {
				if t.is_sign_negative() {
					return None;
				}
				let segment = t.trunc() as usize;
				if segment >= self.points.len() - 1 {
					return None;
				}

				let s = BezierSegment {
					point_a: &self.points[segment],
					point_b: &self.points[segment + 1],
				};

				let n = t.fract();
				Some(s.derivative(n))
			}
			_ => None,
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

#[derive(Debug, Clone, Copy)]
pub struct BezierSegment<'a> {
	point_a: &'a BezierPoint,
	point_b: &'a BezierPoint,
}

impl BezierSegment<'_> {
	pub fn curve(&self, t: f32) -> Point {
		let p0 = self.point_a.origin;
		let p1 = self.point_a.handle_b;
		let p2 = self.point_b.handle_a;
		let p3 = self.point_b.origin;

		//https://en.wikipedia.org/wiki/B%C3%A9zier_curve#:~:text=The%20explicit%20form,contain%20a%20cusp.
		(1.0 - t).powi(3) * p0
			+ 3.0 * (1.0 - t).powi(2) * t * p1
			+ 3.0 * (1.0 - t) * t.powi(2) * p2
			+ t.powi(3) * p3
	}

	pub fn derivative(&self, t: f32) -> Point {
		let p0 = self.point_a.origin;
		let p1 = self.point_a.handle_b;
		let p2 = self.point_b.handle_a;
		let p3 = self.point_b.origin;

		//https://en.wikipedia.org/wiki/B%C3%A9zier_curve#:~:text=the%20derivative%20of%20the%20cubic%20bezier%20curve%20with%20respect%20to%20t%20is%20
		3.0 * (1.0 - t).powi(2) * (p1 - p0)
			+ 6.0 * (1.0 - t) * t * (p2 - p1)
			+ 3.0 * t.powi(2) * (p3 - p2)
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

		let v = if self.idx >= 64 {
			return None;
		} else {
			let p = self.curve.curve(self.idx as f32 / 64.0).unwrap();

			let offset = self
				.curve
				.derivative(self.idx as f32 / 64.0)
				.unwrap()
				.norm()
				.rotate_90_clockwise()
				* 15.0;

			let p_a = p + 1.0 * offset;
			let p_b = p + -1.0 * offset;

			((p_a.x, p_a.y), (p_b.x, p_b.y))
		};
		self.idx += 1;

		Some(v)
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

	pub fn len(&self) -> f32 {
		f32::sqrt(self.x.powi(2) + self.y.powi(2))
	}

	pub fn norm(&self) -> Self {
		let l = self.len();
		Self {
			x: self.x / l,
			y: self.y / l,
		}
	}

	pub fn rotate_90_clockwise(&self) -> Self {
		Self {
			x: self.y,
			y: -self.x,
		}
	}

	pub fn rotate_90_counter_clockwise(&self) -> Self {
		Self {
			x: -self.y,
			y: self.x,
		}
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

impl Mul<f32> for Point {
	type Output = Self;
	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Mul<Point> for f32 {
	type Output = Point;
	fn mul(self, rhs: Point) -> Self::Output {
		rhs * self
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

impl From<Point> for (f32, f32) {
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BezierPoint {
	pub origin: Point,
	pub handle_a: Point,
	pub handle_b: Point,
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
