use std::io::{ self, BufRead };
use anyhow::{ Result, anyhow };
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use derive_more::Deref;
use std::cmp;
use std::collections::BinaryHeap;

struct Node {
	risk: u8,
	neighbors: Vec<NodeRef>,
	distance: u32,
	previous: Option<NodeRef>,
}

impl Node {
	fn new(risk: u8) -> Self {
		Self {
			risk,
			neighbors: Vec::new(),
			distance: u32::MAX,
			previous: None,
		}
	}

	fn push_neighbor(&mut self, neighbor: NodeRef) {
		self.neighbors.push(neighbor);
	}
}

impl fmt::Debug for Node {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(f, "r={};d={}", self.risk, self.distance)
	}
}

#[derive(Deref, Clone)]
struct NodeRef(#[deref] Rc<RefCell<Node>>);

impl NodeRef {
	fn new(risk: u8) -> Self {
		Self(Rc::new(RefCell::new(Node::new(risk))))
	}

	fn ptr_eq(&self, other: &Self) -> bool {
		Rc::ptr_eq(self, other)
	}
}

impl fmt::Debug for NodeRef {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		let node = self.borrow();
		write!(f, "r={},d={}", node.risk, node.distance)
	}
}

struct HeapNode(u32, NodeRef);

impl cmp::Ord for HeapNode {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		other.0.cmp(&self.0)
	}
}

impl PartialOrd for HeapNode {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for HeapNode {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for HeapNode {}

#[derive(Debug)]
struct Network {
	nodes: Vec<NodeRef>,
	last_line: Vec<NodeRef>,
}

impl Network {
	fn new() -> Self {
		Self {
			nodes: Vec::new(),
			last_line: Vec::new(),
		}
	}

	fn push_row(&mut self, line: Vec<u8>) {
		// Create nodes from numbers
		let line: Vec<_> = line.into_iter()
			.map(|c| NodeRef::new(c))
			.collect();

		// Push nodes into self vec
		line.iter().cloned()
			.for_each(|node| self.nodes.push(node));

		// Push left-right neighbors
		(0..line.len() - 1)
			.for_each(|i| {
				let left = &line[i];
				let right = &line[i + 1];

				left.borrow_mut().push_neighbor(right.clone());
				right.borrow_mut().push_neighbor(left.clone());
			});

		// Push up-down neighbors
		let min_len = cmp::min(line.len(), self.last_line.len());
		(0..min_len)
			.for_each(|i| {
				let up = &self.last_line[i];
				let down = &line[i];

				up.borrow_mut().push_neighbor(down.clone());
				down.borrow_mut().push_neighbor(up.clone());
			});

		// Replace self last line
		self.last_line = line;
	}

	fn first_node(&self) -> Option<NodeRef> {
		self.nodes.first().map(|n| n.clone())
	}

	fn last_node(&self) -> Option<NodeRef> {
		self.nodes.last().map(|n| n.clone())
	}

	fn find_path(&self, source: NodeRef, target: NodeRef) -> Vec<NodeRef> {
		target.borrow_mut().distance = 0;
		let mut heap = BinaryHeap::new();

		heap.push(HeapNode(0, target.clone()));

		while let Some(HeapNode(dist, node_ref)) = heap.pop() {
			let node = node_ref.borrow();

			// Found path to target
			if node_ref.ptr_eq(&source) {
				break;
			}

			// Better path exists
			if dist > node.distance {
				continue;
			}

			for neighbor_ref in &node.neighbors {
				let mut neighbor = neighbor_ref.borrow_mut();
				let next = node.distance + node.risk as u32;

				if next < neighbor.distance {
					neighbor.distance = next;
					neighbor.previous = Some(node_ref.clone());
					heap.push(HeapNode(next, neighbor_ref.clone()))
				}
			}
		}

		// Collect result
		let mut result = Vec::new();
		let mut node = Some(source);

		while let Some(n) = node.clone() {
			result.push(n.clone());
			node = n.borrow().previous.clone();
		}

		result
	}
}

fn basic(lines: &Vec<Vec<u8>>) ->  Result<()> {
	let mut network = Network::new();
	let err = || anyhow!("Error processing network");

	// Push values
	lines.iter()
		.for_each(|l| network.push_row(l.clone()));

	let source = network.first_node().ok_or_else(err)?;
	let target = network.last_node().ok_or_else(err)?;

	let path = network.find_path(source, target);
	let path_cost = path.first().ok_or_else(err)?.borrow().distance;

	println!("Path: {:?}\nConst: {}", path, path_cost);

	Ok(())
}

fn advanced(lines: &Vec<Vec<u8>>) ->  Result<()> {
	let mut network = Network::new();
	let err = || anyhow!("Error processing network");

	// Expand lines right
	let lines: Vec<Vec<u8>> = lines.iter().cloned()
		.map(|l| {
			(0..5)
				.map(|i| {
					l.iter().cloned()
						.map(|l| l + i)
						.map(|l| if l > 9 { l - 9 } else { l })
						.collect::<Vec<u8>>()
				})
				.flatten()
				.collect()
		})
		.collect();

	// Expand lines down
	let lines: Vec<Vec<u8>> = (0..5)
		.map(|i| {
			lines.iter().cloned()
				.map(|l| {
					l.into_iter()
						.map(|x| x + i)
						.map(|x| if x > 9 { x - 9 } else { x })
						.collect::<Vec<u8>>()
				})
				.collect::<Vec<_>>()
		})
		.flatten()
		.collect();

	// Push values
	lines.iter()
		.for_each(|l| network.push_row(l.clone()));

	let source = network.first_node().ok_or_else(err)?;
	let target = network.last_node().ok_or_else(err)?;

	let path = network.find_path(source, target);
	let path_cost = path.first().ok_or_else(err)?.borrow().distance;

	println!("Path: {:?}\nConst: {}", path, path_cost);

	Ok(())
}

fn main() -> Result<()> {
	let stdin = io::stdin();

	// Load data
	let lines = stdin.lock().lines()
		.filter_map(|l| l.ok())
		.map(|l| {
			l.chars()
				.filter_map(|c| c.to_digit(10))
				.map(|d| d as u8)
				.collect()
		})
		.collect();

	// Basic version
	basic(&lines)?;

	// Advanced version
	advanced(&lines)?;

	Ok(())
}
