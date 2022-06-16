use clap::{self, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Argv {
	/// The program's input
	#[clap(value_parser)]
	input: Option<String>,
	/// The program's source code file
	#[clap(short, long, value_parser, value_name = "FILE")]
	file: Option<PathBuf>,
	/// The program's source code as a string
	#[clap(short, long, value_parser, value_name = "EVAL")]
	eval: Option<String>,
}

fn main() {
	let argv = Argv::parse();

	println!("input: {:?}", argv.input);
}
