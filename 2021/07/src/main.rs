use std::io::{self, BufRead};

fn abs_diff(a: u32, b: u32) -> u32 {
	if a < b {
		b - a
	} else {
		a - b
	}
}

fn fuel(a: u32, b: u32) -> u32 {
	let diff = abs_diff(a, b);
	return (diff * diff + diff) / 2;
}

fn main() {
	let mut numbers = Vec::<u32>::new();
	let stdin = io::stdin();
	let mut input = stdin.lock().lines();

	for number in input.next().unwrap().unwrap().split(",") {
		numbers.push(number.parse().unwrap());
	}

	let max = numbers.iter().max().unwrap();
	let mut range = vec![0; *max as usize + 1];

	for number in numbers.iter() {
		for (i, target) in range.iter_mut().enumerate() {
			*target += fuel(*number, i as u32)
		}
	}

	let (target, distances) = range.iter().enumerate()
		.min_by_key(|&(_, val)| val)
		.unwrap();
	println!("{}, {}", target, distances);
}
