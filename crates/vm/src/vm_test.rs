use super::{instr::Instr, vm::VM};
use anyhow::Result;
use macros::test_with_logger;
use std::{
	env::temp_dir,
	fs::{self, File},
};

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

#[test_with_logger]
pub fn sanity_print() -> Result<()> {
	let tmp_file = temp_dir().join("brainscrews-test");

	let f = File::create(&tmp_file)?;
	let f = Box::new(f);

	let mut vm = VM::new(f);
	let result = vm.run(&[
		Instr::Inc(1),
		Instr::Inc(1),
		Instr::Inc(1),
		Instr::MoveRight(1),
		Instr::MoveRight(1),
		Instr::Inc(b'a'.into()),
		Instr::Print,
	])?;

	assert_eq!(result, b'a'.into());
	assert_eq!(vm.stack, &[3, 0, b'a'.into()]);

	let tmp_file_content = fs::read_to_string(&tmp_file)?;
	assert_eq!(tmp_file_content.as_bytes(), b"a");

	Ok(())
}
