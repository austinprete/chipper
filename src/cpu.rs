use crate::rom::ROM;
use rand::Rng;

pub struct CPU {
    v: [u8; 16],
    i: u16,
    pc: u16,
    memory: [u8; 4096],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: usize,
    debug_mode: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            memory: [0; 4096],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            debug_mode: false,
        }
    }
    //
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[512..].clone_from_slice(&rom.data);
        println!("{:?}", self.memory[512])
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    pub fn run(&mut self) {
        const CYCLES_TO_RUN: u16 = 10_000;
        let mut cycles_ran = 0;
        loop {
            if self.pc >= 4096 {
                break;
            }
            self.execute_op();

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            cycles_ran += 1;

            if cycles_ran == CYCLES_TO_RUN {
                break;
            }

            // self.pc += 2;
        }
    }

    pub fn execute_op(&mut self) {
        let opcode = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16;

        let op1 = ((opcode & 0xF000) >> 12) as u8;
        let op2 = ((opcode & 0x0F00) >> 8) as usize;
        let op3 = ((opcode & 0x00F0) >> 4) as u8;
        let op4 = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        if self.debug_mode {
            println!("-------------------\nPC: {:#06X?}", self.pc);
        }

        // self.disassemble_op(opcode);

        self.pc += 2;

        let mut description: String;

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => {
                description = format!("Clear screen");

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x0, 0x0, 0xE, 0xE) => {
                description = format!("Return from a subroutine");

                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            // This instruction only applies to original CHIP-8 hardware
            (0x0, _, _, _) => {
                description = format!("Execute machine language subroutine at address {:#05X?}", nnn);
            }
            (0x1, _, _, _) => {
                description = format!("Jump to address {}", nnn);

                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                description = format!("Execute subroutine at address {:#05X?}", nnn);

                self.stack[self.sp] = self.pc;
                self.sp += 1;

                self.pc = nnn;
            }
            (0x3, x, _, _) => {
                description = format!("Skip the following instruction if V{} == {}", x, nn);

                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            (0x4, x, _, _) => {
                description = format!("Skip the following instruction if V{} != {}", x, nn);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x5, x, y, 0) => {
                description = format!("Skip the following instruction if V{} == V{}", x, y);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x6, x, _, _) => {
                description = format!("Store {} in register V{}", nn, x);

                self.v[x] = nn;
            }
            (0x7, x, _, _) => {
                description = format!("Add {} to register V{}", nn, x);

                self.v[x] += nn;
            }
            (0x8, x, y, 0x0) => {
                description = format!("Store V{} in V{}", y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x1) => {
                description = format!("Set V{} to V{} | V{}", x, x, y);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x2) => {
                description = format!("Set V{} to V{} & V{}", x, x, y);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x3) => {
                description = format!("Set V{} to V{} ^ V{}", x, x, y);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x4) => {
                description = format!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x5) => {
                description = format!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x6) => {
                description = format!("Store the value of register V{} shifted right one bit in register V{}\n\tSet register VF to the least significant bit prior to the shift", y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0x7) => {
                description = format!("Set register V{} to the value of V{} minus V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", x, y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x8, x, y, 0xE) => {
                description = format!("Store the value of register V{} shifted left one bit in register V{}\n\tSet register VF to the most significant bit prior to the shift", y, x);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0x9, x, y, 0x0) => {
                description = format!("Skip the following instruction if V{} != V{}", x, y);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0xA, _, _, _) => {
                description = format!("Store memory address {:#05X?} to register I", nnn);

                self.i = nnn;
            }
            (0xB, _, _, _) => {
                description = format!("Jump to address {} + V0", nnn);

                panic! {"UNIMPLEMENTED OP"}
            }
            (0xC, x, _, _) => {
                description = format!("Set V{} = random byte AND {:#04X?}", x, nn);

                // TODO: better random implementation here
                let mut rng = rand::thread_rng();
                let rnd = rng.gen_range(0, 256) as u8;
                self.v[x] = rnd & nn;
            }
            (0xD, x, y, n) => {
                description = format!("Draw sprite {:?} at x={} y={}", n, x, y);

                // TODO: implement drawing
            }
            (0xF, x, 0x0, 0x7) => {
                description = format!("Set V{} = delay timer", x);

                self.v[x] = self.delay_timer;
            }
            (0xF, x, 0x1, 0x5) => {
                description = format!("Set delay timer = V{}", x);

                self.delay_timer = self.v[x];
            }
            (0xF, x, 0x2, 0x9) => {
                description = format!("Set I = location of sprite for digit V{}.", x);

                self.i = self.v[x] as u16 * 5;
            }
            (0xF, x, 0x3, 0x3) => {
                description = format!("Store BCD representation of V{} in memory locations {:#06X?}, {:#06X?}, and {:#06X?}.", x, self.i, self.i + 1, self.i + 2);

                let mut num = self.v[x];

                let hundreds = num / 100;
                let tens = (num % 100) / 10;
                let ones = (num % 10);

                let start_address = self.i as usize;

                self.memory[start_address] = hundreds;
                self.memory[start_address] = tens;
                self.memory[start_address] = ones;
            }
            (0xF, x, 0x6, 0x5) => {
                description = format!("Read registers V0 through V{} from memory starting at location {:#06X?}.", x, self.i);

                for reg in 0..x {
                    self.v[reg] = self.memory[(self.i as usize) + reg];
                }
            }
            (_, _, _, _) => panic!("UNRECOGNIZED OP: {:#06X?}", opcode)
        }

        if self.debug_mode {
            println!("{}", description);
        }
    }

    // pub fn disassemble_op(&mut self, opcode: u16) {
    //     let op1 = ((opcode & 0xF000) >> 12) as u8;
    //     let op2 = ((opcode & 0x0F00) >> 8) as u8;
    //     let op3 = ((opcode & 0x00F0) >> 4) as u8;
    //     let op4 = (opcode & 0x000F) as u8;
    //
    //     let nnn = opcode & 0x0FFF;
    //     let nn = (opcode & 0x00FF) as u8;
    //
    //     print!("{:#06X?}: ", opcode);
    //
    //     match (op1, op2, op3, op4) {
    //         (0x0, 0x0, 0xE, 0x0) => println!("Clear screen"),
    //         (0x0, 0x0, 0xE, 0xE) => println!("Return from a subroutine"),
    //         (0x0, _, _, _) => println!("Execute machine language subroutine at address {:#05X?}", nnn),
    //         (0x1, _, _, _) => println!("Jump to address {}", nnn),
    //         (0x2, _, _, _) => println!("Execute subroutine at address {:#05X?}", nnn),
    //         (0x3, x, _, _) => println!("Skip the following instruction if V{} == {}", x, nn),
    //         (0x4, x, _, _) => println!("Skip the following instruction if V{} != {}", x, nn),
    //         (0x5, x, y, 0) => println!("Skip the following instruction if V{} == V{}", x, y),
    //         (0x6, x, _, _) => { println!("Store {} in register V{}", nn, x) }
    //         (0x7, x, _, _) => println!("Add {} to register V{}", nn, x),
    //         (0x8, x, y, 0x0) => println!("Store V{} in V{}", y, x),
    //         (0x8, x, y, 0x1) => println!("Set V{} to V{} | V{}", x, x, y),
    //         (0x8, x, y, 0x2) => println!("Set V{} to V{} & V{}", x, x, y),
    //         (0x8, x, y, 0x3) => println!("Set V{} to V{} ^ V{}", x, x, y),
    //         (0x8, x, y, 0x4) => println!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", y, x),
    //         (0x8, x, y, 0x5) => println!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", y, x),
    //         (0x8, x, y, 0x6) => println!("Store the value of register V{} shifted right one bit in register V{}\n\tSet register VF to the least significant bit prior to the shift", y, x),
    //         (0x8, x, y, 0x7) => println!("Set register V{} to the value of V{} minus V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", x, y, x),
    //         (0x8, x, y, 0xE) => println!("Store the value of register V{} shifted left one bit in register V{}\n\tSet register VF to the most significant bit prior to the shift", y, x),
    //         (0x9, x, y, 0x0) => println!("Skip the following instruction if V{} != V{}", x, y),
    //         (0xA, _, _, _) => println!("Store memory address {:#05X?} to register I", nnn),
    //         (0xB, _, _, _) => println!("Jump to address {} + V0", nnn),
    //         (0xC, x, _, _) => println!("Set V{} = random byte AND {:#04X?}", x, nn),
    //         (0xD, x, y, n) => println!("Draw sprite {:?} at x={} y={}", n, x, y),
    //         (0xF, x, 0x0, 0x7) => println!("Set V{} = delay timer", x),
    //         (0xF, x, 0x1, 0x5) => println!("Set delay timer = V{}", x),
    //         (0xF, x, 0x2, 0x9) => println!("Set I = location of sprite for digit V{}.", x),
    //         (0xF, x, 0x3, 0x3) => println!("Store BCD representation of V{} in memory locations {:#06X?}, {:#06X?}, and {:#06X?}.", x, self.i, self.i + 1, self.i + 2),
    //         (0xF, x, 0x6, 0x5) => println!("Read registers V0 through V{} from memory starting at location {:#06X?}.", x, self.i),
    //         (_, _, _, _) => println!("UNRECOGNIZED OP: {:#06X?}", opcode)
    //     }
    // }
}
