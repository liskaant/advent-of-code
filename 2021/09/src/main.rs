use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use ansi_term::{Style, Colour::Green};

type Point = u32;
type Coord = (usize, usize);
type CoordSet = HashSet<Coord>;

#[derive(Debug)]
enum Dir {
	L, R, U, D,
}

#[derive(Debug)]
struct Map(Vec<Vec<Point>>);

impl Map {
	fn new() -> Self {
		Self(Vec::new())
	}

	fn highlight(&self, indexes: &CoordSet) -> String {
		format!(
			"--- Map: {} ---\n{}",
			self.0.len(),
			self.0.iter()
				.enumerate()
				.map(|(i, row)| {
					row.iter()
						.enumerate()
						.map(|(j, c)| {
							if indexes.contains(&(i, j)) {
								Style::new().fg(Green).bold().paint(c.to_string()).to_string()
							} else {
								c.to_string()
							}
						})
						.collect::<Vec<String>>().concat()
				})
				.collect::<Vec<String>>().join("\n")
		)
	}

	fn move_coord(&self, coord: Coord, dir: Dir) -> Option<Coord> {
		match dir {
			Dir::L if coord.0 != 0                         => Some((coord.0 - 1, coord.1)),
			Dir::R if coord.0 != self.0.len() - 1          => Some((coord.0 + 1, coord.1)),
			Dir::U if coord.1 != 0                         => Some((coord.0, coord.1 - 1)),
			Dir::D if coord.1 != self.0[coord.0].len() - 1 => Some((coord.0, coord.1 + 1)),
			_ => None,
		}
	}

	fn coord_higher(&self, coord: Option<Coord>, point: Point) -> bool {
		match coord {
			None => true,
			Some(coord) => self.find(coord) > point,
		}
	}

	fn minimums(&self) -> CoordSet {
		self.0.iter()
			.enumerate().flat_map(|(i, row)| {
				row.iter()
					.enumerate().map(|(j, &c)| {
						self.coord_higher(self.move_coord((i, j), Dir::L), c)
							&& self.coord_higher(self.move_coord((i, j), Dir::R), c)
							&& self.coord_higher(self.move_coord((i, j), Dir::U), c)
							&& self.coord_higher(self.move_coord((i, j), Dir::D), c)
					})
					.enumerate().filter_map(|(j, c)| match c {
						true => Some((i, j)),
						false => None,
					})
					.collect::<Vec<Coord>>()
			})
			.collect()
	}

	fn find(&self, coord: Coord) -> Point {
		self.0[coord.0][coord.1]
	}

	fn expand_basin(&self, set: CoordSet) -> CoordSet {
		set.into_iter()
			.map(|coord| {
				[
					self.move_coord(coord, Dir::L).filter(|v| self.find(*v) != 9),
					self.move_coord(coord, Dir::R).filter(|v| self.find(*v) != 9),
					self.move_coord(coord, Dir::U).filter(|v| self.find(*v) != 9),
					self.move_coord(coord, Dir::D).filter(|v| self.find(*v) != 9),
					Some(coord),
				].iter().filter_map(|v| *v).collect::<CoordSet>()
			})
			.flatten()
			.collect()
	}

	fn basins(&self, minimums: &CoordSet) -> Vec<CoordSet> {
		minimums.iter()
			.map(|min| {
				let mut basin = CoordSet::from([*min]);
				let mut last_size = 0;

				while last_size != basin.len() {
					last_size = basin.len();
					basin = self.expand_basin(basin);
				}

				basin
			})
			.collect()
	}
}

impl ToString for Map {
	fn to_string(&self) -> String {
		self.highlight(&[].into())
	}
}

fn main() {
	let mut map = Map::new();

	for line in io::stdin().lock().lines() {
		map.0.push(
			line.unwrap().chars()
				.map(|c| c.to_digit(10).unwrap())
				.collect()
		);
	}

	let min_coords = map.minimums();
	let risk_map: HashMap<Coord, Point> = min_coords.iter()
		.map(|i| (*i, map.find(*i) + 1))
		.collect();
	let risk_level: u32 = risk_map.values().sum();

	let mut basins: Vec<(CoordSet, usize)> = map.basins(&min_coords).into_iter()
		.map(|basin| {
			let len = basin.len();
			(basin, len)
		})
		.collect();
	basins.sort_by_key(|basin| basin.1);
	let top_basin_product: usize = basins.iter().rev().map(|b| b.1).take(3).reduce(|c, b| c * b).unwrap_or(0);

	basins.iter().rev().take(3).for_each(|b| println!("--- Basin: {} {}", b.1, map.highlight(&b.0)));
	println!("{}", map.highlight(&min_coords));
	println!("Risk level: {}", risk_level);
	println!("Top 3 basin product: {}", top_basin_product);
}
