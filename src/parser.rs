use logos::Lexer;
use crate::keywords::{Register, Keyword};
use crate::lexer::*;

#[derive(Debug)]
pub enum Instruction {
	CONST(String, u16),
	MARK(String),
	DATA(Vec<u16>),
	DSTR(Vec<u16>),
	
	ADD(Target, Target),
	SUB(Target, Target),
	MUL(Target, Target),
	SMUL(Target, Target),
	DIV(Target, Target),
	SDIV(Target, Target),
	MOD(Target, Target),
	SMOD(Target, Target),
	NEG(Target),
	
	NOT(Target),
	AND(Target, Target),
	OR(Target, Target),
	XOR(Target, Target),
	SHL(Target, Target),
	SHR(Target, Target),
	SAR(Target, Target),
	
	CMP(Target, Target),
	JG(Target),
	JNG(Target),
	JL(Target),
	JNL(Target),
	JE(Target),
	JNE(Target),
	JMP(Target),
	
	SET(Target, Target),
	GET(Target, Target),
	SWAP(Target, Target),
	
	PUSH(Target),
	POP(Target),
	CALL(Target),
	RET,
	VPUSH(Target),
	VPOP(Target),
	
	NOP,
	HALT,
	EXTI(Target),
}

#[derive(Debug)]
pub enum Target {
	Register(Register),
	Literal(Literal),
	FromMem(FromMem),
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Literal {
	Number(u16),
	Identifier(String),
}

#[derive(Clone)]
#[derive(Debug)]
pub enum FromMem {
	Register(Register),
	RegisterLiteral(Register, Literal),
	TwoRegister(Register, bool, Register),
	TwoRegisterLiteral(Register, bool, Register, Literal),
}

pub fn parse(lex: &mut Lexer<Token>) -> Option<Instruction> {
	let token = lex.next();
	let instruction = match token {
		Some(token) => match token {
				Token::Keyword(keyword) => {
					let instruction = match_keyword(lex, keyword);
					Some(instruction)
				},
				_ => panic!("Unexpected token: {:?}", token)
			},
		None => None
	};
	
	instruction
}

fn match_keyword(lex: &mut Lexer<Token>, keyword: Keyword) -> Instruction {
	match keyword {
		Keyword::CONST => {
			assemble_CONST(lex)
		}
		Keyword::MARK => {
			assemble_MARK(lex)
		}
		Keyword::DATA => {
			assemble_DATA(lex)
		}
		Keyword::DSTR => {
			assemble_DSTR(lex)
		}
		
		Keyword::HALT => Instruction::HALT,
		Keyword::NOP => Instruction::NOP,
		Keyword::RET => Instruction::RET,
		
		Keyword::NEG	=> Instruction::NEG		(get_next_operand(lex, false)),
		Keyword::NOT	=> Instruction::NOT		(get_next_operand(lex, false)),
		Keyword::PUSH	=> Instruction::PUSH	(get_next_operand(lex, false)),
		Keyword::POP	=> Instruction::POP		(get_next_operand(lex, false)),
		Keyword::VPUSH	=> Instruction::VPUSH	(get_next_operand(lex, false)),
		Keyword::VPOP	=> Instruction::VPOP	(get_next_operand(lex, false)),
		Keyword::CALL	=> Instruction::CALL	(get_next_operand(lex, false)),
		Keyword::JMP	=> Instruction::JMP		(get_next_operand(lex, false)),
		Keyword::JG		=> Instruction::JG		(get_next_operand(lex, false)),
		Keyword::JNG	=> Instruction::JNG		(get_next_operand(lex, false)),
		Keyword::JL		=> Instruction::JL		(get_next_operand(lex, false)),
		Keyword::JNL	=> Instruction::JNL		(get_next_operand(lex, false)),
		Keyword::JE		=> Instruction::JE		(get_next_operand(lex, false)),
		Keyword::JNE	=> Instruction::JNE		(get_next_operand(lex, false)),
		Keyword::EXTI	=> Instruction::EXTI	(get_next_operand(lex, false)),
		
		Keyword::ADD	=> Instruction::ADD		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SUB	=> Instruction::SUB		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::MUL	=> Instruction::MUL		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::DIV	=> Instruction::DIV		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::MOD	=> Instruction::MOD		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SMUL	=> Instruction::SMUL	(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SDIV	=> Instruction::SDIV	(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SMOD	=> Instruction::SMOD	(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::AND	=> Instruction::AND		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::OR		=> Instruction::OR		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::XOR	=> Instruction::XOR		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SHL	=> Instruction::SHL		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SHR	=> Instruction::SHR		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SAR	=> Instruction::SAR		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SET	=> Instruction::SET		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::GET	=> Instruction::GET		(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::SWAP	=> Instruction::SWAP	(get_next_operand(lex, false), get_next_operand(lex, true)),
		Keyword::CMP	=> Instruction::CMP		(get_next_operand(lex, false), get_next_operand(lex, true)),
		
		_ => panic!("Not implemented"),
	}
}

fn assemble_CONST(lex: &mut Lexer<Token>) -> Instruction {
	let mut token = lex.next();
	
	let identifier = match token {
		Some(token) => {
			match token {
				Token::Identifier => {
						lex.slice()
					},
				_ => panic!("Malformed CONST: Expected Identifier, got {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed CONST: Expected Identifier, enountered EOF")
	};
	
	token = lex.next();
	
	let value = match token {
		Some(token) => {
			match token {
				Token::Number(num) => num,
				Token::Operator => {
					match assemble_signed_number(lex) {
						Ok(value) => value,
						Err(token) => panic!("Malformed CONST: Expected number, got {:?}, {}", token, token_display(lex, &token))
					}
				}
				_ => panic!("Malformed CONST: Expected Number, got {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed CONST: Expected Number, enountered EOF")
	};
	
	Instruction::CONST(identifier.to_owned(), value)
}

fn assemble_MARK(lex: &mut Lexer<Token>) -> Instruction {
	let token = lex.next();
	
	let identifier = match token {
		Some(token) => {
			match token {
				Token::Identifier => {
						lex.slice()
					},
				_ => panic!("Malformed MARK: Expected Identifier, got {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed MARK: Expected Identifier, enountered EOF")
	};
	
	Instruction::MARK(identifier.to_owned())
}

fn assemble_DATA(lex: &mut Lexer<Token>) -> Instruction {
	let mut values: Vec<u16> = vec!();
	
	let mut old_lex = lex.clone();
	
	let mut value_count: usize = 0;
	loop {
		let token = lex.next();
		
		let value = match token {
			Some(token) => {
				match token {
					Token::Number(num) => num,
					Token::Operator => {
						match assemble_signed_number(lex) {
							Ok(value) => value,
							Err(token) => panic!("Malformed DATA: Expected number, got {:?}, {}", token, token_display(lex, &token))
						}
					}
					_ => break
				}
			}
			None => break,
		};
		
		values.push(value);
		
		value_count += 1;
	}
	
	for _ in 0..value_count {
		old_lex.next();
	}
	
	*lex = old_lex;
	
	Instruction::DATA(values)
}

fn assemble_DSTR(lex: &mut Lexer<Token>) -> Instruction {
	let mut token = lex.next();
	
	let mut values: Vec<u16> = match token {
		Some(token) => {
			match token {
				Token::String => decompile_string(lex.slice()),
				_ => panic!("Malformed DSTR: Expected String, got {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed DSTR: Expected String, encountered EOF")
	};
	
	token = lex.next();
	let append: Option<u16> = match token {
		Some(token) => {
			match token {
				Token::Operator => {
					match assemble_signed_number(lex) {
						Ok(value) => Some(value),
						Err(token) => panic!("Malformed DSTR: Expected Number after signage, got {:?}, {}", token, token_display(lex, &token))
					}
				},
				Token::Number(value) => Some(value),
				_ => None,
			}
		}
		None => None,
	};
	
	match append {
		Some(appendix) => {
			for value in values.iter_mut() {
				*value = (*value & 0x00FF) | (appendix << 8);
			}
		}
		None => ()
	};
	
	Instruction::DSTR(values)
}

fn get_next_operand(lex: &mut Lexer<Token>, secondary: bool) -> Target {
	let token = lex.next();
	
	match token {
		Some(token) => {
			match token {
				Token::Number(value) => Target::Literal(Literal::Number(value)),
				Token::Operator => {
					match assemble_signed_number(lex) {
						Ok(value) => Target::Literal(Literal::Number(value)),
						Err(token) => panic!("Malformed Operand: Expected Number after signage, got {:?}, {}", token, token_display(lex, &token)),
					}
				},
				Token::Identifier => Target::Literal(Literal::Identifier(lex.slice().to_owned())),
				Token::Register(reg) => Target::Register(reg),
				Token::OpenBracket => if secondary {
					panic!("Syntax Error: Second operand can never be FromMem")
				} else {Target::FromMem(assemble_from_mem(lex))},
				_ => panic!("Malformed Operand: Expected Number, Identifier, Register, or FromMem, got {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed Operand: Expected Identifier, enountered EOF")
	}
}

fn decompile_string(string: &str) -> Vec<u16> {
	// Cut off quotes
	let string = &string[1..string.len() - 1];
	
	let mut values: Vec<u16> = vec!();
	
	for byte in string.chars() {
		values.push(byte as u16);
	}
	
	values
}

fn assemble_signed_number(lex: &mut Lexer<Token>) -> Result<u16, Token> {
	let operator = lex.slice();
	let negative = match operator {
		"+" => false,
		"-" => true,
		_ => panic!("Invalid number signage operator: {}", operator),
	};
	
	let number = match lex.next() {
		Some(token) => {
			match token {
				Token::Number(value) => value,
				_ => return Err(token),
			}
		},
		None => panic!("Malformed number: Expected Number, encountered EOF")
	};
	
	Ok(signed_number(negative, number))
}

fn signed_number(negative: bool, number: u16) -> u16 {
	if negative {-(number as i16) as u16} else {number}
}

fn assemble_from_mem(lex: &mut Lexer<Token>) -> FromMem {
	let first_register = match lex.next() {
		Some(token) => {
			match token {
				Token::Register(reg) => reg,
				_ => panic!("Malformed FromMem Operand: Expected Register, found {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed FromMem Operand: Expected Register, encountered EOF")
	};
	println!("Found first register");
	
	let first_offset_subtract = match lex.next() {
		Some(token) => {
			match token {
				Token::Operator => {
					let operator = lex.slice();
					match operator {
						"+" => false,
						"-" => true,
						_ => panic!("FromMem Operand: Invalid signage operator: {:?}", operator),
					}
				},
				Token::CloseBracket => {return FromMem::Register(first_register)},
				_ => panic!("Malformed FromMem Operand: Expected Operator or Close Bracket, found {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed FromMem Operand: Expected Operator or Close Bracket, encountered EOF")
	};
	println!("Found sign");
	
	let second_register = match lex.next() {
		Some(token) => {
			match token {
				Token::Register(reg) => reg,
				Token::Number(number) => {expect_close_bracket(lex); return FromMem::RegisterLiteral(first_register, Literal::Number(signed_number(first_offset_subtract, number)))},
				Token::Identifier => {expect_close_bracket(lex); return FromMem::RegisterLiteral(first_register, Literal::Identifier(lex.slice().to_owned()))},
				_ => panic!("Malformed FromMem Operand: Expected Register, Number, or Identifier, found {:?}, {}", token, token_display(lex, &token))
			}
		},
		None => panic!("Malformed FromMem Operand: Expected Register or Offset, ecountered EOF"),
	};
	println!("Found second register");
	
	let second_offset_subtract = match lex.next() {
		Some(token) => {
			match token {
				Token::Operator => {
					let operator = lex.slice();
					match operator {
						"+" => false,
						"-" => true,
						_ => panic!("FromMem Operand: Invalid signage operator: {:?}", operator),
					}
				},
				Token::CloseBracket => {return FromMem::TwoRegister(first_register, first_offset_subtract, second_register);},
				_ => panic!("Malformed FromMem Operand: Expected Operator or Close Bracket, found {:?}, {}", token, token_display(lex, &token))
			}
		}
		None => panic!("Malformed FromMem Operand: Expected Operator or Close Bracket, encountered EOF")
	};
	println!("Found sign");
	
	let last_offset = match lex.next() {
		Some(token) => {
			match token {
				Token::Number(number) => Literal::Number(signed_number(first_offset_subtract, number)),
				Token::Identifier => Literal::Identifier(lex.slice().to_owned()),
				_ => panic!("Malformed FromMem Operand: Expected Number or Identifier, found {:?}, {}", token, token_display(lex, &token))
			}
		},
		None => panic!("Malformed FromMem Operand: Expected Register or Offset, ecountered EOF"),
	};
	println!("Found last offset");
	
	expect_close_bracket(lex);
	FromMem::TwoRegisterLiteral(first_register, second_offset_subtract, second_register, last_offset)
}

fn expect_close_bracket(lex: &mut Lexer<Token>) {
	match lex.next() {
		Some(token) => match token {
			Token::CloseBracket => (),
			_ => panic!("Malformed FromMem Operand: Expected Close Bracket, found {:?}, {}", token, token_display(lex, &token))
		}
		None => panic!("Malformed FromMem Operand: Expected Close Bracket, encountered EOF")
	}
}