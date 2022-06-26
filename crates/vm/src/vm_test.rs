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
	vm.run(&[Inc])?;

	assert_eq!(vm.stack, &[1]);
	Ok(())
}

#[test_with_logger]
pub fn sanity_2() -> Result<()> {
	use Instr::*;

	let mut vm = VM::default();
	vm.run(&[Inc, Inc, Inc, MoveRight, MoveRight, Inc, Inc, Inc])?;

	assert_eq!(vm.stack, &[3, 0, 3]);
	Ok(())
}

#[test_with_logger]
pub fn sanity_print() -> Result<()> {
	use Instr::*;

	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new(Box::new("".as_bytes()), f_out);
	vm.run(&[
		Inc, Inc, Inc, MoveRight, MoveRight, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
		Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
		Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
		Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
		Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc,
		Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Inc, Print,
	])?;

	assert_eq!(vm.stack, &[3, 0, 97]);

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "a");

	Ok(())
}

#[test_with_logger]
pub fn sanity_input() -> Result<()> {
	use Instr::*;

	let tmp_out_file = temp_dir().join(TEST_OUTPUT_FILE);
	let f_out = File::create(&tmp_out_file)?;
	let f_out = Box::new(f_out);

	let mut vm = VM::new(Box::new("a".as_bytes()), f_out);
	vm.run(&[Insert, Print])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "a");

	Ok(())
}

#[test_with_logger]
pub fn move_value() -> Result<()> {
	use Instr::*;

	let mut vm = VM::default();

	vm.stack = vec![5, 0, 0];
	vm.run(&[
		LoopStart, Dec, MoveRight, MoveRight, Inc, MoveLeft, MoveLeft, LoopEnd,
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

	let mut vm = VM::new(Box::new("abcdefg".as_bytes()), f_out);
	vm.run(&[Insert, LoopStart, Print, Insert, LoopEnd])?;

	let tmp_file_content = fs::read_to_string(&tmp_out_file)?;
	assert_eq!(tmp_file_content, "abcdefg");

	Ok(())
}
