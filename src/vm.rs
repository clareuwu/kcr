use crate::bytecode::*;

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    bc: Bytecode,
}

impl VM {
    pub fn new(bc: Bytecode) -> Self {
        VM { bc, }
    }
}
