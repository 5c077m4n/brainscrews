use super::{instr::Instr, vm::VM};
use anyhow::Result;
use macros::test_with_logger;

#[test_with_logger]
pub fn sanity() -> Result<()> {
	let mut vm = VM::default();
	let result = vm.run(&[Instr::Inc(1)])?;

	assert_eq!(result, 1);
	assert_eq!(vm.stack, &[1]);
	Ok(())
}

#[test_with_logger]
pub fn sanity_2() -> Result<()> {
	let mut vm = VM::default();
	let result = vm.run(&[
		Instr::Inc(1),
		Instr::Inc(1),
		Instr::Inc(1),
		Instr::MoveRight(1),
		Instr::MoveRight(1),
		Instr::Inc(1),
		Instr::Inc(1),
		Instr::Inc(1),
	])?;

	assert_eq!(result, 3);
	assert_eq!(vm.stack, &[3, 0, 3]);
	Ok(())
}
