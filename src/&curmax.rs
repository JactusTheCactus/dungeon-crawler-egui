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
