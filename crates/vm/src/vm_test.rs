use super::{instr::Instr, vm::VM};
use anyhow::Result;
use macros::test_with_logger;
use std::{
	env::temp_dir,
	fs::{self, File},
};

const TEST_OUTPUT_FILE: &str = "brainscrews_test_output_file";

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
	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new("", f_out);
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

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content.as_bytes(), b"a");

	Ok(())
}

#[test_with_logger]
pub fn sanity_input() -> Result<()> {
	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new("a", f_out);
	let _ = vm.run(&[Instr::Insert, Instr::Print])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "a");

	Ok(())
}

#[test_with_logger]
#[ignore]
pub fn loop_zero_param() -> Result<()> {
	use Instr::*;

	let mut vm = VM::default();
	let result = vm.run(&[Inc(10), LoopStart, Dec(1), LoopEnd])?;

	assert_eq!(result, 0);

	Ok(())
}

#[test_with_logger]
#[ignore]
pub fn loop_cat() -> Result<()> {
	use Instr::*;

	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new(Some("abcdefg"), f_out);
	let _ = vm.run(&[Insert, LoopStart, Print, Insert, LoopEnd])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "abcdefg");

	Ok(())
}
