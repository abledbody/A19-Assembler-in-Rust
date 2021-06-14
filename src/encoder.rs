use std::collections::HashMap;
use crate::parser::*;
use crate::keywords::*;

#[derive(Debug)]
pub enum Byte {
	Definite(u16),
	Identifier(String),
	FromMemWithIdentifier(FromMem),
}

pub fn partially_encode(instruction: &Instruction, constants: &mut HashMap<String, u16>, byte_address: u16) -> Vec<Byte> {
	match instruction {
		Instruction::CONST(name, value) => {
			println!("Added constant {}, {}", name, *value);
			constants.insert(name.to_owned(), *value);
			vec!()
		},
		Instruction::MARK(name) => {
			println!("Added constant {}, {}", name, byte_address);
			constants.insert(name.to_owned(), byte_address);
			vec!()
		},
		Instruction::DATA(data)
		|Instruction::DSTR(data) => {
			data.into_iter().map(|value| Byte::Definite(*value)).collect()
		},
		
		Instruction::HALT	=> vec!(Byte::Definite(0x0000)),
		Instruction::NOP	=> vec!(Byte::Definite(0x0001)),
		Instruction::RET	=> vec!(Byte::Definite(0x0002)),
		
		Instruction::NEG	(target)	=> encode_one_op_instruction(0x0003, target),
		Instruction::NOT	(target)	=> encode_one_op_instruction(0x000D, target),
		Instruction::PUSH	(target)	=> encode_one_op_instruction(0x0017, target),
		Instruction::POP	(target)	=> encode_one_op_instruction(0x0021, target),
		Instruction::VPUSH	(target)	=> encode_one_op_instruction(0x002B, target),
		Instruction::VPOP	(target)	=> encode_one_op_instruction(0x0035, target),
		Instruction::CALL	(target)	=> encode_one_op_instruction(0x003F, target),
		Instruction::JMP	(target)	=> encode_one_op_instruction(0x0049, target),
		Instruction::JG		(target)	=> encode_one_op_instruction(0x0053, target),
		Instruction::JNG	(target)	=> encode_one_op_instruction(0x005D, target),
		Instruction::JL		(target)	=> encode_one_op_instruction(0x0067, target),
		Instruction::JNL	(target)	=> encode_one_op_instruction(0x0071, target),
		Instruction::JE		(target)	=> encode_one_op_instruction(0x007B, target),
		Instruction::JNE	(target)	=> encode_one_op_instruction(0x0085, target),
		Instruction::EXTI	(target)	=> encode_one_op_instruction(0x008F, target),
		
		Instruction::ADD	(lhs, rhs)	=> encode_two_op_instruction(0x0099, lhs, rhs),
		Instruction::SUB	(lhs, rhs)	=> encode_two_op_instruction(0x00F3, lhs, rhs),
		Instruction::MUL	(lhs, rhs)	=> encode_two_op_instruction(0x014D, lhs, rhs),
		Instruction::DIV	(lhs, rhs)	=> encode_two_op_instruction(0x01A7, lhs, rhs),
		Instruction::MOD	(lhs, rhs)	=> encode_two_op_instruction(0x0201, lhs, rhs),
		Instruction::SMUL	(lhs, rhs)	=> encode_two_op_instruction(0x025B, lhs, rhs),
		Instruction::SDIV	(lhs, rhs)	=> encode_two_op_instruction(0x02B5, lhs, rhs),
		Instruction::SMOD	(lhs, rhs)	=> encode_two_op_instruction(0x030F, lhs, rhs),
		Instruction::AND	(lhs, rhs)	=> encode_two_op_instruction(0x0369, lhs, rhs),
		Instruction::OR		(lhs, rhs)	=> encode_two_op_instruction(0x03C3, lhs, rhs),
		Instruction::XOR	(lhs, rhs)	=> encode_two_op_instruction(0x041D, lhs, rhs),
		Instruction::SHL	(lhs, rhs)	=> encode_two_op_instruction(0x0477, lhs, rhs),
		Instruction::SHR	(lhs, rhs)	=> encode_two_op_instruction(0x04D1, lhs, rhs),
		Instruction::SAR	(lhs, rhs)	=> encode_two_op_instruction(0x052B, lhs, rhs),
		Instruction::SET	(lhs, rhs)	=> encode_two_op_instruction(0x0585, lhs, rhs),
		Instruction::GET	(lhs, rhs)	=> encode_two_op_instruction(0x05DF, lhs, rhs),
		Instruction::SWAP	(lhs, rhs)	=> encode_two_op_instruction(0x0639, lhs, rhs),
		Instruction::CMP	(lhs, rhs)	=> encode_two_op_instruction(0x0693, lhs, rhs),
		
		_ => panic!("Not implemented"),
	}
}

pub fn encode_identifiers(constants: &HashMap<String, u16>, partially_encoded_file: &Vec<Byte>) -> Vec<u16> {
	let mut encoded_file = vec!();
	
	for byte in partially_encoded_file.into_iter() {
		match byte {
			Byte::Definite(value) => encoded_file.push(*value),
			Byte::Identifier(name) => {
				match constants.get_key_value(name) {
					Some((_, value)) => encoded_file.push(*value),
					None => panic!("Invalid identifier: \"{}\"", name),
				}
			},
			Byte::FromMemWithIdentifier(from_mem) => match from_mem {
				FromMem::RegisterLiteral(reg, literal) => match literal {
					Literal::Identifier(subtract, name) => {
						let offset = match constants.get_key_value(name) {
							Some((_, value)) => if *subtract {-(*value as i16) as u16} else {*value},
							None => panic!("Invalid identifier: \"{}\"", name),
						};
						
						encoded_file.push(register_offset(&reg) | (offset << 4))
					},
					_ => panic!("Theoretically unreachable state."),
				},
				FromMem::TwoRegisterLiteral(lhs, reg_subtract, rhs, literal) => match literal {
					Literal::Identifier(subtract, name) => {
						let offset = match constants.get_key_value(name) {
							Some((_, value)) => if *subtract {-(*value as i16) as u16} else {*value},
							None => panic!("Invalid identifier: \"{}\"", name),
						};
						
						encoded_file.push(encode_two_register_from_mem(&lhs, &rhs, *reg_subtract) | offset << 8)
					},
					_ => panic!("Theoretically unreachable state."),
				},
				_ => panic!("Theoretically unreachable state."),
			}
		}
	};
	
	encoded_file
}

fn encode_one_op_instruction(base_op: u16, target: &Target) -> Vec<Byte> {
	let mut data = vec!(Byte::Definite(base_op + target_offset(&target)));
	
	match target {
		Target::Register(_) => (),
		Target::Literal(literal) => match literal {
			Literal::Number(value) => data.push(Byte::Definite(*value)),
			Literal::Identifier(_, identifier) => data.push(Byte::Identifier(identifier.clone())),
		},
		Target::FromMem(from_mem) => data.push(from_mem_partial_encode(&from_mem)),
	};
	
	data
}

fn encode_two_op_instruction(base_op: u16, lhs: &Target, rhs: &Target) -> Vec<Byte> {
	let mut data = vec!(Byte::Definite(base_op + target_offset(&lhs) + target_offset(&rhs) * 10));
	
	match lhs {
		Target::Register(_) => (),
		Target::Literal(literal) => match literal {
			Literal::Number(value) => data.push(Byte::Definite(*value)),
			Literal::Identifier(_, identifier) => data.push(Byte::Identifier(identifier.clone())),
		},
		Target::FromMem(from_mem) => data.push(from_mem_partial_encode(&from_mem)),
	};
	
	match rhs {
		Target::Register(_) => (),
		Target::Literal(literal) => match literal {
			Literal::Number(value) => data.push(Byte::Definite(*value)),
			Literal::Identifier(_, identifier) => data.push(Byte::Identifier(identifier.clone())),
		},
		Target::FromMem(_) => panic!("Encoding error: Second operand cannot be FromMem!")
	};
	
	data
}

fn target_offset(target: &Target) -> u16 {
	match target {
		Target::Register(register) => register_offset(register),
		Target::Literal(_) => 8,
		Target::FromMem(_) => 9,
	}
}

fn register_offset(register: &Register) -> u16 {
	match register {
		Register::A => 0,
		Register::B => 1,
		Register::C => 2,
		Register::T => 3,
		Register::SP => 4,
		Register::VP => 5,
		Register::PP => 6,
		Register::FL => 7,
	}
}

fn from_mem_partial_encode(data: &FromMem) -> Byte {
	match data {
		FromMem::Register(reg) => Byte::Definite(register_offset(reg)),
		FromMem::RegisterLiteral(reg, literal) => match literal {
			Literal::Number(value) => {
				let value = (*value as i16 % (1 << 12)) as u16;
				Byte::Definite(register_offset(reg) | (value << 4))
			}
			Literal::Identifier(_,_) => Byte::FromMemWithIdentifier((*data).clone()),
		}
		FromMem::TwoRegister(left_reg, subtract, right_reg) => Byte::Definite(encode_two_register_from_mem(left_reg, right_reg, *subtract)),
		FromMem::TwoRegisterLiteral(left_reg, subtract, right_reg, literal) => match literal {
			Literal::Number(value) => {
				let value = (*value as i16 % (1 << 8)) as u16;
				Byte::Definite(encode_two_register_from_mem(left_reg, right_reg, *subtract) | (value << 8))
			}
			Literal::Identifier(_,_) => Byte::FromMemWithIdentifier((*data).clone()),
		}
	}
}

fn encode_two_register_from_mem(lhs: &Register, rhs: &Register, subtract: bool) -> u16 {
	register_offset(lhs) // Left register ID
	| 0b1000 // Two register indicator bit
	| (register_offset(rhs) << 4) // Right register ID
	| if subtract {0b1000_0000} else {0} // Register operation
}