use anyhow::Result;
use clap::Clap;
use std::{fs, str::FromStr};
use xs_core::{parser::Token, Error, Vm};

enum Parser {
	Xs,
	Pp,
}

impl FromStr for Parser {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pp" => Ok(Self::Pp),
			"xsc" => Ok(Self::Xs),
			_ => Err(Error::InvalidParser.into()),
		}
	}
}

impl Parser {
	fn parse(&self, input: &str) -> Result<Vec<Token>> {
		Ok(match *self {
			Self::Xs => xs_xsc::Parser::new(input).parse()?,
			Self::Pp => xs_pp::Parser::new(input).parse()?,
		})
	}
}

#[derive(Clap)]
#[clap(version = "1.0", author = "Safin S. <safinsingh.dev@gmail.com>")]
struct Opts {
	#[clap(short, long)]
	/// Parser to use to interpret your xscript code [xsc, pp]
	parser: Parser,

	#[clap(index = 1)]
	/// File to interpret
	file: String,
}

fn main() -> Result<()> {
	let opts: Opts = Opts::parse();

	let program =
		fs::read_to_string(opts.file).map_err(|_| Error::InputReadFailed)?;

	let tokens = opts.parser.parse(&program)?;
	let mut registers = [0, 0, 0, 0];

	Vm::new(tokens, &mut registers).run()
}
