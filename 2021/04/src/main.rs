use std::io::{self, BufRead};
use ansi_term::Style;

#[derive(Debug, Copy, Clone)]
struct Value {
	number: u32,
	marked: bool,
}

impl Value {
	fn new() -> Self {
		Self {
			number: 0,
			marked: false,
		}
	}
}

impl ToString for Value {
	fn to_string(&self) -> String {
		if self.marked {
			return format!("{: <2}", Style::new().bold().paint(self.number.to_string()));
		} else {
			return format!("{: <2}", self.number.to_string());
		}
	}
}

#[derive(Debug)]
struct Board<const N: usize> {
	values: [[Value; N]; N],
	fill_rows: [usize; N],
	fill_columns: [usize; N],
	pushed_rows: usize,
	won: bool,
}

impl<const N: usize> Board<N> {
	fn new() -> Self {
		Self {
			values: [[Value::new(); N]; N],
			fill_rows: [0; N],
			fill_columns: [0; N],
			pushed_rows: 0,
			won: false,
		}
	}

	fn push_row(&mut self, row: &str) {
		let mut cols = row.split_whitespace();

		for i in 0..N {
			self.values[self.pushed_rows][i].number = cols.next().unwrap().parse().unwrap();
		}

		self.pushed_rows += 1;
	}

	fn mark(&mut self, number: u32) {
		for i in 0..N {
			for l in 0..N {
				if self.values[i][l].number == number {
					self.values[i][l].marked = true;
					self.fill_rows[i] += 1;
					self.fill_columns[l] += 1;
				}
			}
		}
	}

	fn check_win(&mut self) -> bool {
		for i in 0..N {
			if self.fill_rows[i] == N || self.fill_columns[i] == N {
				self.won = true;
				return true;
			}
		}

		false
	}
}

impl<const N: usize> ToString for Board<N> {
	fn to_string(&self) -> String {
		let mut out: String = String::new();

		for i in 0..N {
			for l in 0..N {
				out += &self.values[i][l].to_string();
				out += " ";
			}

			out.pop();
			out += "\n";
		}

		out.pop();
		out
	}
}

fn main() {
	let mut boards = Vec::<Board<5>>::new();
	let mut numbers = Vec::<u32>::new();
	let stdin = io::stdin();
	let mut input = stdin.lock().lines();

	// Drawn numbers
	for number in input.next().unwrap().unwrap().split(",") {
		numbers.push(number.parse().unwrap());
	}

	// Boards
	for line in input {
		let line = line.unwrap();

		if line == "" {
			if boards.len() != 0 {
				println!("\nLoaded board:\n{}", boards.last().unwrap().to_string());
			}

			boards.push(Board::new());
		} else {
			boards.last_mut().unwrap().push_row(&line);
		}
	}
	println!("\nLoaded board:\n{}", boards.last().unwrap().to_string());

	// Mark numbers
	println!();
	for number in numbers {
		println!("Marking number: {}", number);

		for board in boards.iter_mut() {
			board.mark(number);

			if !board.won && board.check_win() {
				println!("\nWinning board:\n{}", board.to_string());
			}
		}
	}
}
