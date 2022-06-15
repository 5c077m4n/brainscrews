use super::{instr::Instr, vm::VM};
use anyhow::Result;
use log::debug;
use macros::test_with_logger;

#[test_with_logger]
pub fn sanity() -> Result<()> {
	let program = &[Instr::Inc(1)];

	let mut vm = VM::default();
	let result = vm.run(program)?;
	debug!("Stack: {:?}", vm.stack);

	assert_eq!(result, 1);
	Ok(())
}
