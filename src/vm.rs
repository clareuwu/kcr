use crate::bytecode::*;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum InterpretResult {
    InterpretOK,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    bc: Bytecode,
    ip: usize,
}

impl VM {
    pub fn new(bc: Bytecode) -> Self {
        VM { bc, ip: 0}
    }

    pub fn interpret(&mut self) -> InterpretResult {
        return InterpretResult::InterpretOK;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_vm() {
        let bc = Bytecode::new();
        let vm = VM::new(bc);
        assert!(vm.bc.code.is_empty());
        assert!(vm.bc.constants.is_empty());
        assert!(vm.ip == 0);
    }

    #[test]
    fn test_vm_interpret() {
        let bc = Bytecode::new();
        let mut vm = VM::new(bc);
        assert_eq!(vm.interpret(), InterpretResult::InterpretOK);
    }
}
