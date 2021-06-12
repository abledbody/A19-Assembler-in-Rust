use termcolor::{StandardStream, ColorChoice};

mod lexer;
mod keywords;


fn main() {
	//let args: Vec<String> = std::env::args().collect();
	//let path = args[1].to_owned();
	let path = "debug.a19";
	let data = std::fs::read_to_string(path).unwrap();
	//let data = "Something about \"Magic\" But with \"Escape characters, like \\\"this\\\"\"";
	
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	
	lexer::token_display(&mut stdout, &data);
}