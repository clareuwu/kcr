mod bytecode;
use bytecode::*;
mod value;

fn main() {
    let mut bc = Bytecode::new();
    bc.push_code(Op::Return);
    bc.push_code(Op::Return);
    bc.push_code(Op::Return);
    bc.push_code(Op::Return);
    bc.disassemble();
    bc.push_code(Op::Return);
    bc.push_code(Op::Return);
    bc.disassemble();
    bc.push_constant(value::Value::Number(5.));
    bc.push_constant(value::Value::Number(9.));
    println!("{:?}", &bc.constants.last());
}
