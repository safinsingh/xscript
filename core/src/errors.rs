use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
	#[error("Unidentified word: `{0}`")]
	UnidentifiedWord(String),

	#[error("Attempted to apply operation on invalid register")]
	OpOnInvalidRegister,

	#[error("Attempted to read token at undefined index")]
	UndefinedIndex,

	#[error("Input file not specified in argv[1]")]
	NoInputFile,

	#[error("Failed to read file specified in argv[1]")]
	InputReadFailed,

	#[error("Invalid parser. Run with --help for more options")]
	InvalidParser,
}
