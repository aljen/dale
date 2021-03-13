mod vm;
use vm::*;

fn main() {
    let mut vm = VM::new();
    vm.reset();
}
