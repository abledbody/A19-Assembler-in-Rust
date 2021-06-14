
use std::{
	fs,
	path::Path,
	collections::HashMap
};
use termcolor::{StandardStream, ColorChoice};
use logos::Logos;
use lexer::Token;

mod lexer;
mod keywords;
mod parser;
mod encoder;


fn main() {
	let args: Vec<String> = std::env::args().collect();
	let path = Path::new(&args[1]);
	//let path = "debug.a19";
	let data = std::fs::read_to_string(path).unwrap();
	
	let mut stdout = StandardStream::stdout(ColorChoice::Always);
	
	lexer::print_all(&mut stdout, &data, false);
	
	let mut lex = Token::lexer(&data);
	
	let mut partially_encoded_file: Vec<encoder::Byte> = vec!();
	let mut constants: HashMap<String, u16> = HashMap::new();
	
	loop {
		match parser::parse(&mut lex) {
			Some(instruction) => {
				for byte in encoder::partially_encode(&instruction, &mut constants, partially_encoded_file.len() as u16).into_iter() {
					print!("{:?}\t", byte);
					partially_encoded_file.push(byte);
				}
				println!("{:?}", instruction);
			},
			None => break
		};
	}
	
	let all_bytes = encoder::encode_identifiers(&constants, &partially_encoded_file);
	
	let mut encoded_file = vec!();
	
	for value in all_bytes.into_iter() {
		encoded_file.push((value >> 8) as u8);
		encoded_file.push(value as u8)
	};
	
	let output_path = format!("{}\\{}.bin", path.parent().unwrap().to_str().unwrap(), path.file_stem().unwrap().to_str().unwrap());
	
	fs::write(output_path, &encoded_file).unwrap();
}