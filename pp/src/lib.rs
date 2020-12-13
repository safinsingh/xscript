use anyhow::Result;
use std::str::SplitWhitespace;
use xs_core::{parser::Token, Error};

pub struct Parser<'a> {
	input: SplitWhitespace<'a>,
}

impl<'a> Parser<'a> {
	pub fn new(input: &'a str) -> Parser<'a> {
		Self {
			input: input.split_whitespace(),
		}
	}

	pub fn parse(&mut self) -> Result<Vec<Token>> {
		let mut tokens = Vec::new();

		while let Some(word) = self.input.next() {
			match word {
				"p" => tokens.push(Token::Register(1)),
				"pp" => tokens.push(Token::Register(2)),
				"ppp" => tokens.push(Token::Register(3)),
				"pppp" => tokens.push(Token::Register(4)),
				"ppppp" => tokens.push(Token::Increment),
				"pppppp" => tokens.push(Token::Decrement),
				"ppppppp" => tokens.push(Token::Zero),
				"pppppppp" => tokens.push(Token::Loop),
				"ppppppppp" => tokens.push(Token::Out),
				_ => {
					if word.starts_with('!') {
						let binary_str =
							&word.replace("p", "0").replace("P", "1")[1..];

						if let Ok(number) = u8::from_str_radix(binary_str, 2) {
							tokens.push(Token::Number(number))
						}
					} else {
						return Err(Error::UnidentifiedWord(word.into()).into());
					}
				}
			}
		}
		Ok(tokens)
	}
}
