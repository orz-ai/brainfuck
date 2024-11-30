
mod opcode;
mod ir;

use std::io::{Read, Write};

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {

        let opcode_code = opcode::Code::from(data)?;
        let code = ir::Code::from(opcode_code.instrs)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // Program Counter
        let mut sp = 0; // Stack Pointer

        loop {
            if pc >= code_len {
                break;
            }

            match code.instrs[pc] {
                ir::IR::SHR(x) => {
                    sp += x as usize;
                    if sp >= self.stack.len() {
                        let expand = sp - self.stack.len() + 1;
                        for _ in 0..expand {
                            self.stack.push(0);
                        }
                    }
                },
                ir::IR::SHL(x) => sp = if sp == 0 { 0 } else { sp - x as usize },
                ir::IR::ADD(x) =>  self.stack[sp] = self.stack[sp].overflowing_add(x).0,
                ir::IR::SUB(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(x).0;
                },
                ir::IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                },
                ir::IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                },
                ir::IR::JIZ(x) => {
                    if self.stack[sp] == 0x00 {
                        pc = x as usize;
                    }
                },
                ir::IR::JNZ(x) => {
                    if self.stack[sp] != 0x00 {
                        pc = x as usize;
                    }
                },
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() {
    let mut interpreter = Interpreter::default();
    let data = std::fs::read("res/hello_word.bf").unwrap();
    interpreter.run(data).unwrap();
}