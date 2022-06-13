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
	CellInsert,
	/// `.`
	CellPrint,
	/// `[`
	LoopStart,
	/// `]`
	LoopEnd,
	/// Any other character is considered a comment
	Comment(u8),
}
