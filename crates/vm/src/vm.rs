use super::instr::Instr;
use anyhow::{anyhow, bail, Result};
use log::debug;
use std::io::{self, Read, Write};

const MEMORY_LENGTH_LIMIT: usize = 30_000;

pub struct VM {
	pub(crate) stack: Vec<u8>,

	/// Instruction pointer
	ip: usize,
	/// Stack pointer
	sp: usize,
	loop_indexes: Vec<(usize, u8)>,

	reader: Box<dyn Read>,
	writer: Box<dyn Write>,
}

impl Default for VM {
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
			reader: Box::new(io::stdin()),
			writer: Box::new(io::stdout()),
		}
	}
}

impl VM {
	pub fn new(reader: Box<dyn Read>, writer: Box<dyn Write>) -> Self {
		Self {
			reader,
			writer,
			..Self::default()
		}
	}
	fn handle_instr(&mut self, instr: &Instr) -> Result<()> {
		match instr {
			Instr::MoveRight => {
				if let Some(result) = self.sp.checked_add(1) {
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
			Instr::MoveLeft => {
				if let Some(result) = self.sp.checked_sub(1) {
					self.sp = result;
				} else {
					bail!("Sorry, the index is too small")
				}
			}
			Instr::Inc => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					*value = value.saturating_add(1);
				} else {
					bail!("Index not found")
				}
			}
			Instr::Dec => {
				if let Some(value) = self.stack.get_mut(self.sp) {
					*value = value.saturating_sub(1);
				} else {
					bail!("Index not found")
				}
			}
			Instr::LoopStart => {
				let loop_counter = self.stack.get(self.sp).unwrap();
				self.loop_indexes.push((self.ip, *loop_counter - 1));
			}
			Instr::LoopEnd => {
				let (loop_start_ip, loop_counter) = self
					.loop_indexes
					.last_mut()
					.expect("You need to start the loop before ending it...");
				if *loop_counter == 0 {
					let _ = self.loop_indexes.pop();
				} else {
					self.ip = *loop_start_ip;
					*loop_counter -= 1;
				}
			}
			Instr::Insert => {
				let mut buffer: [u8; 1] = [0];
				self.reader
					.read_exact(&mut buffer)
					.map_err(|error| anyhow!("Could not read the given input ({:?})", &error))?;
				debug!("Input: {:?}/`{}`", &buffer, std::str::from_utf8(&buffer)?);

				if let Some(value) = self.stack.get_mut(self.sp) {
					*value = buffer[0];
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
			debug!(
				"Instruction: {:?}, ip: {:?}, stack: {:?}, loop indexes: {:?}",
				&instr, &self.ip, &self.stack, &self.loop_indexes,
			);
			self.ip += 1;
		}

		Ok(())
	}
}
