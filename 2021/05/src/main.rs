use std::io::{self, BufRead};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type Pos = (u32, u32);

#[derive(Debug)]
struct Line {
	start: Pos,
	end: Pos,
}

impl Line {
	fn new(start: Pos, end: Pos) -> Self {
		Self {
			start: start,
			end: end,
		}
	}

	fn fields(&self) -> Vec<Pos> {
		let mut out = Vec::<Pos>::new();
		let mut pos = self.start.clone();

		while pos != self.end {
			out.push(pos);
			pos.0 = (pos.0 as i64 + (1 * (self.end.0 as i64 - pos.0 as i64).signum())) as u32;
			pos.1 = (pos.1 as i64 + (1 * (self.end.1 as i64 - pos.1 as i64).signum())) as u32;
		}

		out.push(pos);
		out
	}
}

impl FromStr for Line {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		lazy_static! {
			static ref REGEX: Regex = Regex::new("^(\\d+),(\\d+) -> (\\d+),(\\d+)$").unwrap();
		}

		let capture = REGEX.captures(input).unwrap();

		let start_x = capture.get(1).unwrap().as_str().parse().unwrap();
		let start_y = capture.get(2).unwrap().as_str().parse().unwrap();
		let end_x = capture.get(3).unwrap().as_str().parse().unwrap();
		let end_y = capture.get(4).unwrap().as_str().parse().unwrap();

		Ok(Self::new((start_x, start_y), (end_x, end_y)))
	}
}

#[derive(Debug)]
struct State {
	fields: HashMap<Pos, u32>,
}

impl State {
	fn new() -> Self {
		Self {
			fields: HashMap::new(),
		}
	}

	fn push(&mut self, line: &Line) {
		for field in line.fields() {
			match self.fields.get_mut(&field) {
				Some(field) => {
					*field += 1;
				},
				None => {
					self.fields.insert(field, 1);
				},
			}
		}
	}

	fn multi_fields(&self, treshold: u32) -> HashMap<Pos, u32> {
		self.fields.clone()
			.into_iter()
			.filter(|val| val.1 >= treshold)
			.collect()
	}
}

fn main() {
	let mut state = State::new();

	for line in io::stdin().lock().lines() {
		let line = Line::from_str(&line.unwrap()).unwrap();
		println!("Line: {:?}, Fields: {:?}", line, line.fields().len());
		state.push(&line);
	}

	let multi_fields = state.multi_fields(2);

	println!("{:?}", multi_fields.len());
}
