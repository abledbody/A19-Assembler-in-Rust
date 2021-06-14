use logos::*;
use std::io::Write;
use termcolor::{self, Color, ColorSpec, StandardStream, WriteColor};
use crate::keywords::{Keyword, Register};


fn get_register(lex: &mut Lexer<Token>) -> Option<Register> {
	let slice =lex.slice()
		.to_uppercase();
	
	match slice.as_str() {
		"A"		=>	Some(Register::A),
		"B"		=>	Some(Register::B),
		"C"		=>	Some(Register::C),
		"T"		=>	Some(Register::T),
		"SP"	=>	Some(Register::SP),
		"VP"	=>	Some(Register::VP),
		"PP"	=>	Some(Register::PP),
		"FL"	=>	Some(Register::FL),
		_ => None,
	}
}

fn get_keyword(lex: &mut Lexer<Token>) -> Option<Keyword> {
	let slice =lex.slice()
		.to_uppercase();
	
	match slice.as_str() {
		"CONST"		=>	Some(Keyword::CONST),
		"MARK"		=>	Some(Keyword::MARK),
		"DATA"		=>	Some(Keyword::DATA),
		"DSTR"		=>	Some(Keyword::DSTR),
		
		"NEG"		=>	Some(Keyword::NEG),
		"ADD"		=>	Some(Keyword::ADD),
		"SUB"		=>	Some(Keyword::SUB),
		"MUL"		=>	Some(Keyword::MUL),
		"DIV"		=>	Some(Keyword::DIV),
		"MOD"		=>	Some(Keyword::MOD),
		"SMUL"		=>	Some(Keyword::SMUL),
		"SDIV"		=>	Some(Keyword::SDIV),
		"SMOD"		=>	Some(Keyword::SMOD),
		
		"NOT"		=>	Some(Keyword::NOT),
		"AND"		=>	Some(Keyword::AND),
		"OR"		=>	Some(Keyword::OR),
		"XOR"		=>	Some(Keyword::XOR),
		"SHL"		=>	Some(Keyword::SHL),
		"SHR"		=>	Some(Keyword::SHR),
		"SAR"		=>	Some(Keyword::SAR),
		
		"CMP"		=>	Some(Keyword::CMP),
		"JG"		=>	Some(Keyword::JG),
		"JNG"		=>	Some(Keyword::JNG),
		"JL"		=>	Some(Keyword::JL),
		"JNL"		=>	Some(Keyword::JNL),
		"JE"		=>	Some(Keyword::JE),
		"JNE"		=>	Some(Keyword::JNE),
		"JMP"		=>	Some(Keyword::JMP),
		
		"SET"		=>	Some(Keyword::SET),
		"GET"		=>	Some(Keyword::GET),
		"SWAP"		=>	Some(Keyword::SWAP),
		"PUSH"		=>	Some(Keyword::PUSH),
		"POP"		=>	Some(Keyword::POP),
		"CALL"		=>	Some(Keyword::CALL),
		"RET"		=>	Some(Keyword::RET),
		"VPUSH"		=>	Some(Keyword::VPUSH),
		"VPOP"		=>	Some(Keyword::VPOP),
		
		"NOP"		=>	Some(Keyword::NOP),
		"HALT"		=>	Some(Keyword::HALT),
		"EXTI"		=>	Some(Keyword::EXTI),
		_ => None,
	}
}

fn get_decimal_number (lex: &mut Lexer<Token>) -> Option<u16> {
	let string = lex.slice()
		.replace("_", "");
	
	let value = u16::from_str_radix(string.as_str(), 10);
	
	match value {
		Ok(value) => Some(value),
		Err(err) => panic!("{}", err),
	}
}

fn get_hexadecimal_number (lex: &mut Lexer<Token>) -> Option<u16> {
	let string = &lex.slice()[2..]
		.replace("_", "");
	
	let value = u16::from_str_radix(string.as_str(), 16);
	
	match value {
		Ok(value) => Some(value),
		Err(err) => panic!("{}", err),
	}
}

fn get_binary_number (lex: &mut Lexer<Token>) -> Option<u16> {
	let string = &lex.slice()[2..]
		.replace("_", "");
	
	let value = u16::from_str_radix(string.as_str(), 2);
	
	match value {
		Ok(value) => Some(value),
		Err(err) => panic!("{}", err),
	}
}

#[derive(Debug)]
#[derive(Logos)]
#[derive(Clone)]
pub enum Token {
	#[error]
	Error,
	
	#[regex("[ \t\n\r]+", logos::skip)]
	Whitespace,
	
	#[regex(";.*", logos::skip)]
	Comment,
	
	#[regex("(?i)((CONST)|(MARK)|(DATA)|(DSTR))", get_keyword)]
	#[regex("(?i)((NEG)|(ADD)|(SUB)|(MUL)|(DIV)|(MOD)|(SMUL)|(SDIV)|(SMOD))", get_keyword)]
	#[regex("(?i)((NOT)|(AND)|(OR)|(XOR)|(SHL)|(SHR)|(SAR))", get_keyword)]
	#[regex("(?i)((CMP)|(JG)|(JNG)|(JL)|(JNL)|(JE)|(JNE)|(JMP))", get_keyword)]
	#[regex("(?i)((SET)|(GET)|(SWAP)|(PUSH)|(POP)|(CALL)|(RET)|(VPUSH)|(VPOP))", get_keyword)]
	#[regex("(?i)((NOP)|(HALT)|(EXTI))", get_keyword)]
	Keyword(Keyword),
	
	#[regex("(?i)(A|B|C|T|SP|VP|PP|FL)", get_register, priority = 2)]
	Register(Register),
	
	#[regex(",", logos::skip)]
	Separator,
	
	#[regex("\\[")]
	OpenBracket,
	
	#[regex("\\]")]
	CloseBracket,
	
	#[regex("\"(?:[^\"]|\\\\\")*\"")]
	String,
	
	#[regex("([0-9][_0-9]*)", get_decimal_number)]
	#[regex("(?i)(0x[0-9A-F][_0-9A-F]*)", get_hexadecimal_number)]
	#[regex("(?i)(0b[0-1][_0-1]*)", get_binary_number)]
	Number(u16),
	
	#[regex("[\\+-]")]
	Operator,
	
	#[regex("(?i)([_A-Z][_A-Z0-9]*\\.?)+", priority = 1)]
	Identifier,
}

pub fn print_all(stdout: &mut StandardStream, data: &str, hold: bool) {
	let mut lex = Token::lexer(&data);
	
	'lexing: loop {
		for _ in 0..30 {
			let token = match lex.next() {
				Some(token) => token,
				None => {
					if hold {std::io::stdin().read_line(&mut "".to_owned()).unwrap();}
					break 'lexing;
				}
			};
				
			let col = match token {
				Token::Error		=> Color::Red,
				Token::Whitespace	=> Color::Ansi256(8),
				Token::Comment		=> Color::Green,
				Token::Keyword(_)	=> Color::Blue,
				
				Token::Register(_)	=> Color::Cyan,
				Token::Separator	=> Color::Yellow,
				Token::OpenBracket	=> Color::Ansi256(166),
				Token::CloseBracket	=> Color::Ansi256(166),
				
				Token::String		=> Color::Yellow,
				Token::Number(_)	=> Color::Ansi256(105),
				Token::Operator		=> Color::Ansi256(127),
				Token::Identifier	=> Color::White,
			};
			stdout.set_color(ColorSpec::new().set_fg(Some(col))).unwrap();
			
			write!(stdout, "{}\t", token_display(&lex, &token)).unwrap();
		}
		if hold {std::io::stdin().read_line(&mut "".to_owned()).unwrap();}
	}
	stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
}

pub fn token_display(lex: &Lexer<Token>, token: &Token) -> String {
	let string = match token {
		Token::Keyword(_) => "\n",
		Token::Error => panic!(" <There's an invalid token here!> "),
		_ => ""
	};
	format!("{}{}", string, lex.slice())
}