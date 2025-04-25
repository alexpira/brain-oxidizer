use std::env;

macro_rules! dbg {
	($e:expr) => {
		// print!($e);
    };
    ($e:expr, $($es:expr),+) => {
        // print!($e, $($es),+)
    };
}

struct State {
	pos: usize,
	values: Vec<u32>,
}

impl State {
	fn new() -> Self {
		State { pos: 0, values: Vec::new() }
	}

	fn ensure(&mut self, pos: usize) {
		while pos >= self.values.len() {
			self.values.push(0);
		}
	}

	fn repeat(&mut self, action: &str, counter: usize, count: u32) {
		dbg!("\nRPT {} {}\n", action, count);

		self.ensure(counter);
		self.ensure(self.pos);

		let bpos = self.pos;
		self.go(counter);
		self.to(count, None);
		print!("[");
		self.go(bpos);
		print!("{}", action);
		self.go(counter);
		print!("-]");
		self.values[counter] = 0;
		self.pos = 0;
	}

	fn to(&mut self, value: u32, counter: Option<usize>) {
		let target = self.pos;
		self.ensure(target);

		dbg!("\nTO {} (from {})\n", value, self.values[target]);

		while self.values[target] != value {
			if self.values[target] > value {
				let diff = self.values[target] - value;
				if counter.is_some() && diff > 20 {
					let rpt = diff / 10;
					self.repeat("----------", counter.unwrap(), rpt);
					self.values[target] -= 10*rpt;
				} else {
					self.go(target);
					print!("-");
					self.values[target] -= 1;
				}
			} else if self.values[target] < value {
				let diff = value - self.values[target];
				if counter.is_some() && diff > 20 {
					let rpt = diff / 10;
					self.repeat("++++++++++", counter.unwrap(), rpt);
					self.values[target] += 10*rpt;
				} else {
					self.go(target);
					print!("+");
					self.values[target] += 1;
				}
			}
		}
	}

	fn go(&mut self, pos: usize) {
		dbg!("\nGO {}\n", pos);
		while self.pos != pos {
			if self.pos > pos {
				print!("<");
				self.pos -= 1;
			} else if self.pos < pos {
				print!(">");
				self.pos += 1;
			}
		}
	}

	fn put(&self) {
		print!(".");
	}
}

fn braindefuck(s: String) {
	let mut state = State::new();

	state.go(1); state.to(60, Some(0));
	state.go(2); state.to(80, Some(0));
	state.go(3); state.to(100, Some(0));
	state.go(4); state.to(120, Some(0));
	state.go(5); state.to(40, Some(0));
	state.go(6); state.to(10, Some(0));

	for c in s.chars() {
		let intv = c as u32;
		let idx = if intv < 30 {
			6
		} else if intv < 50 {
			5
		} else if intv < 70 {
			1
		} else if intv < 90 {
			2
		} else {
			3
		};

		state.go(idx);
		state.to(intv, Some(0));
		state.put();
	}
}

fn main() {
	let mut args: Vec<String> = env::args().collect();
	args.remove(0);

	for arg in args {
		braindefuck(arg);
		print!("\n");
	}
}
