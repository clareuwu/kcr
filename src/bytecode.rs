use crate::value::*;

#[derive(Debug)]
pub enum Op {
    Return,
    Constant(usize),
}

pub struct Bytecode {
    pub code: Vec<Op>,
    pub constants: Vec<Value>
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { code: Vec::new(), constants: Vec::new() }
    }

    pub fn disassemble(&self) {
        for op in &self.code {
            match op {
                Op::Constant(idx) => println!("constant, index: {}", idx),
                _ => println!("{:?}", op)
            }
        }
    }

    pub fn push_code(&mut self, val: Op) {
        self.code.push(val);
    }

    pub fn push_constant(&mut self, val: Value) -> usize {
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }
}
