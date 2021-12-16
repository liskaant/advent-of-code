use std::io::{ self, BufRead };
use anyhow::{ Result, anyhow };
use std::collections::HashSet;
use std::fmt;
use std::cmp;
use regex::Regex;
use lazy_static::lazy_static;

type Coord = (u32, u32);

struct State {
	dots: HashSet<Coord>,
}

impl State {
	fn coord_bound(&self) -> Coord {
		let max = self.dots.iter()
			.fold(None, |max, (x, y)| {
				match max {
					Some((max_x, max_y)) => Some((cmp::max(max_x, *x), cmp::max(max_y, *y))),
					None => Some((*x, *y)),
				}
			});

		match max {
			Some((x, y)) => (x + 1, y + 1),
			None => (0, 0),
		}
	}

	#[allow(unused)]
	fn coords(&self) -> impl Iterator<Item=Coord> {
		let max = self.coord_bound();
		(0..max.0).map(move |x| (0..max.1).map(move |y| (x, y))).flatten()
	}

	fn from_lines<T: AsRef<str>, I: Iterator<Item=T>>(lines: &mut I) -> Result<Self> {
		let dots = lines
			.by_ref()
			.take_while(|line| !line.as_ref().is_empty())
			.filter_map(|v| {
				let mut split = v.as_ref().split(",");
				let coord = (
					split.next().and_then(|v| v.parse::<u32>().ok()),
					split.next().and_then(|v| v.parse::<u32>().ok()),
				);

				match coord {
					(Some(x), Some(y)) => Some((x, y)),
					_ => None,
				}
			})
			.collect();

		Ok(Self { dots })
	}

	fn fold(&mut self, fold: &Fold) {
		let new_dots: Vec<_> = match fold.along {
			FoldAlong::X => {
				self.dots.iter()
					.filter(|d| d.0 > fold.coord)
					.cloned()
					.filter_map(|d| {
						match (2 * fold.coord).checked_sub(d.0) {
							Some(x) => Some((x, d.1)),
							None => None,
						}
					})
					.collect()
			},
			FoldAlong::Y => {
				self.dots.iter()
					.filter(|d| d.1 > fold.coord)
					.cloned()
					.filter_map(|d| {
						match (2 * fold.coord).checked_sub(d.1) {
							Some(y) => Some((d.0, y)),
							None => None,
						}
					})
					.collect()
			},
		};

		match fold.along {
			FoldAlong::X => self.dots.retain(|d| d.0 < fold.coord),
			FoldAlong::Y => self.dots.retain(|d| d.1 < fold.coord),
		}

		new_dots.into_iter().for_each(|d| {
			self.dots.insert(d);
		});
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let max = self.coord_bound();
		let dots = (0..max.1).map(move |y| {
				(0..max.0).map(move |x| match self.dots.contains(&(x, y)) {
					true => "#",
					false => ".",
				})
				.collect::<Vec<_>>()
				.join("")
			})
			.collect::<Vec<_>>()
			.join("\n");

		write!(f, "{}", dots)
	}
}

#[derive(Debug)]
enum FoldAlong {
	X, Y,
}

#[derive(Debug)]
struct Fold {
	along: FoldAlong,
	coord: u32,
}

impl Fold {
	fn from_str<T: AsRef<str>>(input: T) -> Result<Self> {
		lazy_static! {
			static ref REGEX: Regex = Regex::new("^fold along (x|y)=(\\d+)$").unwrap();
		}

		let err = || anyhow!("Invalid fold format: {}", input.as_ref());
		let capture = REGEX.captures(input.as_ref()).ok_or_else(err)?;

		let along = match capture.get(1).ok_or_else(err)?.as_str() {
			"x" => FoldAlong::X,
			"y" => FoldAlong::Y,
			_ => return Err(err()),
		};

		let coord = capture.get(2).ok_or_else(err)?.as_str().parse().map_err(|_| err())?;

		Ok(Self { along, coord })
	}
}

fn main() -> Result<()> {
	let stdin = io::stdin();
	let mut input = stdin.lock().lines()
		.filter_map(|l| l.ok());
	let mut state = State::from_lines(&mut input)?;
	let folds: Vec<_> = input.filter_map(|l| Fold::from_str(l).ok()).collect();

	println!("Start state:\n{}", state);
	for fold in folds {
		state.fold(&fold);
		println!("After fold: {:?}:\n{}", fold, state);
	}

	Ok(())
}
