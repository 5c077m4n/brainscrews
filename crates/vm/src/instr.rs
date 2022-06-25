use lexer::tokens::Token;

#[derive(Debug)]
pub enum Instr {
	MoveRight,
	MoveLeft,
	Inc,
	Dec,
	Insert,
	Print,
	LoopStart,
	LoopEnd,
	NoOp,
}

impl From<Token> for Instr {
	fn from(token: Token) -> Self {
		match token {
			Token::PointerRight => Self::MoveRight,
			Token::PointerLeft => Self::MoveLeft,
			Token::CellInc => Self::Inc,
			Token::CellDec => Self::Dec,
			Token::Insert => Self::Insert,
			Token::Print => Self::Print,
			Token::LoopStart => Self::LoopStart,
			Token::LoopEnd => Self::LoopEnd,
			Token::Comment(_) => Self::NoOp,
		}
	}
}
