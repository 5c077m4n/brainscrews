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
