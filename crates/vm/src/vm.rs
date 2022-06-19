use super::instr::Instr;
use anyhow::{anyhow, bail, Result};
use std::io::{self, Write};

const MEMORY_LENGTH_LIMIT: usize = 30_000;

pub struct VM {
	pub(crate) stack: Vec<u32>,

	/// Instruction pointer
	ip: usize,
	/// Stack pointer
	sp: usize,
	writer: Box<dyn Write>,
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			sp: 0,
			stack: {
				let mut vec = Vec::with_capacity(MEMORY_LENGTH_LIMIT);
				vec.push(0);
				vec
			},
			writer: Box::new(io::stdout()),
		}
	}
}

impl VM {
	pub fn new(writer: Box<dyn Write>) -> Self {
		Self {
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
			other => todo!("Handle instr {:?}", other),
		}
		Ok(())
	}

	pub fn run(&mut self, program: &[Instr]) -> Result<u32> {
		self.ip = 0;
		self.sp = 0;

		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr)?;
		}

		Ok(*self.stack.last().unwrap_or(&0))
	}

	pub fn dump(&mut self) -> Result<(), io::Error> {
		self.writer
			.write_all(format!("{:?}", self.stack).as_bytes())?;
		self.writer.flush()
	}
}
