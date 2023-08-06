mod bytecode;

fn main() {
    let mut bc = bytecode::Bytecode::new();
    bc.write_code(bytecode::Op::Return);
    println!("{:?}", bc.code[0]);
}
