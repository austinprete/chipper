use crate::rom::ROM;

pub struct CPU {
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub memory: [u8; 4096],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub sp: usize,
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
        }
    }
    //
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[512..].clone_from_slice(&rom.data);
        println!("{:?}", self.memory[512])
    }

    pub fn run(&mut self) {
        const CYCLES_TO_RUN: u8 = 20;
        let mut cycles_ran = 0;
        loop {
            if self.pc >= 4096 {
                break;
            }
            self.execute_op();

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
        let op2 = ((opcode & 0x0F00) >> 8) as u8;
        let op3 = ((opcode & 0x00F0) >> 4) as u8;
        let op4 = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;


        println!("-------------------\nPC: {:#06X?}", self.pc);

        self.debug_op(opcode);

        self.pc += 2;

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => panic! {"UNIMPLEMENTED OP"},
            (0x0, 0x0, 0xE, 0xE) => panic! {"UNIMPLEMENTED OP"},
            (0x0, _, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0x1, _, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0x2, _, _, _) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;

                self.pc = nnn;
            },
            (0x3, x, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0x4, x, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0x5, x, y, 0) => panic! {"UNIMPLEMENTED OP"},
            (0x6, x, _, _) => {
                self.v[x as usize] = nn;
                println!("{:?}", self.pc);
            }
            (0x7, x, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x0) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x1) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x2) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x3) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x4) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x5) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x6) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0x7) => panic! {"UNIMPLEMENTED OP"},
            (0x8, x, y, 0xE) => panic! {"UNIMPLEMENTED OP"},
            (0x9, x, y, 0x0) => panic! {"UNIMPLEMENTED OP"},
            (0xA, _, _, _) => {
                self.i = nnn;
            }
            (0xB, _, _, _) => panic! {"UNIMPLEMENTED OP"},
            (0xD, x, y, n) => {
                println!("Draw sprite {:?} at x={} y={}", n, x, y);
            }
            (_, _, _, _) => panic!("UNRECOGNIZED OP: {:?}", opcode)
        }
    }

    pub fn debug_op(&mut self, opcode: u16) {
        let op1 = ((opcode & 0xF000) >> 12) as u8;
        let op2 = ((opcode & 0x0F00) >> 8) as u8;
        let op3 = ((opcode & 0x00F0) >> 4) as u8;
        let op4 = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        print!("{:#06X?}: ", opcode);

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => println!("Clear screen"),
            (0x0, 0x0, 0xE, 0xE) => println!("Return from a subroutine"),
            (0x0, _, _, _) => println!("Execute machine language subroutine at address {:#05X?}", nnn),
            (0x1, _, _, _) => println!("Jump to address {}", nnn),
            (0x2, _, _, _) => println!("Execute subroutine at address {:#05X?}", nnn),
            (0x3, x, _, _) => println!("Skip the following instruction if V{} == {}", x, nn),
            (0x4, x, _, _) => println!("Skip the following instruction if V{} != {}", x, nn),
            (0x5, x, y, 0) => println!("Skip the following instruction if V{} == V{}", x, y),
            (0x6, x, _, _) => { println!("Store {} in register V{}", nn, x) }
            (0x7, x, _, _) => println!("Add {} to register V{}", nn, x),
            (0x8, x, y, 0x0) => println!("Store V{} in V{}", y, x),
            (0x8, x, y, 0x1) => println!("Set V{} to V{} | V{}", x, x, y),
            (0x8, x, y, 0x2) => println!("Set V{} to V{} & V{}", x, x, y),
            (0x8, x, y, 0x3) => println!("Set V{} to V{} ^ V{}", x, x, y),
            (0x8, x, y, 0x4) => println!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", y, x),
            (0x8, x, y, 0x5) => println!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", y, x),
            (0x8, x, y, 0x6) => println!("Store the value of register V{} shifted right one bit in register V{}\n\tSet register VF to the least significant bit prior to the shift", y, x),
            (0x8, x, y, 0x7) => println!("Set register V{} to the value of V{} minus V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", x, y, x),
            (0x8, x, y, 0xE) => println!("Store the value of register V{} shifted left one bit in register V{}\n\tSet register VF to the most significant bit prior to the shift", y, x),
            (0x9, x, y, 0x0) => println!("Skip the following instruction if V{} != V{}", x, y),
            (0xA, _, _, _) => println!("Store memory address {:#05X?} to register I", nnn),
            (0xB, _, _, _) => println!("Jump to address {} + V0", nnn),
            (0xD, x, y, n) => println!("Draw sprite {:?} at x={} y={}", n, x, y),
            (_, _, _, _) => println!("UNRECOGNIZED OP: {:#06X?}", opcode)
        }
    }
}
