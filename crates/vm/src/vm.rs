use super::instr::Instr;
use anyhow::{anyhow, bail, Result};
use std::{
	io::{self, Write},
	str::Chars,
};

const MEMORY_LENGTH_LIMIT: usize = 30_000;

pub struct VM<'v> {
	pub(crate) stack: Vec<u32>,

	/// Instruction pointer
	ip: usize,
	/// Stack pointer
	sp: usize,
	loop_indexes: Vec<usize>,

	input: Chars<'v>,
	writer: Box<dyn Write>,
}

impl Default for VM<'_> {
	fn default() -> Self {
		Self {
			ip: 0,
			sp: 0,
			loop_indexes: Vec::new(),
			stack: {
				let mut vec = Vec::with_capacity(MEMORY_LENGTH_LIMIT);
				vec.push(0);
				vec
			},
			input: "".chars(),
			writer: Box::new(io::stdout()),
		}
	}
}

impl VM<'_> {
	pub fn new(input: &'static str, writer: Box<dyn Write>) -> Self {
		Self {
			input: input.chars(),
			writer,
			..Self::default()
		}
	}
	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::MoveRight(n) => {
				if let Some(result) = self.sp.checked_add(*n) {
					self.sp = result;
					if self.stack.len() < self.sp {
						for _ in self.stack.len()..=self.sp {
							self.stack.push(0);
						}
					}
				} else {
					bail!("Sorry, the index is too large")
				}
			}
			Instr::MoveLeft(n) => {
				if let Some(result) = self.sp.checked_sub(*n) {
					self.sp = result;
				} else {
					bail!("Sorry, the index is too small")
				}
			}
			Instr::Inc(n) => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					*value += n;
				} else {
					bail!("Index not found")
				}
			}
			Instr::Dec(n) => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					if let Some(n) = value.checked_sub(*n) {
						*value = n;
					} else {
						bail!("Stack values cannot be negative")
					}
				} else {
					bail!("Index not found")
				}
			}
			Instr::LoopStart => {
				self.loop_indexes.push(self.sp);
			}
			Instr::LoopEnd => {
				if let Some(latest_loop_sp) = self.loop_indexes.last() {
					if let Some(latest_loop_index) = self.stack.get(*latest_loop_sp) {
						if *latest_loop_index != 0 {
							self.sp = *latest_loop_index as usize;
						} else {
							let _ = self.loop_indexes.pop();
							self.sp += 1;
						}
					} else {
						bail!("Could not get the loop's index")
					}
				} else {
					bail!("Could not get the last loop stack pointer")
				}
			}
			Instr::Insert => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					if let Some(next_char) = self.input.next() {
						*value = next_char.into();
					} else {
						bail!("Could not get the next input char")
					}
				} else {
					bail!("Could not get the current value")
				}
			}
			Instr::Print => {
				if let Some(value) = self.stack.get(self.sp) {
					let value =
						char::from_u32(*value).ok_or_else(|| anyhow!("Could not parse to char"))?;
					self.writer.write_all(&[value as u8])?;
					self.writer.flush()?;
				} else {
					bail!("Could not get the current value")
				}
			}
			Instr::NoOp => {}
		}
		Ok(())
	}

	pub fn run(&mut self, program: &[Instr]) -> Result<u32> {
		self.ip = 0;
		self.sp = 0;
		self.loop_indexes = Vec::new();

		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr)?;
		}

		Ok(*self.stack.last().unwrap_or(&0))
	}
}
