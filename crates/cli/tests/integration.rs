use anyhow::Result;
use assert_cmd::Command;

const BIN_NAME: &str = env!("CARGO_PKG_NAME");

#[test]
#[ignore]
fn hello_world() -> Result<()> {
	let mut cmd = Command::cargo_bin(BIN_NAME)?;

	cmd.args(&["--eval", "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."])
		.assert()
		.success();
	Ok(())
}
