#[derive(Debug)]
pub enum Token {
	Register(u8),
	Increment,
	Decrement,
	Zero,
	Loop,
	Out,
	Number(u8),
}
