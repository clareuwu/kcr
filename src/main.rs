mod bytecode;
use bytecode::*;
mod value;

fn main() {
    let mut bc = Bytecode::new();
    bc.push_code(Op::Return, 0);
    bc.push_code(Op::Return, 1);
    bc.push_code(Op::Return, 2);
    bc.push_code(Op::Return, 3);
    bc.push_code(Op::Return, 4);
    bc.push_code(Op::Return, 5);
    let cidx = bc.push_constant(value::Value::Number(5.));
    bc.push_code(Op::Constant(cidx), 6);
    bc.disassemble();
}
