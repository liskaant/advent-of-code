use std::io::{ self, BufRead };
use anyhow::{ Result, anyhow };
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fmt;
use regex::Regex;
use lazy_static::lazy_static;

type Pair = (char, char);

struct Rule {
	from: Pair,
	to: char,
}

impl Rule {
	fn from_str<T: AsRef<str>>(input: T) -> Result<Self> {
		lazy_static! {
			static ref REGEX: Regex = Regex::new("^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
		}

		let err = || anyhow!("Invalid rule format: {}", input.as_ref());
		let capture = REGEX.captures(input.as_ref()).ok_or_else(err)?;

		let from1 = capture.get(1).ok_or_else(err)?.as_str().chars().next().ok_or_else(err)?;
		let from2 = capture.get(2).ok_or_else(err)?.as_str().chars().next().ok_or_else(err)?;
		let from = (from1, from2);
		let to = capture.get(3).ok_or_else(err)?.as_str().chars().next().ok_or_else(err)?;

		Ok(Self { from, to })
	}
}

impl fmt::Display for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{} -> {}", self.from.0, self.from.1, self.to)
	}
}

struct Polymer {
	pairs: BTreeMap<Pair, usize>,
	first: char,
	last: char,
}

impl Polymer {
	fn apply_rules(&mut self, rules: &HashMap<Pair, Rule>) {
		self.pairs = self.pairs.iter()
			.map(|(&k, &v)| {
				match rules.get(&k) {
					None => Vec::from([(k, v)]),
					Some(p) => {
						Vec::from([
							((k.0, p.to), v),
							((p.to, k.1), v),
						])
					},
				}
			})
			.flatten()
			.fold(BTreeMap::new(), |mut m, v| {
				*m.entry(v.0).or_default() += v.1;
				m
			});
	}

	fn from_str<T: AsRef<str>>(input: T) -> Self {
		let mut pairs = BTreeMap::new();
		let input: Vec<_> = input.as_ref().chars().collect();
		let first = input.iter().next().unwrap().clone();
		let last = input.iter().last().unwrap().clone();

		for (i, c) in input.iter().enumerate() {
			if i < input.len() - 1 {
				*pairs.entry((*c, input[i + 1])).or_default() += 1;
			}
		}

		Self { pairs, first, last }
	}

	fn char_counts(&self) -> Vec<(char, usize)> {
		let mut counts: Vec<_> = self.pairs.iter()
			.fold(HashMap::new(), |mut m, v| {
				*m.entry(v.0.0).or_default() += v.1;
				*m.entry(v.0.1).or_default() += v.1;
				m
			})
			.into_iter()
			.map(|(k, mut c)| {
				c = c / 2;
				if k == self.first || k == self.last {
					c += 1;
				}
				(k, c)
			})
			.collect();

		counts.sort_by_key(|v| v.1);
		counts
	}
}

impl fmt::Display for Polymer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let chars = self.pairs.iter().map(|c| format!("{}{}: {}", c.0.0, c.0.1, c.1)).collect::<Vec<_>>().join(", ");
		write!(f, "{}", chars)
	}
}

fn main() -> Result<()> {
	let stdin = io::stdin();
	let mut input = stdin.lock().lines()
		.filter_map(|l| l.ok());

	let mut polymer = Polymer::from_str(input.next().unwrap());
	let rules: HashMap<_, _> = input
		.filter_map(|l| Rule::from_str(l).ok())
		.map(|r| (r.from, r))
		.collect();

	println!("Rules:");
	rules.iter().for_each(|r| println!("{}", r.1));
	println!("Start polymer: {}", polymer);
	for i in 1..=40 {
		polymer.apply_rules(&rules);
		println!("After rules #{}: {}", i, polymer);
	}

	println!("Letter counts: {:?}", polymer.char_counts());

	Ok(())
}
