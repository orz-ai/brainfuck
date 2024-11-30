use crate::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32),
    JNZ(u32),
}

#[derive(Debug)]
pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instrs = Vec::new();
        let mut jstack = Vec::new();

        for e in &data {
            match e {
                Opcode::SHR => {
                    match instrs.last_mut() {
                        Some(IR::SHR(x)) => {
                            *x += 1;
                        }
                        _ => instrs.push(IR::SHR(1)),
                    }
                },
                Opcode::SHL => {
                    match instrs.last_mut() {
                        Some(IR::SHL(x)) => {
                            *x += 1;
                        }
                        _ => instrs.push(IR::SHL(1)),
                    }
                },
                Opcode::ADD => {
                    match instrs.last_mut() {
                        Some(IR::ADD(x)) => {
                            let (sum, _) = x.overflowing_add(1);
                            *x = sum;
                        }
                        _ => instrs.push(IR::ADD(1)),
                    }
                },
                Opcode::SUB => {
                    match instrs.last_mut() {
                        Some(IR::SUB(x)) => {
                            let (sum, _) = x.overflowing_add(1);
                            *x = sum;
                        }
                        _ => instrs.push(IR::SUB(1)),
                    }
                },
                Opcode::PUTCHAR => {
                    instrs.push(IR::PUTCHAR);
                },
                Opcode::GETCHAR => {
                    instrs.push(IR::GETCHAR);
                },
                Opcode::LB => {
                    instrs.push(IR::JIZ(0));
                    jstack.push(instrs.len() - 1);
                },
                Opcode::RB => {
                    let j = jstack.pop().ok_or("vec is empty")?;
                    instrs.push(IR::JNZ(j as u32));
                    let instr_len = instrs.len();
                    match &mut instrs[j]  {
                        IR::JIZ(x) => {
                            *x = instr_len as u32 - 1;
                        }
                        _ => unreachable!(),
                    }
                },
            }
        }

        Ok(Code {instrs})
    }

}