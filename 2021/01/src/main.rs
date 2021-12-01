use std::io::{self, BufRead};

fn main() {
	let mut last_scalar: Option<u16> = None;

	let mut count_scalar = 0u16;
	let mut count_window = 0u16;

	let mut windows: [u16; 3] = [0; 3];
	let mut window_index: usize = 0;
	let mut index = 0u16;

	for line in io::stdin().lock().lines() {
		let current: u16 = line.unwrap().parse().unwrap();
		let next_window = (window_index + 1) % 3;

		if index >= 3 && windows[window_index] < windows[next_window] + current {
			count_window += 1;
		}

		// Reset window
		windows[window_index] = 0;
		window_index = next_window;
		index += 1;

		// Add current value to all windows
		for window in &mut windows {
			*window += current;
		}

		// Scalar count
		if last_scalar.is_some() && last_scalar.unwrap() < current {
			count_scalar += 1;
		}

		last_scalar = Some(current);

		println!("{} - (S: {}, W: {}), Windows: {:?}", current, count_scalar, count_window, windows);
	}

	println!("Count - S: {}, W: {}", count_scalar, count_window);
}
