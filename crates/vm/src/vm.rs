use super::instr::Instr;

pub struct VM {
	pub ip: usize,
	pub stack: Vec<isize>,

	pub memory_pointer: usize,
	pub memory: Vec<isize>,
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack: Vec::new(),
			memory_pointer: 0,
			memory: Vec::with_capacity(30_000),
		}
	}
}

impl VM {
	pub fn handle_instr(&mut self, instr: &Instr) {
		match instr {
			Instr::MoveRight(n) => self.memory_pointer += n,
			Instr::MoveLeft(n) => self.memory_pointer -= n,
			other => todo!("Handle instr {:?}", other),
		}
	}

	pub fn run(&mut self, program: &[Instr]) -> Option<isize> {
		self.ip = 0;
		while let Some(instr) = program.get(self.ip) {
			self.ip += 1;
			self.handle_instr(instr);
		}

		self.stack.pop()
	}
}
