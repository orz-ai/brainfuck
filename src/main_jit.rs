use std::io::prelude::*;

use dynasm::dynasm;
use dynasmrt::{DynasmApi, DynasmLabelApi};

mod ir;
mod opcode;

unsafe extern "sysv64" fn putchar(char: u8) {
    std::io::stdout().write_all(&[char]).unwrap()
}

#[derive(Default)]
struct Interpreter {}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let opcode_code = opcode::Code::from(data)?;
        let code = ir::Code::from(opcode_code.instrs)?;
        let mut loops = vec![];

        let mut ops = dynasmrt::x64::Assembler::new()?;
        let entry_point = ops.offset();

        // To understand the code below, you may want to read
        // https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI first.

        dynasm!(ops
            ; .arch x64
            ; mov rcx, rdi
        );

        for ir in code.instrs {
            match ir {
                ir::IR::SHL(x) => dynasm!(ops
                    ; sub rcx, x as i32 // sp -= x
                ),
                ir::IR::SHR(x) => dynasm!(ops
                    ; add rcx, x as i32 // sp += x
                ),
                ir::IR::ADD(x) => dynasm!(ops
                    ; add BYTE [rcx], x as i8 // sp* += x
                ),
                ir::IR::SUB(x) => dynasm!(ops
                    ; sub BYTE [rcx], x as i8 // sp* -= x
                ),
                ir::IR::PUTCHAR => dynasm!(ops
                    ; push rcx
                    ; mov  rdi, [rcx]
                    ; mov  rax, QWORD putchar as _
                    ; sub  rsp, BYTE 0x28 // See https://coding.imooc.com/learn/questiondetail/259971.html
                    ; call rax
                    ; add  rsp, BYTE 0x28
                    ; pop  rcx
                ),
                ir::IR::GETCHAR => {}
                ir::IR::JIZ(_) => {
                    let l = ops.new_dynamic_label();
                    let r = ops.new_dynamic_label();
                    loops.push((l, r));
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jz  => r
                        ; => l
                    )
                }
                ir::IR::JNZ(_) => {
                    let (l, r) = loops.pop().unwrap();
                    dynasm!(ops
                        ; cmp BYTE [rcx], 0
                        ; jnz => l
                        ; => r
                    )
                }
            }
        }
        dynasm!(ops
            ; ret
        );

        let exec_buffer = ops.finalize().unwrap();
        let mut memory: Box<[u8]> = vec![0; 65536].into_boxed_slice();
        let memory_addr_from = memory.as_mut_ptr();
        let fun: fn(memory_addr_from: *mut u8) =
            unsafe { std::mem::transmute(exec_buffer.ptr(entry_point)) };
        fun(memory_addr_from);

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    // assert!(args.len() >= 2);
    let mut f = std::fs::File::open("res/hello_word.bf")?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();
    i.run(c)
}