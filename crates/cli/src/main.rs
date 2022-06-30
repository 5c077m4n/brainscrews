use anyhow::Result;
use clap::{self, Parser};
use lexer::lexer::lex;
use std::{fs, path::PathBuf};
use vm::vm::VM;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Argv {
	/// The program's source code file
	#[clap(short, long, value_parser, value_name = "FILE")]
	file: Option<PathBuf>,
	/// The program's source code as a string
	#[clap(short, long, value_parser, value_name = "EVAL")]
	eval: Option<String>,
}

fn main() -> Result<()> {
	let argv = Argv::parse();

	if let Some(code) = argv.eval {
		let tokens = lex(&code);
		let tokens: Vec<_> = tokens.map(|t| t.into()).collect();

		let mut vm = VM::default();
		vm.run(&tokens)?;
	} else if let Some(file_path) = argv.file {
		let code = fs::read_to_string(file_path)?;
		let tokens = lex(&code);
		let tokens: Vec<_> = tokens.map(|t| t.into()).collect();

		let mut vm = VM::default();
		vm.run(&tokens)?;
	}

	Ok(())
}
