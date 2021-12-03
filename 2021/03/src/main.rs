use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct BitString<const N: usize> {
	bits: [u8; N],
}

impl<const N: usize> BitString<N> {
	fn new(init_val: u8) -> Self {
		Self {
			bits: [init_val; N],
		}
	}
}

impl<const N: usize> FromStr for BitString<N> {
	type Err = ();

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		let mut chars = input.chars();
		let mut out = Self::new(0);

		for v in out.bits.iter_mut() {
			if chars.next().unwrap() == '1' {
				*v = 1;
			}
		}

		Ok(out)
	}
}

impl<const N: usize> ToString for BitString<N> {
	fn to_string(&self) -> String {
		let mut out: String = String::new();

		for v in self.bits.iter() {
			if *v == 1 {
				out += "1"
			} else {
				out += "0"
			}
		}

		out
	}
}

#[derive(Debug)]
struct State<const N: usize> {
	one_count: [u32; N],
	values: Vec<BitString<N>>,
}

impl<const N: usize> State<N> {
	fn new() -> Self {
		Self {
			one_count: [0; N],
			values: vec![],
		}
	}

	fn push(&mut self, input: BitString<N>) {
		for (i, v) in self.one_count.iter_mut().enumerate() {
			*v += input.bits[i] as u32;
		}

		self.values.push(input);
	}

	fn calculate_oxygen(&self) -> BitString<N> {
		let mut oxygen_list: Vec<&BitString<N>> = self.values.iter().collect();

		for i in 0..N {
			if oxygen_list.len() == 1 {
				return *oxygen_list[0];
			}

			let one_count = oxygen_list.iter().filter(|value| value.bits[i] == 1).count();
			let criteria = if (one_count as f32) >= oxygen_list.len() as f32 / 2f32 { 1 } else { 0 };
			oxygen_list.retain(|value| value.bits[i] == criteria);
		}

		*oxygen_list[0]
	}

	fn calculate_co2(&self) -> BitString<N> {
		let mut co2_list: Vec<&BitString<N>> = self.values.iter().collect();

		for i in 0..N {
			if co2_list.len() == 1 {
				return *co2_list[0];
			}

			let one_count = co2_list.iter().filter(|value| value.bits[i] == 1).count();
			let criteria = if (one_count as f32) < co2_list.len() as f32 / 2f32 { 1 } else { 0 };
			co2_list.retain(|value| value.bits[i] == criteria);
		}

		*co2_list[0]
	}

	fn output(&self) -> (BitString<N>, BitString<N>, BitString<N>, BitString<N>) {
		let mut gamma = BitString::<N>::new(0);
		let mut epsilon = BitString::<N>::new(0);

		for i in 0..N {
			if self.one_count[i] > self.values.len() as u32 / 2 {
				gamma.bits[i] = 1;
			} else {
				epsilon.bits[i] = 1;
			}
		}

		let oxygen = self.calculate_oxygen();
		let co2 = self.calculate_co2();

		(gamma, epsilon, oxygen, co2)
	}
}

fn main() {
	let mut state = State::<12>::new();

	for line in io::stdin().lock().lines() {
		let bit_string = BitString::from_str(&line.unwrap()).unwrap();
		println!("Input: {}, State: {:?}", bit_string.to_string(), (state.one_count, state.values.len()));
		state.push(bit_string);
	}

	let (gamma, epsilon, oxygen, co2) = state.output();
	println!("State: {:?}, Gamma: {}, Epsilon: {}, Oxygen: {}, CO2: {}", (state.one_count, state.values.len()), gamma.to_string(), epsilon.to_string(), oxygen.to_string(), co2.to_string());
}
