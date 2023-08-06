use crate::value::*;

#[derive(Debug)]
pub enum Op {
    Return,
    Constant(usize),
}

pub struct Bytecode {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { code: Vec::new(), constants: Vec::new(), lines: Vec::new(), }
    }

    pub fn disassemble(&self) {
        for (i, op) in self.code.iter().enumerate() {
            match op {
                Op::Constant(idx) => println!("{} - constant, index: {}", self.lines[i], idx),
                _ => println!("{} - {:?}", self.lines[i], op)
            }
        }
    }

    pub fn push_code(&mut self, val: Op, line: usize) {
        self.code.push(val);
        self.lines.push(line);
    }

    pub fn push_constant(&mut self, val: Value) -> usize {
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }
}
