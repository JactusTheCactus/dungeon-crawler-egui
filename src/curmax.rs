use {
	num_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub},
	std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Debug, Default, Copy, Clone)]
pub struct CurMax<T> {
	pub cur: T,
	pub max: T,
}
impl<T: Copy + Ord> CurMax<T> {
	pub fn new(cur: T, max: T) -> Self {
		Self {
			cur: cur.min(max),
			max,
		}
	}
	pub fn set(&mut self, value: T) {
		self.cur = value.min(self.max);
	}
	pub fn get(&self) -> (T, T) {
		(self.cur, self.max)
	}
}
impl<
	T: Copy
		+ Ord
		+ Add
		+ AddAssign
		+ Sub
		+ SubAssign
		+ Mul
		+ MulAssign
		+ Div<Output = T>
		+ DivAssign
		+ SaturatingAdd
		+ SaturatingSub
		+ SaturatingMul,
> CurMax<T>
{
	pub fn add(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_add(&n).min(self.max), self.max)
	}
	pub fn add_assign(&mut self, n: T) {
		self.cur = self.add(n).cur;
	}
	pub fn sub(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_sub(&n), self.max)
	}
	pub fn sub_assign(&mut self, n: T) {
		self.cur = self.sub(n).cur;
	}
	pub fn mul(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_mul(&n).min(self.max), self.max)
	}
	pub fn mul_assign(&mut self, n: T) {
		self.cur = self.mul(n).cur;
	}
	pub fn div(&self, n: T) -> Self {
		CurMax::new(self.cur / n, self.max)
	}
	pub fn div_assign(&mut self, n: T) {
		self.cur = self.div(n).cur;
	}
}
