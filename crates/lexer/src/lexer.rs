use super::tokens::Token;

pub fn lex(input: &str) -> Box<impl Iterator<Item = Token> + '_> {
	let token_iter = input.as_bytes().iter().map(Token::from);
	Box::new(token_iter)
}
