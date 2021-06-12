
#[derive(Debug)]
pub enum Keyword {
	CONST,
	MARK,
	DATA,
	
	ADD,
	SUB,
	MUL,
	SMUL,
	DIV,
	SDIV,
	MOD,
	SMOD,
	NEG,
	
	NOT,
	AND,
	OR,
	XOR,
	SHL,
	SHR,
	SAR,
	
	CMP,
	JG,
	JNG,
	JL,
	JNL,
	JE,
	JNE,
	JMP,
	
	SET,
	GET,
	SWAP,
	
	PUSH,
	POP,
	CALL,
	RET,
	VPUSH,
	VPOP,
	
	NOP,
	HALT,
	EXTI,
}

#[derive(Debug)]
pub enum Register {
	A,
	B,
	C,
	T,
	SP,
	VP,
	PP,
	FL,
}