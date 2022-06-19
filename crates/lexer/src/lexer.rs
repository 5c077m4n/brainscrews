use super::tokens::Token;

pub fn lex(input: &'static str) -> Box<impl Iterator<Item = Token>> {
	let token_iter = input.as_bytes().iter().map(|c| match c {
		b'>' => Token::PointerRight,
		b'<' => Token::PointerLeft,
		b'+' => Token::CellInc,
		b'-' => Token::CellDec,
		b',' => Token::Insert,
		b'.' => Token::Print,
		b'[' => Token::LoopStart,
		b']' => Token::LoopEnd,
		other => Token::Comment(*other),
	});
	Box::new(token_iter)
}
