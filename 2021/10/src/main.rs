use std::io::{self, BufRead};
use anyhow::{Result, anyhow};

#[derive(Debug, PartialEq, Clone, Copy)]
enum TokenKind {
	A, B, C, D,
}

impl TokenKind {
	fn broken_points(&self) -> u32 {
		match self {
			Self::A => 3,
			Self::B => 57,
			Self::C => 1197,
			Self::D => 25137,
		}
	}

	fn fix_points(&self) -> u32 {
		match self {
			Self::A => 1,
			Self::B => 2,
			Self::C => 3,
			Self::D => 4,
		}
	}
}

#[derive(Debug)]
struct Token {
	kind: TokenKind,
	closing: bool,
}

impl Token {
	fn new(kind: TokenKind, closing: bool) -> Self {
		Self {
			kind: kind,
			closing: closing,
		}
	}

	fn from_char(c: char) -> Result<Self> {
		match c {
			'(' => Ok(Self { kind: TokenKind::A, closing: false }),
			')' => Ok(Self { kind: TokenKind::A, closing: true }),
			'[' => Ok(Self { kind: TokenKind::B, closing: false }),
			']' => Ok(Self { kind: TokenKind::B, closing: true }),
			'{' => Ok(Self { kind: TokenKind::C, closing: false }),
			'}' => Ok(Self { kind: TokenKind::C, closing: true }),
			'<' => Ok(Self { kind: TokenKind::D, closing: false }),
			'>' => Ok(Self { kind: TokenKind::D, closing: true }),
			_ => Err(anyhow!("Invalid token: {}", c)),
		}
	}
}

impl ToString for Token {
	fn to_string(&self) -> String {
		match (&self.kind, self.closing) {
			(TokenKind::A, false)  => "(".to_string(),
			(TokenKind::A, true) => ")".to_string(),
			(TokenKind::B, false)  => "[".to_string(),
			(TokenKind::B, true) => "]".to_string(),
			(TokenKind::C, false)  => "{".to_string(),
			(TokenKind::C, true) => "}".to_string(),
			(TokenKind::D, false)  => "<".to_string(),
			(TokenKind::D, true) => ">".to_string(),
		}
	}
}

#[derive(Debug)]
struct Node {
	token_kind: TokenKind,
	children: Vec<usize>,
}

impl Node {
	fn new(token_kind: TokenKind) -> Self {
		Self {
			token_kind: token_kind,
			children: Vec::default(),
		}
	}
}

#[derive(Debug)]
struct Tree {
	nodes: Vec<Node>,
	root: Vec<usize>,
	stack: Vec<usize>,
}

impl Tree {
	fn new() -> Self {
		Self {
			nodes: Vec::default(),
			root: Vec::default(),
			stack: Vec::default(),
		}
	}

	fn push(&mut self, token: Token) -> Result<()> {
		match token.closing {
			false => {
				self.nodes.push(Node::new(token.kind));
				let node_id = self.nodes.len() - 1;

				match self.stack.last() {
					None => self.root.push(node_id),
					Some(current) => self.nodes[*current].children.push(node_id),
				}

				self.stack.push(node_id);
				Ok(())
			},
			true => {
				match self.stack.last() {
					Some(current) if self.nodes[*current].token_kind == token.kind => {
						self.stack.pop();
						Ok(())
					},
					_ => Err(anyhow!("Unexpected token: {}", token.to_string())),
				}
			},
		}
	}

	fn pop(&mut self) -> Option<TokenKind> {
		match self.stack.pop() {
			None => None,
			Some(current) => Some(self.nodes[current].token_kind),
		}
	}
}

fn main() {
	let lines: Vec<Vec<Token>> = io::stdin()
		.lock().lines()
		.filter_map(|line| {
			line.ok().map(|line| {
				line.chars()
					.map(Token::from_char)
					.filter_map(|v| v.ok())
					.collect()
			})
		})
		.collect();

	let (incomplete, invalid): (Vec<Option<Tree>>, Vec<u32>) = lines.into_iter()
		.map(|line| {
			let mut tree = Tree::new();

			for token in line {
				let points = token.kind.broken_points();
				match tree.push(token) {
					Ok(_) => (),
					Err(e) => {
						println!("{} - Points: {}", e, points);
						return (None, points);
					}
				}
			}

			(Some(tree), 0)
		})
		.unzip();

	let invalid: u32 = invalid.into_iter().sum();

	let mut fixed: Vec<u64> = incomplete.into_iter()
		.filter_map(|v| v)
		.map(|mut tree| {
			let mut points = 0;

			print!("Fixed: ");
			while let Some(kind) = tree.pop() {
				points = points * 5 + kind.fix_points() as u64;
				print!("{}", Token::new(kind, true).to_string());
			}

			println!();
			points
		})
		.collect();
	fixed.sort();
	let fixed = fixed[fixed.len() / 2];

	println!("Invlid: {}, Fixed: {}", invalid, fixed);
}
