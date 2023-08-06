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
    pub constants: Vec<Constant>
}

impl Bytecode {
    pub fn new() -> Self {
        Bytecode { code: Vec::new(), constants: Vec::new() }
    }

    pub fn write_code(&mut self, val: Op) {
        self.code.push(val);
    }
}
