#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	/// `>`
	PointerRight,
	/// `<`
	PointerLeft,
	/// `+`
	CellInc,
	/// `-`
	CellDec,
	/// `,`
	Insert,
	/// `.`
	Print,
	/// `[`
	LoopStart,
	/// `]`
	LoopEnd,
	/// Any other character is considered a comment
	Comment(u8),
}

impl From<&u8> for Token {
	fn from(c: &u8) -> Self {
		match *c {
			b'>' => Token::PointerRight,
			b'<' => Token::PointerLeft,
			b'+' => Token::CellInc,
			b'-' => Token::CellDec,
			b',' => Token::Insert,
			b'.' => Token::Print,
			b'[' => Token::LoopStart,
			b']' => Token::LoopEnd,
			other => Token::Comment(other),
		}
	}
}
