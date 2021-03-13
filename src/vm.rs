pub const MEMORY_SIZE: usize = 4096;
pub const STACK_SIZE: usize = 16;
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
        self.sp = STACK_SIZE as u16;
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
                sp: STACK_SIZE as u16,
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

    pub fn write_u16(&mut self, address: usize, value: u16) {
        self.memory[address] = (value >> 8) as u8;
        self.memory[address + 1] = (value & 0xff) as u8;
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_u8(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn step(&mut self) {
        let opcode = self.read_u16(self.regs.pc as usize);
        self.process_opcode(opcode);
    }

    pub fn process_opcode(&mut self, opcode: u16) {
        let op = (opcode >> 12) as u8;
        println!("opcode: {:#06x} op: {:#04x}", opcode, op);

        match op {
            0x0 => self.process_opcode_0(opcode),
            0x1 => self.process_opcode_1nnn(opcode),
            0x2 => self.process_opcode_2nnn(opcode),
            0x3 => self.process_opcode_3xkk(opcode),
            0x4 => self.process_opcode_4xkk(opcode),
            0x5 => self.process_opcode_5xy0(opcode),
            0x6 => self.process_opcode_6xkk(opcode),
            0x7 => self.process_opcode_7xkk(opcode),
            0x8 => self.process_opcode_8(opcode),
            0x9 => self.process_opcode_9xy0(opcode),
            0xa => self.process_opcode_annn(opcode),
            0xb => self.process_opcode_bnnn(opcode),
            0xc => self.process_opcode_cxkk(opcode),
            0xd => self.process_opcode_dxyn(opcode),
            0xe => self.process_opcode_e(opcode),
            0xf => self.process_opcode_f(opcode),
            _ => panic!("Invalid opcode {:#06x}", opcode),
        }
    }

    fn process_opcode_0(&mut self, opcode: u16) {
        let value = opcode & 0x0fff;
        match value {
            0x00ee => self.process_opcode_00ee(),
            0x00e0 => self.process_opcode_00e0(),
            _ => self.process_opcode_0nnn(value),
        }
    }

    // CLS
    fn process_opcode_00e0(&mut self) {
        unimplemented!();
    }

    // RET
    fn process_opcode_00ee(&mut self) {
        unimplemented!();
    }

    // SYS addr
    fn process_opcode_0nnn(&mut self, _opcode: u16) {
        unimplemented!();
    }

    // JP addr
    fn process_opcode_1nnn(&mut self, opcode: u16) {
        self.regs.pc = opcode & 0x0fff;
    }

    // CALL addr
    fn process_opcode_2nnn(&mut self, opcode: u16) {
        self.regs.pc += 2;
        self.regs.sp -= 1;
        self.stack[self.regs.sp as usize] = self.regs.pc;
        self.regs.pc = opcode & 0x0fff;
    }

    // SE Vx, byte
    fn process_opcode_3xkk(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let kk: u8 = (opcode & 0x00ff) as u8;

        self.regs.pc += 2;

        if self.regs.v[x as usize] == kk {
            self.regs.pc += 2;
        }
    }

    // SNE Vx, byte
    fn process_opcode_4xkk(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let kk: u8 = (opcode & 0x00ff) as u8;

        self.regs.pc += 2;

        if self.regs.v[x as usize] != kk {
            self.regs.pc += 2;
        }
    }

    // SE Vx, Vy
    fn process_opcode_5xy0(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let y: u8 = ((opcode >> 4) & 0x000f) as u8;

        self.regs.pc += 2;

        if self.regs.v[x as usize] == self.regs.v[y as usize] {
            self.regs.pc += 2;
        }
    }

    // LD Vx, byte
    fn process_opcode_6xkk(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let kk: u8 = (opcode & 0x00ff) as u8;

        self.regs.pc += 2;
        self.regs.v[x as usize] = kk;
    }

    // ADD Vx, byte
    fn process_opcode_7xkk(&mut self, _opcode: u16) {
        unimplemented!();
    }

    fn process_opcode_8(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let y: u8 = ((opcode >> 4) & 0x000f) as u8;
        let op: u8 = (opcode & 0x000f) as u8;

        match op {
            0x0 => self.process_opcode_8xy0(x, y),
            0x1 => self.process_opcode_8xy1(x, y),
            0x2 => self.process_opcode_8xy2(x, y),
            0x3 => self.process_opcode_8xy3(x, y),
            0x4 => self.process_opcode_8xy4(x, y),
            0x5 => self.process_opcode_8xy5(x, y),
            0x6 => self.process_opcode_8xy6(x, y),
            0x7 => self.process_opcode_8xy7(x, y),
            0xe => self.process_opcode_8xye(x, y),
            _ => panic!("Invalid opcode {:#06x}", opcode),
        }
    }

    // LD Vx, Vy
    fn process_opcode_8xy0(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // OR Vx, Vy
    fn process_opcode_8xy1(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // AND Vx, Vy
    fn process_opcode_8xy2(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // XOR Vx, Vy
    fn process_opcode_8xy3(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // ADD Vx, Vy
    fn process_opcode_8xy4(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // SUB Vx, Vy
    fn process_opcode_8xy5(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // SHR Vx {, Vy}
    fn process_opcode_8xy6(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // SUBN Vx, Vy
    fn process_opcode_8xy7(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // SHL Vx {, Vy}
    fn process_opcode_8xye(&mut self, _x: u8, _y: u8) {
        unimplemented!();
    }

    // SNE Vx, Vy
    fn process_opcode_9xy0(&mut self, _opcode: u16) {
        unimplemented!();
    }

    // LD I, addr
    fn process_opcode_annn(&mut self, _opcode: u16) {
        unimplemented!();
    }

    // JP V0, addr
    fn process_opcode_bnnn(&mut self, _opcode: u16) {
        unimplemented!();
    }

    // RND Vx, byte
    fn process_opcode_cxkk(&mut self, _opcode: u16) {
        unimplemented!();
    }

    // DRW Vx, Vy, nibble
    fn process_opcode_dxyn(&mut self, _opcode: u16) {
        unimplemented!();
    }

    fn process_opcode_e(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let op: u8 = (opcode & 0x000f) as u8;

        match op {
            0x9e => self.process_opcode_ex9e(x),
            0xa1 => self.process_opcode_exa1(x),
            _ => panic!("Invalid opcode {:#06x}", opcode),
        }
    }

    // SKP Vx
    fn process_opcode_ex9e(&mut self, _x: u8) {
        unimplemented!();
    }

    // SKNP Vx
    fn process_opcode_exa1(&mut self, _x: u8) {
        unimplemented!();
    }

    fn process_opcode_f(&mut self, opcode: u16) {
        let x: u8 = ((opcode >> 8) & 0x000f) as u8;
        let op: u8 = (opcode & 0x000f) as u8;

        match op {
            0x07 => self.process_opcode_fx07(x),
            0x0a => self.process_opcode_fx0a(x),
            0x15 => self.process_opcode_fx15(x),
            0x18 => self.process_opcode_fx18(x),
            0x1e => self.process_opcode_fx1e(x),
            0x29 => self.process_opcode_fx29(x),
            0x33 => self.process_opcode_fx33(x),
            0x55 => self.process_opcode_fx55(x),
            0x65 => self.process_opcode_fx65(x),
            _ => panic!("Invalid opcode {:#06x}", opcode),
        }
    }

    // LD Vx, DT
    fn process_opcode_fx07(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD Vx, K
    fn process_opcode_fx0a(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD DT, Vx
    fn process_opcode_fx15(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD ST, Vx
    fn process_opcode_fx18(&mut self, _x: u8) {
        unimplemented!();
    }

    // ADD I, Vx
    fn process_opcode_fx1e(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD F, Vx
    fn process_opcode_fx29(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD B, Vx
    fn process_opcode_fx33(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD [I], Vx
    fn process_opcode_fx55(&mut self, _x: u8) {
        unimplemented!();
    }

    // LD Vx, [I]
    fn process_opcode_fx65(&mut self, _x: u8) {
        unimplemented!();
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
        assert_eq!(vm.regs.sp, STACK_SIZE as u16);
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
        assert_eq!(vm.regs.sp, STACK_SIZE as u16);
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

    #[test]
    fn opcode_1nnn() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);

        vm.write_u16(vm.regs.pc as usize, 0x1123); // JP 0x123
        vm.step();

        assert_eq!(vm.regs.pc, 0x0123);
    }

    #[test]
    fn opcode_2nnn() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);
        assert_eq!(vm.regs.sp, STACK_SIZE as u16);

        vm.write_u16(vm.regs.pc as usize, 0x2123); // CALL 0x123
        vm.step();

        assert_eq!(vm.regs.pc, 0x0123);
        assert_eq!(vm.regs.sp, (STACK_SIZE - 1) as u16);
        assert_eq!(vm.stack[vm.regs.sp as usize], INITIAL_PC + 2);
    }

    #[test]
    fn opcode_3xkk() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);

        vm.write_u16(vm.regs.pc as usize, 0x3123); // SE V1, 0x23
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 2);

        vm.regs.v[1] = 0x23;

        vm.write_u16(vm.regs.pc as usize, 0x3123); // SE V1, 0x23
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 6);
    }

    #[test]
    fn opcode_4xkk() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);

        vm.write_u16(vm.regs.pc as usize, 0x4123); // SNE V1, 0x23
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 4);

        vm.regs.v[1] = 0x23;

        vm.write_u16(vm.regs.pc as usize, 0x4123); // SNE V1, 0x23
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 6);
    }

    #[test]
    fn opcode_5xy0() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);

        vm.write_u16(vm.regs.pc as usize, 0x5120); // SE V1, V2
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 4);

        vm.regs.v[1] = 0x23;

        vm.write_u16(vm.regs.pc as usize, 0x5120); // SE V1, V2
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 6);
    }

    #[test]
    fn opcode_6xkk() {
        let mut vm = VM::new();

        assert_eq!(vm.regs.pc, INITIAL_PC);
        assert_eq!(vm.regs.v[1], 0x00);

        vm.write_u16(vm.regs.pc as usize, 0x6123); // LD V1, 0x23
        vm.step();

        assert_eq!(vm.regs.pc, INITIAL_PC + 2);
        assert_eq!(vm.regs.v[1], 0x23);
    }
}
