
enum OpCode {
    PUSH,
    PLUS,
    MULT,
}

pub struct Instruction {
    op_code: OpCode,
    value: i64,
}

impl Instruction {

    fn new_nullary(op_code: OpCode) -> Instruction {
        Instruction {
            op_code,
            value: 0,
        }
    }

    fn new_unary(op_code: OpCode, value: i64) -> Instruction {
        Instruction {
            op_code,
            value,
        }
    }

    pub fn new_push(value: i64) -> Instruction {
        Instruction::new_unary(OpCode::PUSH, value)
    }

    pub fn new_plus() -> Instruction {
        Instruction::new_nullary(OpCode::PLUS)
    }

    pub fn new_mult() -> Instruction {
        Instruction::new_nullary(OpCode::MULT)
    }
}

pub struct VM {
    instructions: Vec<Instruction>,
    stack: Vec<i64>,
}

impl VM {

    pub fn new(instructions: Vec<Instruction>) -> VM {
        VM {
            instructions,
            stack: Vec::new(),
        }
    }

    pub fn run(self: &mut VM) -> Option<i64> {
        for instruction in &self.instructions {
            match instruction.op_code {
                OpCode::PUSH => self.stack.push(instruction.value),
                OpCode::PLUS => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                },
                OpCode::MULT => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left * right);
                },
            }
        }
        if self.stack.len() == 0 {
            None
        } else {
            self.stack.pop()
        }
    }
}

