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
	use Instr::*;

	let mut vm = VM::default();
	vm.run(&[Inc(1)])?;

	assert_eq!(vm.stack, &[1]);
	Ok(())
}

#[test_with_logger]
pub fn sanity_2() -> Result<()> {
	use Instr::*;

	let mut vm = VM::default();
	vm.run(&[
		Inc(1),
		Inc(1),
		Inc(1),
		MoveRight(1),
		MoveRight(1),
		Inc(1),
		Inc(1),
		Inc(1),
	])?;

	assert_eq!(vm.stack, &[3, 0, 3]);
	Ok(())
}

#[test_with_logger]
pub fn sanity_print() -> Result<()> {
	use Instr::*;

	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new(None, f_out);
	vm.run(&[
		Inc(1),
		Inc(1),
		Inc(1),
		MoveRight(1),
		MoveRight(1),
		Inc(b'a'),
		Print,
	])?;

	assert_eq!(vm.stack, &[3, 0, b'a']);

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content.as_bytes(), b"a");

	Ok(())
}

#[test_with_logger]
pub fn sanity_input() -> Result<()> {
	use Instr::*;

	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new(Some("a"), f_out);
	vm.run(&[Insert, Print])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "a");

	Ok(())
}

#[test_with_logger]
#[ignore]
pub fn move_value() -> Result<()> {
	use Instr::*;

	let mut vm = VM::default();
	vm.run(&[
		Inc(5),
		MoveRight(2),
		MoveLeft(2),
		LoopStart,
		Dec(1),
		MoveRight(2),
		Inc(1),
		MoveLeft(2),
		LoopEnd,
	])?;

	assert_eq!(vm.stack, &[0, 0, 5]);

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
	vm.run(&[Insert, LoopStart, Print, Insert, LoopEnd])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "abcdefg");

	Ok(())
}
