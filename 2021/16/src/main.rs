use std::io::{ self, BufRead };
use bitvec::prelude::{ BitVec, Msb0, BitSlice };
use std::ops::{ AddAssign, ShlAssign };

type Msg = BitVec<Msb0, u8>;
type MsgSlice = BitSlice<Msb0, u8>;

fn extract<T>(slice: &MsgSlice) -> T where
		T: From<u8> + AddAssign<T> + ShlAssign<usize> {
	let mut result: T = 0u8.into();

	slice.iter()
		.rev()
		.enumerate()
		.filter(|(_, b)| **b)
		.for_each(|(i, _)| {
			let mut num: T = 1u8.into();
			num.shl_assign(i);
			result.add_assign(num);
		});

	result
}

#[derive(Debug)]
enum OperatorType {
	Sum,
	Product,
	Min,
	Max,
	Gt,
	Lt,
	Eq,
}

impl OperatorType {
	fn from_version(version: u8) -> Self {
		match version {
			0 => Self::Sum,
			1 => Self::Product,
			2 => Self::Min,
			3 => Self::Max,
			5 => Self::Gt,
			6 => Self::Lt,
			7 => Self::Eq,
			_ => panic!("Not a operator version: {}", version),
		}
	}

	fn eval(&self, packets: &Vec<Packet>) -> u64 {
		match self {
			Self::Sum => packets.iter().map(|p| p.eval()).sum(),
			Self::Product => packets.iter().map(|p| p.eval()).product(),
			Self::Min => packets.iter().map(|p| p.eval()).min().unwrap_or(u64::MIN),
			Self::Max => packets.iter().map(|p| p.eval()).max().unwrap_or(u64::MAX),
			Self::Gt => if packets[0].eval() > packets[1].eval() { 1 } else { 0 },
			Self::Lt => if packets[0].eval() < packets[1].eval() { 1 } else { 0 },
			Self::Eq => if packets[0].eval() == packets[1].eval() { 1 } else { 0 },
		}
	}
}

#[derive(Debug)]
enum PacketData {
	Literal {
		number: u64,
		seg_num: usize,
	},
	Operator {
		len_type: bool,
		packets: Vec<Packet>,
		optype: OperatorType,
	},
}

impl PacketData {
	fn from_message(bits: &MsgSlice) -> Self {
		match extract::<u8>(&bits[..3]) {
			4 => Self::literal_from_message(&bits[3..]),
			n => Self::operator_from_message(&bits[3..], OperatorType::from_version(n)),
		}
	}

	fn literal_from_message(mut bits: &MsgSlice) -> Self {
		let mut result = Msg::new();
		let mut seg_num = 0;

		loop {
			let end = !bits[0];
			result.extend_from_bitslice(&bits[1..5]);
			bits = &bits[5..];
			seg_num += 1;

			if end { break; }
		}

		let number = extract(&result);
		Self::Literal { number, seg_num }
	}

	fn operator_from_message(mut bits: &MsgSlice, optype: OperatorType) -> Self {
		let len_type = bits[0];
		bits = &bits[1..];
		let mut packets = Vec::new();
		let mut len = 0;

		// Extract expexted length
		let exp_len;
		if len_type {
			exp_len = extract(&bits[..11]);
			bits = &bits[11..];
		} else {
			exp_len = extract(&bits[..15]);
			bits = &bits[15..];
		}

		// Parse child packets
		loop {
			let packet = Packet::from_message(bits);
			bits = &bits[packet.len()..];
			len += packet.len();
			packets.push(packet);

			if !len_type && len == exp_len { break; }
			if len_type && packets.len() == exp_len { break; }
		}

		Self::Operator { len_type, packets, optype }
	}

	fn len(&self) -> usize {
		match self {
			Self::Literal { seg_num, .. } => 3 + (5 * seg_num),
			Self::Operator { packets, len_type, .. } => {
				let packets_len: usize = packets.iter()
					.map(|p| p.len())
					.sum();
				let size_len = if *len_type { 11 } else { 15 };

				3 + 1 + size_len + packets_len
			},
		}
	}

	fn sum_versions(&self) -> u32 {
		match self {
			Self::Literal { .. } => 0,
			Self::Operator { packets, .. } => {
				packets.iter()
					.map(|p| p.sum_versions())
					.sum()
			},
		}
	}

	fn eval(&self) -> u64 {
		match self {
			Self::Literal { number, .. } => *number,
			Self::Operator { optype, packets, .. } => optype.eval(packets),
		}
	}
}

#[derive(Debug)]
struct Packet {
	version: u8,
	data: PacketData,
}

impl Packet {
	fn from_message(bits: &MsgSlice) -> Self {
		let version = extract(&bits[0..3]);
		let data = PacketData::from_message(&bits[3..]);

		Self { version, data }
	}

	fn len(&self) -> usize {
		return 3 + self.data.len()
	}

	fn sum_versions(&self) -> u32 {
		self.data.sum_versions() + self.version as u32
	}

	fn eval(&self) -> u64 {
		self.data.eval()
	}
}

fn main() {
	let stdin = io::stdin();

	// Load data
	let line = stdin.lock().lines().next().unwrap().unwrap().into_bytes();
	let message: Msg = line.as_slice().windows(2)
		.step_by(2)
		.map(|s| (s[0] as char).to_string() + &(s[1] as char).to_string())
		.filter_map(|s| u8::from_str_radix(&s, 16).ok())
		.collect();

	let packet = Packet::from_message(&message);

	println!("Message: {:?}", message);
	println!("Packet: {:?}", packet);
	println!("Version sum: {}", packet.sum_versions());
	println!("Eval: {}", packet.eval());
}
