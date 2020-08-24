mod complex;
mod errors;

use std::cmp;
use std::env;
use std::fmt;

#[derive(Debug)]
struct Polynom {
    coef: f64,
    exponent: u8,
}

impl fmt::Display for Polynom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} * X^{}", self.coef, self.exponent)
    }
}

impl cmp::PartialEq for Polynom {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef && self.exponent == other.exponent
    }
}

fn extract_polynom(raw_input: &str, mut i: usize) -> Result<(Polynom, usize), errors::BadFormat> {
    let sign = if i == 0 {
        1.0
    } else {
        match raw_input.chars().nth(i).ok_or(errors::BadFormat)? {
            '-' => {
                i += 2;
                -1.0
            }
            '+' => {
                i += 2;
                1.0
            }
            '=' => {
                i += 2;
                1.0
            }
            _ => 1.0,
        }
    };
    let sep = i + raw_input
        .get(i..)
        .ok_or(errors::BadFormat)?
        .find(' ')
        .ok_or(errors::BadFormat)?;
    let coef = sign
        * raw_input
            .get(i..sep)
            .ok_or(errors::BadFormat)?
            .parse::<f64>()
            .or(Err(errors::BadFormat))?;
    let exponent = raw_input
        .get(sep + 5..sep + 6)
        .ok_or(errors::BadFormat)?
        .parse::<u8>()
        .or(Err(errors::BadFormat))?;
    Ok((
        Polynom {
            coef: coef,
            exponent: exponent,
        },
        sep + 7,
    ))
}

fn parse(raw_input: &str) -> Result<Vec<Polynom>, errors::BadFormat> {
    let mut ret = vec![
		Polynom {
			coef: 0.0,
			exponent: 0,
		},
		Polynom {
			coef: 0.0,
			exponent: 1,
		},
		Polynom {
			coef: 0.0,
			exponent: 2,
		},
	];
    let mut i = 0;
    while i < raw_input.len() {
        let r = extract_polynom(raw_input, i)?;
        let index = match ret.iter().position(|p| p.exponent == r.0.exponent) {
            Some(i) => i,
            None => {
                ret.push(Polynom {
                    coef: 0.0,
                    exponent: r.0.exponent,
                });
                ret.len() - 1
            }
        };
        match raw_input[i + 1..].find('=') {
            Some(_) => ret[index].coef += r.0.coef,
            None => ret[index].coef -= r.0.coef,
        }
        i = r.1;
    }
	ret.sort_unstable_by(|a, b| a.exponent.cmp(&b.exponent)); 
    Ok(ret)
}

fn print_reduced_form(polynoms: &Vec<Polynom>) {
	let last = match polynoms.iter().rposition(|p| p.coef != 0.0) {
		Some(i) => i + 1,
		None => 1,
	};
    print!("Reduced form: {}", polynoms[0]);
    for polynom in polynoms[1..last].iter() {
        if polynom.coef < 0.0 {
            print!(" - {} * X^{}", -1.0 * polynom.coef, polynom.exponent);
        } else {
            print!(" + {}", polynom);
        }
    }
    println!(" = 0");
}

fn resolve_2nd_degree(a: f64, b: f64, c: f64) {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant == 0.0 {
        println!("Discriminant is equal to 0. The solution is:");
        println!("{}", -1.0 * b / 2.0 * a);
    } else if discriminant > 0.0 {
        println!("Discriminant strictly positive, the two solutions are:");
        println!("{}", (-1.0 * b - discriminant.sqrt()) / (2.0 * a));
        println!("{}", (-1.0 * b + discriminant.sqrt()) / (2.0 * a));
    } else {
        println!("Discriminant strictly negative, there is two complex solutions:");
        println!(
            "{}",
            complex::Complex {
                real_part: -1.0 * b / (2.0 * a),
                imaginary_part: -1.0 * (-1.0 * discriminant).sqrt() / (2.0 * a)
            }
        );
        println!(
            "{}",
            complex::Complex {
                real_part: -1.0 * b / (2.0 * a),
                imaginary_part: (-1.0 * discriminant).sqrt() / (2.0 * a)
            }
        );
    }
}

fn resolve(polynoms: &Vec<Polynom>) {
    let degree = polynoms[match polynoms.iter().rposition(|p| p.coef != 0.0) {
        Some(i) => i,
        None => 0,
    }]
    .exponent;
    println!("Polynomial degree: {}", degree);

    let (a, b, c) = (polynoms[2].coef, polynoms[1].coef, polynoms[0].coef);
    if degree == 2 {
        resolve_2nd_degree(a, b, c);
    } else if degree == 1 {
        println!("The solution is:\n{}", -1.0 * c / b);
    } else if degree == 0 {
        if c == 0.0 {
            println!("Any number is a valid solution.")
        } else {
            println!("There is no solution.")
        }
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
        }
    };
    print_reduced_form(&polynoms);
    resolve(&polynoms);
}

#[cfg(test)]
mod parse_tests {
    #[test]
    fn simple_degree_2() {
        let polynoms = crate::parse("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 4.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: 4.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: -9.3,
                exponent: 2
            }
        );
    }

    #[test]
    fn simple_degree_1() {
        let polynoms = crate::parse("5 * X^0 + 4 * X^1 = 4 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 1.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: 4.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: 0.0,
                exponent: 2
            }
        );
    }

    #[test]
    fn degree_3() {
        let polynoms = crate::parse("8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 5.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: -6.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: 0.0,
                exponent: 2
            }
        );
        assert_eq!(
            polynoms[3],
            crate::Polynom {
                coef: -5.6,
                exponent: 3
            }
        );
    }

    #[test]
    #[should_panic]
    fn input_format_error() {
        crate::parse("5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 *  X^0").unwrap();
    }

    #[test]
    fn noisy_degree_1() {
        let polynoms = crate::parse("5 * X^0 + 4 * X^1 + 0 * X^2 = 4 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 1.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: 4.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: 0.0,
                exponent: 2
            }
        );
    }

    #[test]
    fn noisy_degree_2() {
        let polynoms = crate::parse("5 * X^0 + 4 * X^1 - 5 * X^2 + 3 * X^2 = 4 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 1.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: 4.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: -2.0,
                exponent: 2
            }
        );
    }

    #[test]
    fn shuffled_degree_2() {
        let polynoms = crate::parse("4 * X^1 - 9.3 * X^2 + 5 * X^0 = 1 * X^0").unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 4.0,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: 4.0,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: -9.3,
                exponent: 2
            }
        );
    }

    #[test]
    fn big_degree_2() {
        let polynoms =
            crate::parse("5 * X^0 + 4 * X^1 - 9.3 * X^2 = -4.3 * X^0 + 40.1 * X^1 + 6.5 * X^2")
                .unwrap();
        assert_eq!(
            polynoms[0],
            crate::Polynom {
                coef: 9.3,
                exponent: 0
            }
        );
        assert_eq!(
            polynoms[1],
            crate::Polynom {
                coef: -36.1,
                exponent: 1
            }
        );
        assert_eq!(
            polynoms[2],
            crate::Polynom {
                coef: -15.8,
                exponent: 2
            }
        );
    }
}
