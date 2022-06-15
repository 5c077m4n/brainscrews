use super::instr::Instr;
use anyhow::{anyhow, bail, Result};

const MEMORY_LENGTH_LIMIT: usize = 30_000;

pub struct VM {
	pub(crate) ip: usize,
	pub(crate) stack_pointer: usize,
	pub(crate) stack: Vec<isize>,
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack_pointer: 0,
			stack: {
				let mut vec = Vec::with_capacity(MEMORY_LENGTH_LIMIT);
				vec.push(0);
				vec
			},
		}
	}
}

impl VM {
	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::MoveRight(n) => {
				if let Some(result) = self.stack_pointer.checked_add(*n) {
					self.stack_pointer = result;

					if self.stack.len() < self.stack_pointer {
						for _ in self.stack.len()..=self.stack_pointer {
							self.stack.push(0);
						}
					}
				} else {
					bail!("Sorry, the index is too large")
				}
			}
			Instr::MoveLeft(n) => {
				if let Some(result) = self.stack_pointer.checked_sub(*n) {
					self.stack_pointer = result;

					if self.stack.len() < self.stack_pointer {
						for _ in self.stack.len()..=self.stack_pointer {
							self.stack.push(0);
						}
					}
				} else {
					bail!("Sorry, the index is too small")
				}
			}
			Instr::Inc(n) => {
				if let Some(value) = self.stack.get_mut(self.stack_pointer) {
					*value += n;
				} else {
					bail!("Index not found")
				}
			}
			Instr::Dec(n) => {
				if let Some(value) = self.stack.get_mut(self.stack_pointer) {
					*value -= n;
				} else {
					bail!("Index not found")
				}
			}
			Instr::Print => {
				if let Some(value) = self.stack.get(self.stack_pointer) {
					print!("{}", value);
				} else {
					bail!("Could not get the current value")
				}
			}
			Instr::NoOp => {}
			other => todo!("Handle instr {:?}", other),
		}
		Ok(())
	}

	pub fn run(&mut self, program: &[Instr]) -> Result<isize> {
		self.ip = 0;
		self.stack_pointer = 0;

		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr)?;
		}

		self.stack
			.pop()
			.ok_or_else(|| anyhow!("The stack is empty"))
	}
}
