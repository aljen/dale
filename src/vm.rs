pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 128;
pub const V_REG_SIZE: usize = 16;

pub const INITIAL_PC: u16 = 0x200;

pub struct Registers {
    pub v: [u8; V_REG_SIZE],
    pub i: u16,
    pub pc: u16,
    pub sp: u16,
}

pub struct VM {
    pub memory: [u8; MEMORY_SIZE],
    pub stack: [u16; STACK_SIZE],
    pub regs: Registers,
}

impl Registers {
    pub fn reset(&mut self) {
        self.v.fill(0);
        self.i = 0;
        self.pc = INITIAL_PC;
        self.sp = 0;
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            regs: Registers {
                v: [0; V_REG_SIZE],
                i: 0,
                pc: INITIAL_PC,
                sp: 0,
            },
        }
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.stack.fill(0);
        self.regs.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize() {
        let vm = VM::new();
        assert_eq!(vm.memory, [0; MEMORY_SIZE]);
        assert_eq!(vm.stack, [0; STACK_SIZE]);
        assert_eq!(vm.regs.v, [0; V_REG_SIZE]);
        assert_eq!(vm.regs.i, 0);
        assert_eq!(vm.regs.pc, INITIAL_PC);
        assert_eq!(vm.regs.sp, 0);
    }

    #[test]
    fn reset() {
        let mut vm = VM::new();

        vm.memory.fill(1);
        vm.stack.fill(1);
        vm.regs.v.fill(1);
        vm.regs.i = 1;
        vm.regs.pc = 1;
        vm.regs.sp = 1;

        vm.reset();

        assert_eq!(vm.memory, [0; MEMORY_SIZE]);
        assert_eq!(vm.stack, [0; STACK_SIZE]);
        assert_eq!(vm.regs.v, [0; V_REG_SIZE]);
        assert_eq!(vm.regs.i, 0);
        assert_eq!(vm.regs.pc, INITIAL_PC);
        assert_eq!(vm.regs.sp, 0);
    }
}
