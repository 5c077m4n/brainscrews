use lexer::tokens::Token;

#[derive(Debug)]
pub enum Instr {
	MoveRight(usize),
	MoveLeft(usize),
	Inc(usize),
	Dec(usize),
	Insert,
	Print,
	LoopStart,
	LoopEnd,
	Noop,
}

impl From<Token> for Instr {
	fn from(token: Token) -> Self {
		match token {
			Token::PointerRight => Self::MoveRight(1),
			Token::PointerLeft => Self::MoveLeft(1),
			Token::CellInc => Self::Inc(1),
			Token::CellDec => Self::Dec(1),
			Token::CellInsert => Self::Insert,
			Token::CellPrint => Self::Print,
			Token::LoopStart => Self::LoopStart,
			Token::LoopEnd => Self::LoopEnd,
			Token::Comment(_) => Self::Noop,
		}
	}
}
