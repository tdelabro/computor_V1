use std::fmt;

pub struct Complex {
	pub real_part: f64,
	pub imaginary_part: f64,
}

impl fmt::Display for Complex {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} + {}i", self.real_part, self.imaginary_part)
	}
}
