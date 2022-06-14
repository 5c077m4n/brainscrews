use super::{instr::Instr, vm::VM};
use anyhow::Result;
use macros::test_with_logger;

#[ignore]
#[test_with_logger]
pub fn sanity() -> Result<()> {
	let program = &[Instr::Inc(1), Instr::Debug];

	let mut vm = VM::default();
	let result = vm.run(program)?;

	assert_eq!(result, 1);
	Ok(())
}
