use crate::{parser::Token, Error};
use anyhow::Result;

#[derive(Debug)]
pub(crate) struct Vm<'a> {
	tokens: Vec<Token>,
	idx: u8,
	registers: &'a mut [u8; 4],
	pointer: u8,
}

impl<'a> Vm<'a> {
	pub(crate) fn new(
		tokens: Vec<Token>,
		registers: &'a mut [u8; 4],
	) -> Vm<'a> {
		Self {
			tokens,
			idx: 0,
			registers,
			pointer: 0,
		}
	}

	fn exec(&mut self) -> Result<()> {
		match self
			.tokens
			.get(self.idx as usize)
			.ok_or(Error::UndefinedIndex)?
		{
			Token::Register(r) => self.pointer = *r,
			Token::Increment => self.op(|r| *r += 1)?,
			Token::Decrement => self.op(|r| *r -= 1)?,
			Token::Zero => self.op(|r| *r = 0)?,
			Token::Loop => {
				if let Some(Token::Number(reps)) =
					self.tokens.get(self.idx as usize + 1)
				{
					if self.tokens.get(self.idx as usize + 2).is_some() {
						self.idx += 2;
						for _ in 0..*reps {
							self.exec()?;
						}
					}
				}
			}
			Token::Out => self.op(|r| print!("{}", char::from(*r)))?,
			Token::Number(_) => {}
		}

		Ok(())
	}

	fn op<F>(&mut self, mut fun: F) -> Result<()>
	where
		F: FnMut(&mut u8),
	{
		match self.pointer {
			1 => fun(&mut self.registers[0]),
			2 => fun(&mut self.registers[1]),
			3 => fun(&mut self.registers[2]),
			4 => fun(&mut self.registers[3]),
			_ => return Err(Error::OpOnInvalidRegister.into()),
		}
		Ok(())
	}

	pub(crate) fn run(&mut self) -> Result<()> {
		while self.tokens.get(self.idx as usize).is_some() {
			self.exec()?;
			self.idx += 1;
		}

		Ok(())
	}
}
