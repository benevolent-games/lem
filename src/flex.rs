
// use std::str::FromStr;
// use std::num::ParseFloatError;

// #[derive(Debug, PartialEq)]
// pub enum Number {
// 	Int(i64),
// 	Float(f64),
// }

// impl Number {
// 	pub fn add(self, other: Number) -> Number {
// 		match (self, other) {
// 			(Number::Int(a), Number::Int(b)) => Number::Int(a + b),
// 			(Number::Int(a), Number::Float(b)) => Number::Float(a as f64 + b),
// 			(Number::Float(a), Number::Int(b)) => Number::Float(a + b as f64),
// 			(Number::Float(a), Number::Float(b)) => Number::Float(a + b),
// 		}
// 	}
// }

// impl FromStr for Number {
// 	type Err = ParseFloatError;

// 	fn from_str(s: &str) -> Result<Self, Self::Err> {
// 		if let Ok(i) = s.parse::<i64>() {
// 			Ok(Number::Int(i))
// 		} else {
// 			s.parse::<f64>().map(Number::Float)
// 		}
// 	}
// }

