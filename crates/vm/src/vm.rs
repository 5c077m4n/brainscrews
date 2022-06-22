use super::instr::Instr;
use anyhow::{anyhow, bail, Result};
use log::debug;
use std::{
	io::{self, Write},
	str::Chars,
};

const MEMORY_LENGTH_LIMIT: usize = 30_000;

pub struct VM<'v> {
	pub(crate) stack: Vec<u8>,

	/// Instruction pointer
	ip: usize,
	/// Stack pointer
	sp: usize,
	loop_indexes: Vec<usize>,

	input: Option<Chars<'v>>,
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
			input: None,
			writer: Box::new(io::stdout()),
		}
	}
}

impl VM<'_> {
	pub fn new(input: Option<&'static str>, writer: Box<dyn Write>) -> Self {
		Self {
			input: input.map(|i| i.chars()),
			writer,
			..Self::default()
		}
	}
	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		debug!("Instruction: {:?}, current stack: {:?}", &instr, self.stack);
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
					*value = value.wrapping_add(*n);
				} else {
					bail!("Index not found")
				}
			}
			Instr::Dec(n) => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					*value = value.wrapping_sub(*n);
				} else {
					bail!("Index not found")
				}
			}
			Instr::LoopStart => {
				self.loop_indexes.push(self.ip);
			}
			Instr::LoopEnd => {
				if let Some(latest_loop_ip) = self.loop_indexes.last() {
					if let Some(latest_loop_counter) = self.stack.get_mut(*latest_loop_ip - 1) {
						if *latest_loop_counter == 0 {
							let _ = self.loop_indexes.pop();
						} else {
							self.ip = *latest_loop_ip;
							*latest_loop_counter -= 1;
						}
					} else {
						bail!("Could not get the loop's index")
					}
				} else {
					bail!("Could not get the last loop stack pointer")
				}
			}
			Instr::Insert => {
				let input_iter = self.input.as_mut().ok_or_else(|| {
					anyhow!("If you want to insert (`,`) then you need to provide an input")
				})?;

				if let Some(value) = self.stack.get_mut(self.sp) {
					if let Some(next_char) = input_iter.next() {
						*value = next_char as u8;
					}
				} else {
					bail!("Could not get the current value")
				}
			}
			Instr::Print => {
				if let Some(&value) = self.stack.get(self.sp) {
					let value = &[value];
					self.writer.write_all(value)?;
					self.writer.flush()?;
				} else {
					bail!("Could not get the current value")
				}
			}
			Instr::NoOp => {}
		}
		Ok(())
	}

	pub fn run(&mut self, program: &[Instr]) -> Result<()> {
		self.ip = 0;
		self.sp = 0;
		self.loop_indexes = Vec::new();

		while let Some(instr) = program.get(self.ip) {
			self.handle_instr(instr)?;
			self.ip += 1;
		}

		Ok(())
	}
}
