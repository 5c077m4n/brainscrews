use super::instr::Instr;

pub struct VM {
	pub ip: usize,
	pub stack: Vec<Instr>,
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
	pub fn handle_instr(&self, _cmd: &Instr) {}
}
