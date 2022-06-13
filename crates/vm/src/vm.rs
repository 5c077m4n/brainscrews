use super::instr::Instr;

pub struct VM {
	pub ip: usize,
	pub stack: Vec<isize>,
}

impl Default for VM {
	fn default() -> Self {
		Self {
			ip: 0,
			stack: Vec::with_capacity(30_000),
		}
	}
}

impl VM {
	pub fn handle_instr(&self, instr: &Instr) {
		todo!("Handle {:?} instruction", instr)
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
