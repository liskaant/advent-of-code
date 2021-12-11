use std::io::{ self, BufRead };
use anyhow::{ Result, anyhow };
use std::fmt;
use std::collections::HashSet;

type OctopusGrid = Vec<Vec<Octopus>>;
type Coord = (usize, usize);

#[derive(Debug)]
struct Octopus {
	level: u8,
}

impl Octopus {
	fn from_char(level: char) -> Result<Self> {
		match level.to_digit(10) {
			None => Err(anyhow!("Character is not a valid digit: {}", level)),
			Some(level) => Ok(Self {
				level: level as u8,
			}),
		}
	}

	fn add_level(&mut self) {
		if let Some(level) = self.level.checked_add(1) {
			self.level = level;
		}
	}
}

impl fmt::Display for Octopus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.level)
	}
}

#[derive(Debug)]
struct State {
	grid: OctopusGrid,
}

impl State {
	fn from_lines<T: AsRef<str>>(lines: impl IntoIterator<Item=T>) -> Self {
		let grid: OctopusGrid = lines
			.into_iter()
			.map(|line| {
				line.as_ref().chars()
					.map(Octopus::from_char)
					.filter_map(|v| v.ok())
					.collect()
			})
			.collect();

		Self { grid }
	}

	fn coord_shift(current: Coord, shift: (isize, isize)) -> Option<Coord> {
		let x = if shift.0 >= 0 {
			current.0.checked_add(shift.0 as usize)
		} else {
			current.0.checked_sub(shift.0.abs() as usize)
		};

		let y = if shift.1 >= 0 {
			current.1.checked_add(shift.1 as usize)
		} else {
			current.1.checked_sub(shift.1.abs() as usize)
		};

		if let (Some(x), Some(y)) = (x, y) {
			Some((x, y))
		} else {
			None
		}
	}

	fn flash(&mut self, coords: Coord) {
		for shift in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
			Self::coord_shift(coords, shift)
				.map(|c| {
					self.grid.get_mut(c.0)
						.and_then(|x| x.get_mut(c.1))
						.map(|o| o.add_level());
				});
		}
	}

	fn simulate_step(&mut self) -> u32 {
		// Raise numbers
		self.grid.iter_mut()
			.flatten()
			.for_each(|o| o.add_level());

		// Flash
		let mut flashed: HashSet<Coord> = HashSet::new();
		let mut flashed_len;
		while {
			flashed_len = flashed.len();

			(0..self.grid.len()).for_each(|i| {
				(0..self.grid[i].len()).for_each(|j| {
					if self.grid[i][j].level > 9 && !flashed.contains(&(i, j)){
						self.flash((i, j));
						flashed.insert((i, j));
					}
				});
			});

			(0..self.grid.len()).for_each(|i| {
				(0..self.grid[i].len()).for_each(|j| {
					if self.grid[i][j].level > 9 {
					}
				});
			});

			flashed.len() != flashed_len
		} {};

		// Reset flashed levels
		flashed.iter()
			.for_each(|(i, j)| {
				self.grid[*i][*j].level = 0;
			});

		flashed.len() as u32
	}

	fn count(&self) -> u32 {
		self.grid.iter().flatten().count() as u32
	}
}

impl fmt::Display for State {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let grid = self.grid.iter()
			.map(|row| {
				row.iter()
					.map(|o| format!("{}", o))
					.collect::<Vec<String>>()
					.join("")
			})
			.collect::<Vec<String>>()
			.join("\n");

		write!(f, "{}", grid)
	}
}

fn main() {
	let stdin = io::stdin();
	let input = stdin.lock().lines()
		.filter_map(|l| l.ok());
	let mut state = State::from_lines(input);
	let mut flashes = 0u32;
	let mut step = 1u32;

	println!("-- Start --\n{}", state);
	loop {
		let step_flashes = state.simulate_step();
		flashes += step_flashes;
		println!("-- Step: {}, Flashes: {} --\n{}", step, flashes, state);

		if step_flashes == state.count() {
			break;
		}

		step += 1;
	}
}
