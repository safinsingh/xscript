mod errors;
mod parser;
mod vm;

use anyhow::*;
use errors::Error;
use parser::Parser;
use std::{env, fs};
use vm::Vm;

fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect();
	let file = args.get(1).ok_or(Error::NoInputFile)?;

	let program =
		fs::read_to_string(file).map_err(|_| Error::InputReadFailed)?;

	let tokens = Parser::new(&program).parse()?;
	let mut registers = [0, 0, 0, 0];

	Vm::new(tokens, &mut registers).run()?;

	Ok(())
}
