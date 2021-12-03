use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum CommandType {
	Forward,
	Up,
	Down,
}

#[derive(Debug)]
struct Command {
	r#type: CommandType,
	amount: u32,
}

#[derive(Debug)]
struct Position {
	horizontal: u32,
	depth: u32,
	aim: u32,
}

impl FromStr for CommandType {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"forward" => Ok(Self::Forward),
			"up" => Ok(Self::Up),
			"down" => Ok(Self::Down),
			_ => Err(()),
		}
	}
}

impl FromStr for Command {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut split = input.split_whitespace();
		let r#type = split.next().unwrap();
		let amount: u32 = split.next().unwrap().parse().unwrap();

		Ok(Command {
			r#type: CommandType::from_str(r#type).unwrap(),
			amount: amount,
		})
	}
}

impl Position {
	fn new() -> Self {
		Self {
			horizontal: 0,
			depth: 0,
			aim: 0,
		}
	}

	fn r#move(&mut self, command: &Command) {
		match command.r#type {
			CommandType::Forward => {
				self.horizontal += command.amount;
				self.depth += self.aim * command.amount;
			},
			CommandType::Up => self.aim -= command.amount,
			CommandType::Down => self.aim += command.amount,
		}
	}
}

fn main() {
	let mut position = Position::new();

	for line in io::stdin().lock().lines() {
		let command = Command::from_str(&line.unwrap()).unwrap();
		println!("{:?}, {:?}", command, position);
		position.r#move(&command);
	}

	println!("Final position: {:?}", position);
}
