use super::tokens::Token;

pub fn lex(input: &'static str) -> Box<impl Iterator<Item = Token>> {
	let token_iter = input.as_bytes().iter().map(|c| match c {
		b'>' => Token::PointerToRight(1),
		b'<' => Token::PointerToLeft(1),
		b'+' => Token::CellInc(1),
		b'-' => Token::CellDec(1),
		b',' => Token::CellInsert(1),
		b'.' => Token::CellPrint,
		b'[' => Token::LoopStart,
		b']' => Token::LoopEnd,
		other => Token::Comment(*other),
	});
	Box::new(token_iter)
}
