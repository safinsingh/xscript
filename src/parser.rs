use crate::Error;
use anyhow::*;
use std::str::SplitWhitespace;

#[derive(Debug)]
pub(crate) enum Token {
	Register(u8),
	Increment,
	Decrement,
	Zero,
	Loop,
	Out,
	Number(u8),
}

pub(crate) struct Parser<'a> {
	input: SplitWhitespace<'a>,
}

impl<'a> Parser<'a> {
	pub(crate) fn new(input: &'a str) -> Parser<'a> {
		Self {
			input: input.split_whitespace(),
		}
	}

	pub(crate) fn parse(&mut self) -> Result<Vec<Token>> {
		let mut tokens = Vec::new();

		while let Some(word) = self.input.next() {
			match word {
				"@bruh" => tokens.push(Token::Register(1)),
				"@bruuh" => tokens.push(Token::Register(2)),
				"@bruuuh" => tokens.push(Token::Register(3)),
				"@bruuuuh" => tokens.push(Token::Register(4)),
				"yes" => tokens.push(Token::Increment),
				"no" => tokens.push(Token::Decrement),
				"die" => tokens.push(Token::Zero),
				"ok" => tokens.push(Token::Loop),
				"lol" => tokens.push(Token::Out),
				_ => {
					if let Ok(number) = word.parse::<u8>() {
						tokens.push(Token::Number(number))
					} else {
						return Err(Error::UnidentifiedWord(word.into()).into());
					}
				}
			}
		}
		Ok(tokens)
	}
}
