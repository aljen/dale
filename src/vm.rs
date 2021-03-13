pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 128;
pub const V_REG_SIZE: usize = 16;

pub const INITIAL_PC: u16 = 0x200;

pub struct Registers {
    pub v: [u8; V_REG_SIZE],
    pub i: u16,
    pub pc: u16,
    pub sp: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
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
        self.delay_timer = 0;
        self.sound_timer = 0;
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
                delay_timer: 0,
                sound_timer: 0,
            },
        }
    }

    pub fn reset(&mut self) {
        self.memory.fill(0);
        self.stack.fill(0);
        self.regs.reset();
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        ((self.memory[address] as u16) << 8) | self.memory[address + 1] as u16
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_u16(&mut self, address: usize, value: u16) {
        self.memory[address] = (value >> 8) as u8;
        self.memory[address + 1] = (value & 0xff) as u8;
    }

    pub fn write_u8(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
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
        assert_eq!(vm.regs.delay_timer, 0);
        assert_eq!(vm.regs.sound_timer, 0);
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
        vm.regs.delay_timer = 1;
        vm.regs.sound_timer = 1;

        vm.reset();

        assert_eq!(vm.memory, [0; MEMORY_SIZE]);
        assert_eq!(vm.stack, [0; STACK_SIZE]);
        assert_eq!(vm.regs.v, [0; V_REG_SIZE]);
        assert_eq!(vm.regs.i, 0);
        assert_eq!(vm.regs.pc, INITIAL_PC);
        assert_eq!(vm.regs.sp, 0);
        assert_eq!(vm.regs.delay_timer, 0);
        assert_eq!(vm.regs.sound_timer, 0);
    }

    #[test]
    fn memory_read_u16() {
        let mut vm = VM::new();

        let address: usize = 10;

        vm.memory[address] = 0xaa;
        vm.memory[address + 1] = 0xbb;

        let read = vm.read_u16(address);

        assert_eq!(read, 0xaabb);
    }

    #[test]
    fn memory_read_u8() {
        let mut vm = VM::new();

        let address: usize = 10;

        vm.memory[address] = 0xaa;

        let read = vm.read_u8(address);

        assert_eq!(read, 0xaa);
    }

    #[test]
    fn memory_write_u16() {
        let mut vm = VM::new();

        let address: usize = 10;

        vm.write_u16(address, 0xaabb);

        assert_eq!(vm.memory[address], 0xaa);
        assert_eq!(vm.memory[address + 1], 0xbb);
    }

    #[test]
    fn memory_write_u8() {
        let mut vm = VM::new();

        let address: usize = 10;

        vm.write_u8(address, 0xaa);

        assert_eq!(vm.memory[address], 0xaa);
    }
}
