use super::tokens::Token;

pub fn lex(input: &'static str) -> Box<impl Iterator<Item = Token>> {
	let token_iter = input.as_bytes().iter().map(Token::from);
	Box::new(token_iter)
}
