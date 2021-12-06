use std::io::{self, BufRead};
use ansi_term::Style;

#[derive(Debug)]
struct State<const M: usize, const N: usize>{
	queue: [u64; M],
	new: [u64; M],
	next: usize,
}

impl<const M: usize, const N: usize> State<M, N> {
	fn new() -> Self {
		Self {
			queue: [0; M],
			new: [0; M],
			next: 0,
		}
	}

	fn push(&mut self, timer: u64) {
		assert!((timer as usize) < M + N);

		if (timer as usize) < M {
			self.queue[(self.next + timer as usize) % M] += 1;
		} else {
			self.new[(self.next + timer as usize - M) % M] += 1;
		}
	}

	fn day(&mut self) {
		let new_count = self.new[self.next];
		self.new[(self.next + N) % M] += self.queue[self.next];
		self.queue[self.next] += new_count;
		self.new[self.next] = 0;
		self.next = (self.next + 1) % M;
	}

	fn total(&self) -> u64 {
		self.queue.iter().sum::<u64>() + self.new.iter().sum::<u64>()
	}
}

impl<const M: usize, const N: usize> ToString for State<M, N> {
	fn to_string(&self) -> String {
		format!(
			"{} = {}",
			(0..M)
				.map(|i| {
					if i == self.next {
						Style::new().bold()
							.paint(format!("{}+{}", self.queue[i], self.new[i]))
							.to_string()
					} else {
						format!("{}+{}", self.queue[i], self.new[i])
					}
				})
				.collect::<Vec<String>>()
				.join(" "),
			self.total()
		)
	}
}

fn main() {
	let mut state = State::<7, 2>::new();
	let stdin = io::stdin();
	let mut input = stdin.lock().lines();

	for number in input.next().unwrap().unwrap().split(",") {
		state.push(number.parse().unwrap());
	}

	println!("{}", state.to_string());

	for day in 0..256 {
		state.day();
		println!("{}: {}", day, state.to_string());
	}
}
