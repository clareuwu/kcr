use crate::value::*;

#[derive(Debug)]
pub enum Op {
    Return,
}

pub struct Function {
    pub arity: u8,
    pub bc: Bytecode,
    pub name: String,
}

pub struct Closure {
    pub function: Function,
}

pub enum Constant {
    Number(f64),
    String(String),
    Function(Closure),
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
            println!("{:?}", op);
        }
    }

    pub fn push_code(&mut self, val: Op) {
        self.code.push(val);
    }

    pub fn push_constant(&mut self, val: Value) {
        self.constants.push(val);
    }
}
