// > Increment the data pointer (to point to the next cell to the right).
// < Decrement the data pointer (to point to the next cell to the left).
// + Increment the byte at the data pointer.
// - Decrement the byte at the data pointer.
// . Output the byte at the data pointer.
// , Accept one byte of input, and store it at the data pointer.
// [ Jump forward to the matching ] instruction, or if the byte at the data pointer is zero, jump forward to the matching ] instruction.
// ] Jump backward to the matching [ instruction, or if the byte at the data pointer is non-zero, jump backward to the matching [ instruction.

#[derive(Debug, PartialEq)]
pub enum Opcode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2C,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!(),
        }
    }
}

pub struct Code {
    pub instrs: Vec<Opcode>,
    pub jtable: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8,
        ];

        let instrs: Vec<Opcode> = data.
            iter().
            filter(|x| dict.contains(x)).
            map(|x| Opcode::from(*x)).
            collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable:std::collections::HashMap<usize, usize> = std::collections::HashMap::new();

        for (i, instr) in instrs.iter().enumerate() {
            if Opcode::LB == *instr {
                jstack.push(i);
            }
            if Opcode::RB == *instr {
                let j = jstack.pop().ok_or("vec is empty")?;

                // record position of jump to and back
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }
        Ok(Code { instrs, jtable })
    }
}