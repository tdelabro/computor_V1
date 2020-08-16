mod complex;
mod errors;

use std::env;
use std::fmt;

struct Polynom {
	coef: f64,
	exponent: u8,
}

impl fmt::Display for Polynom {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * X^{}", self.coef, self.exponent)
    }
}

fn extract_polynom(raw_input: &str, mut i: usize) 
	-> Result<(Polynom, usize), errors::BadFormat> {
	let sign = match raw_input.chars().nth(i).ok_or(errors::BadFormat)? {
		'-' => {
			i += 2;
			-1.0
		},
		'+' => {
			i += 2;
			1.0
		},
		'=' => {
			i += 2;
			1.0
		},
		_ => 1.0,
	};
	let sep = i + raw_input
		.get(i..).ok_or(errors::BadFormat)?
		.find(' ').ok_or(errors::BadFormat)?;
	let coef = sign * raw_input
		.get(i..sep).ok_or(errors::BadFormat)?
		.parse::<f64>().or(Err(errors::BadFormat))?;
	let exponent = raw_input
		.get(sep + 5..sep + 6).ok_or(errors::BadFormat)?
		.parse::<u8>().or(Err(errors::BadFormat))?;
	Ok((Polynom {coef: coef, exponent: exponent}, sep + 7))
}

fn parse(raw_input: &str) -> Result<[Polynom; 3], errors::ParsingError> {
	let mut ret = [
		Polynom {coef: 0.0, exponent: 0},
		Polynom {coef: 0.0, exponent: 1},
		Polynom {coef: 0.0, exponent: 2},
	];
	let mut i = 0;
	while i < raw_input.len() {
		let r = extract_polynom(raw_input, i)?;
		if r.0.exponent > 2 { 
			return Err(errors::ParsingError::DegreeError(
					errors::DegreeTooHigh { degree: r.0.exponent }));
		}
		match raw_input[i..].find('=') {
			Some(_) => ret[r.0.exponent as usize].coef += r.0.coef,
			None => ret[r.0.exponent as usize].coef -= r.0.coef,
		}
		i = r.1;
	}
	Ok(ret)
}

fn print_reduced_form(polynoms: &[Polynom; 3]) {
	print!("Reduced form: {}", polynoms[0]);
	for polynom in polynoms[1..].iter() {
		if polynom.coef > 0.0 {
			print!(" + {}", polynom);
		} else if polynom.coef < 0.0 {
			print!(" - {} * X^{}", -1.0 * polynom.coef, polynom.exponent);
		}
	}
	println!(" = 0");
}

fn resolve_2nd_degree(a: f64, b: f64, c: f64) {
	let discriminant = b * b - 4.0 * a * c;
	if discriminant  == 0.0 {
		println!("Discriminant is equal to 0. The solution is:");
		println!("{}", -1.0 * b / 2.0 * a);
	} else if discriminant  > 0.0 {
		println!("Discriminant strictly positive, the two solutions are:");
		println!("{}", (-1.0 * b - discriminant .sqrt()) / (2.0 * a));
		println!("{}", (-1.0 * b + discriminant .sqrt()) / (2.0 * a));
	} else {
		println!("Discriminant strictly negative, there is two complex solutions:");
		println!("{}", complex::Complex {
			real_part: -1.0 * b / (2.0 * a),
			imaginary_part: -1.0 * (-1.0 * discriminant ).sqrt() / (2.0 * a)
		});
		println!("{}", complex::Complex {
			real_part: -1.0 * b / (2.0 * a),
			imaginary_part: (-1.0 * discriminant ).sqrt() / (2.0 * a)
		});
	}
}

fn resolve(polynoms: &[Polynom; 3]) {
	let degree = polynoms[match polynoms.iter().rposition(|p| p.coef != 0.0) {
		Some(i) => i,
		None => 0,
	}].exponent;
	println!("Polynomial degree: {}", degree);

	let (a, b, c) = (polynoms[2].coef, polynoms[1].coef, polynoms[0].coef);
	if degree == 2 {
		resolve_2nd_degree(a, b, c);
	} else if degree  == 1 {
		println!("The solution is:\n{}", -1.0 * c / b);
	} else if degree == 0 {
		if c == 0.0 { println!("Any number is a valid solution.") }
		else { println!("There is no solution.") }
	} else {
		println!("The polynomial degree is stricly greater than 2, I can't solve.");
		return;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("{}", errors::MissingArgument);
		return;
	} else if args.len() > 2 {
		println!("{}", errors::TooManyArguments);
		return;
	}

	let polynoms = match parse(&args[1]) {
		Ok(p) => p,
		Err(e) => {
			println!("{}", e);
			return;
		},
	};
	print_reduced_form(&polynoms);
	resolve(&polynoms);
}

