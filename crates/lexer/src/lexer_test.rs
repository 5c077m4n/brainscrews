use super::{lexer::lex, tokens::Token};

#[test]
fn sanity() {
	use Token::*;

	let result: Vec<Token> = lex(">>>").collect();
	let expected = vec![PointerRight, PointerRight, PointerRight];

	assert_eq!(result, expected);
}

#[test]
fn comments() {
	use Token::*;

	let result: Vec<Token> = lex(">>> comment").collect();
	let expected = vec![
		PointerRight,
		PointerRight,
		PointerRight,
		Comment(b' '),
		Comment(b'c'),
		Comment(b'o'),
		Comment(b'm'),
		Comment(b'm'),
		Comment(b'e'),
		Comment(b'n'),
		Comment(b't'),
	];

	assert_eq!(result, expected);
}
