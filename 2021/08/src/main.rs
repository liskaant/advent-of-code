use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Segment {
	A, B, C, D, E, F, G,
}

#[derive(Debug)]
struct Display {
	mapping: HashMap<Segment, Segment>
}

#[derive(Debug, PartialEq)]
struct DisplayedNumber {
	segments: HashSet<Segment>,
}

impl Segment {
	fn from_char(input: char) -> Result<Self, ()> {
		match input.to_ascii_uppercase() {
			'A' => Ok(Self::A),
			'B' => Ok(Self::B),
			'C' => Ok(Self::C),
			'D' => Ok(Self::D),
			'E' => Ok(Self::E),
			'F' => Ok(Self::F),
			'G' => Ok(Self::G),
			_ => Err(()),
		}
	}
}

impl FromStr for DisplayedNumber {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Ok(Self {
			segments: input
				.chars()
				.take(7)
				.map(|c| Segment::from_char(c).unwrap())
				.collect()
		})
	}
}

impl DisplayedNumber {
	fn new(segments: HashSet<Segment>) -> Self {
		Self {
			segments: segments,
		}
	}

	fn to_int(&self) -> Result<u8, ()> {
		lazy_static! {
			static ref NUMBERS: [DisplayedNumber; 10] = [
				DisplayedNumber::from_str("ABCEFG").unwrap(),
				DisplayedNumber::from_str("CF").unwrap(),
				DisplayedNumber::from_str("ACDEG").unwrap(),
				DisplayedNumber::from_str("ACDFG").unwrap(),
				DisplayedNumber::from_str("BCDF").unwrap(),
				DisplayedNumber::from_str("ABDFG").unwrap(),
				DisplayedNumber::from_str("ABDEFG").unwrap(),
				DisplayedNumber::from_str("ACF").unwrap(),
				DisplayedNumber::from_str("ABCDEFG").unwrap(),
				DisplayedNumber::from_str("ABCDFG").unwrap(),
			];
		}

		for (i, number) in NUMBERS.iter().enumerate() {
			if self == number {
				return Ok(i as u8);
			}
		}

		Err(())
	}
}

impl Display {
	fn new(numbers: Vec<DisplayedNumber>) -> Self {
		let numbers: Vec<DisplayedNumber> = numbers.into_iter().take(10).collect();
		let mut mapping = HashMap::<Segment, Segment>::with_capacity(7);

		let number_1 = numbers.iter()
			.filter(|n| n.segments.len() == 2)
			.next().unwrap();

		let number_4 = numbers.iter()
			.filter(|n| n.segments.len() == 4)
			.next().unwrap();

		let number_7 = numbers.iter()
			.filter(|n| n.segments.len() == 3)
			.next().unwrap();

		// Mapping for segment A
		mapping.insert(
			number_7.segments.iter()
				.filter(|s| !number_1.segments.contains(s))
				.cloned().next().unwrap(),
			Segment::A,
		);

		// Candidates for segments C or F
		let seg_c_f: Vec<&Segment> = number_1.segments.iter().collect();

		// Candidates for segments B or D
		let seg_b_d: Vec<&Segment> = number_4.segments.iter()
			.filter(|s| !number_1.segments.contains(s))
			.collect();

		let number_2 = numbers.iter()
			.filter(|n| n.segments.len() == 5)
			.filter(|n| n.segments.iter().filter(|s| seg_c_f.contains(s)).count() == 1)
			.filter(|n| n.segments.iter().filter(|s| seg_b_d.contains(s)).count() == 1)
			.next().unwrap();

		// Mapping for segment B
		let seg_b = seg_b_d.iter()
			.filter(|s| !number_2.segments.contains(s))
			.cloned().next().unwrap().clone();
		mapping.insert(seg_b, Segment::B);

		// Mapping for segment D
		mapping.insert(
			seg_b_d.iter()
				.filter(|s| number_2.segments.contains(s))
				.cloned().next().unwrap().clone(),
			Segment::D,
		);

		// Mapping for segment C
		mapping.insert(
			seg_c_f.iter()
				.filter(|s| number_2.segments.contains(s))
				.cloned().next().unwrap().clone(),
			Segment::C,
		);

		// Mapping for segment F
		mapping.insert(
			seg_c_f.iter()
				.filter(|s| !number_2.segments.contains(s))
				.cloned().next().unwrap().clone(),
			Segment::F,
		);

		let number_5 = numbers.iter()
			.filter(|n| n.segments.len() == 5)
			.filter(|n| n.segments.contains(&seg_b))
			.next().unwrap();

		// Mapping for segment G
		mapping.insert(
			number_5.segments.iter()
				.filter(|s| !mapping.contains_key(*s))
				.cloned().next().unwrap().clone(),
			Segment::G,
		);

		// Mapping for segment E
		mapping.insert(
			number_2.segments.iter()
				.filter(|s| !mapping.contains_key(*s))
				.cloned().next().unwrap().clone(),
			Segment::E,
		);

		Self {
			mapping: mapping,
		}
	}

	fn convert(&self, number: &DisplayedNumber) -> DisplayedNumber {
		DisplayedNumber::new(
			number.segments.iter()
				.map(|s| self.mapping[s])
				.collect(),
		)
	}
}

fn main() {
	let mut count_1478 = 0u32;
	let mut sum = 0u32;

	for line in io::stdin().lock().lines() {
		let line = line.unwrap();
		let mut split = line.split("|");

		let example_numbers: Vec<DisplayedNumber> = split.next().unwrap()
			.split_whitespace()
			.map(|d| DisplayedNumber::from_str(d).unwrap())
			.collect();

		let display = Display::new(example_numbers);

		let numbers: Vec<u8> = split.next().unwrap()
			.split_whitespace()
			.map(|d| DisplayedNumber::from_str(d).unwrap())
			.map(|d| display.convert(&d))
			.map(|d| d.to_int().unwrap())
			.collect();

		count_1478 += numbers.iter()
			.filter(|n| [1, 4, 7, 8].contains(n))
			.count() as u32;

		let number = numbers.iter()
			.enumerate()
			.map(|(i, n)| *n as u32 * 10u32.pow(3 - i as u32))
			.sum::<u32>();

		sum += number;

		println!("{:?}, {:?}, {}", display, numbers, number);
	}

	println!("Count 1478: {}, Sum: {}", count_1478, sum);
}
