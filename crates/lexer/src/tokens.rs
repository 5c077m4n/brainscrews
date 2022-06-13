pub enum Token {
	/// `>`
	PointerToRight(usize),
	/// `<`
	PointerToLeft(usize),
	/// `+`
	CellInc(usize),
	/// `-`
	CellDec(usize),
	/// `,`
	CellInsert(usize),
	/// `.`
	CellPrint,
	/// `[`
	LoopStart,
	/// `]`
	LoopEnd,
	/// Any other character is considered a comment
	Comment(u8),
}
