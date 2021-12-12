use std::io::{ self, BufRead };
use anyhow::{ Result, anyhow };
use std::collections::HashSet;
use std::rc::Rc;
use std::fmt;
use derefable::Derefable;

#[derive(Hash, PartialEq, Eq)]
struct Cave {
	name: String,
}

impl Cave {
	fn new<T: AsRef<str>>(name: T) -> Self {
		Self {
			name: name.as_ref().to_string(),
		}
	}

	fn is_large(&self) -> bool {
		self.name.chars().all(|c| c.is_ascii_uppercase())
	}
}

impl fmt::Display for Cave {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

#[derive(Hash, PartialEq, Eq, Clone, Derefable)]
struct CaveRef(#[deref(mutable)] Rc<Cave>);

impl CaveRef {
	fn new<T: AsRef<str>>(name: T) -> Self {
		Self(Rc::new(Cave::new(name)))
	}
}

#[derive(Hash, PartialEq, Eq)]
struct Conn {
	from: CaveRef,
	to: CaveRef,
}

impl Conn {
	fn from_str<T: AsRef<str>>(input: T) -> Result<Self> {
		let err = || anyhow!("Invalid connection format: {}", input.as_ref());
		let mut input = input.as_ref().split('-');
		let from = CaveRef::new(input.next().ok_or_else(err)?);
		let to = CaveRef::new(input.next().ok_or_else(err)?);

		Ok(Self { from, to })
	}
}

impl fmt::Display for Conn {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{}", *self.from, *self.to)
	}
}

#[derive(Clone)]
struct Path {
	caves: Vec<CaveRef>,
	double_cave: Option<CaveRef>,
}

impl Path {
	fn new() -> Self {
		Self {
			caves: Vec::new(),
			double_cave: None,
		}
	}
}

impl fmt::Display for Path {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let caves = self.caves.iter().map(|v| format!("{}", **v)).collect::<Vec<_>>().join(",");
		write!(f, "{}", caves)
	}
}

struct CaveSystem {
	connections: HashSet<Conn>,
	start: CaveRef,
	end: CaveRef,
}

impl CaveSystem {
	fn from_lines<T: AsRef<str>>(lines: impl IntoIterator<Item=T>) -> Result<Self> {
		let mut caves = HashSet::<CaveRef>::new();
		let connections: HashSet<Conn> = lines
			.into_iter()
			.filter_map(|line| {
				Conn::from_str(line.as_ref()).map_err(|e| dbg!(e)).ok()
			})
			.map(|conn: Conn| {
				caves.insert(conn.from.clone());
				caves.insert(conn.to.clone());
				conn
			})
			.collect();

		let err_start = || anyhow!("Cave system has to have a start cave");
		let err_end = || anyhow!("Cave system has to have an end cave");
		let start = caves.iter().find(|cave| cave.name == "start").ok_or_else(err_start)?.clone();
		let end = caves.iter().find(|cave| cave.name == "end").ok_or_else(err_end)?.clone();

		Ok(Self { connections, start, end })
	}

	fn calculate_single_paths(&self, mut parent: Path) -> Vec<Path> {
		if let None = parent.caves.last() {
			parent.caves.push(self.start.clone());
		}
		let start = parent.caves.last().unwrap();

		self.connections.iter()
			.filter_map(|conn| {
				if conn.from == *start {
					Some(&conn.to)
				} else if conn.to == *start {
					Some(&conn.from)
				} else {
					None
				}
			})
			.filter(|cave| cave.is_large() || !parent.caves.contains(&cave))
			.map(|cave| {
				let mut path = parent.clone();
				path.caves.push(cave.clone());

				if *cave == self.end {
					Vec::from([path])
				} else {
					self.calculate_single_paths(path)
				}
			})
			.flatten()
			.collect()
	}

	fn calculate_double_paths(&self, mut parent: Path) -> Vec<Path> {
		if let None = parent.caves.last() {
			parent.caves.push(self.start.clone());
		}
		let start = parent.caves.last().unwrap();

		self.connections.iter()
			.filter_map(|conn| {
				if conn.from == *start {
					Some(&conn.to)
				} else if conn.to == *start {
					Some(&conn.from)
				} else {
					None
				}
			})
			.filter(|cave| {
				cave.is_large()
					|| !parent.caves.contains(&cave)
					|| (parent.double_cave.is_none() && **cave != self.start)
			})
			.map(|cave| {
				let mut path = parent.clone();

				if !cave.is_large() && parent.caves.contains(&cave) {
					path.double_cave = Some(cave.clone());
				}

				path.caves.push(cave.clone());
				if *cave == self.end {
					Vec::from([path])
				} else {
					self.calculate_double_paths(path)
				}
			})
			.flatten()
			.collect()
	}
}

impl fmt::Display for CaveSystem {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let connections = self.connections.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join("\n");
		write!(f, "{}", connections)
	}
}

fn main() -> Result<()> {
	let stdin = io::stdin();
	let input = stdin.lock().lines()
		.filter_map(|l| l.ok());

	let cave_system = CaveSystem::from_lines(input)?;
	let single_paths = cave_system.calculate_single_paths(Path::new());
	let double_paths = cave_system.calculate_double_paths(Path::new());

	println!("{}", cave_system);
	println!("Single Paths: {}, Double paths: {}", single_paths.iter().count(), double_paths.iter().count());

	Ok(())
}
