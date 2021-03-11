use std::thread::sleep;
use std::time;
use std::time::Duration;

use rand::{random, Rng};
use rand::rngs::ThreadRng;

use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::rom::ROM;
use crate::SCALE_FACTOR;

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
    keyboard: Keyboard,
    display: Display,
    rng: ThreadRng,
    output_buffer: Vec<u32>,
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

impl CPU {
    pub fn new(keyboard: Keyboard, display: Display) -> CPU {
        let height = (display.get_height() / SCALE_FACTOR);
        let width = (display.get_width() / SCALE_FACTOR);
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
            keyboard,
            display,
            output_buffer: vec![0; width * height],
            rng: rand::thread_rng(),
        }
    }
    //
    pub fn load_rom(&mut self, rom: ROM) {
        self.memory[0x200..].clone_from_slice(&rom.data);
        println!("{:?}", self.memory[0x200])
    }

    pub fn enable_debug(&mut self) {
        self.debug_mode = true;
    }

    pub fn load_fontset(&mut self) {
        // 0
        self.memory[0] = 0xF0;
        self.memory[1] = 0x90;
        self.memory[2] = 0x90;
        self.memory[3] = 0x90;
        self.memory[4] = 0xF0;
        // 1
        self.memory[5] = 0x20;
        self.memory[6] = 0x60;
        self.memory[7] = 0x20;
        self.memory[8] = 0x20;
        self.memory[9] = 0x70;
        // 2
        self.memory[10] = 0xF0;
        self.memory[11] = 0x10;
        self.memory[12] = 0xF0;
        self.memory[13] = 0x80;
        self.memory[14] = 0xF0;
        // 3
        self.memory[15] = 0xF0;
        self.memory[16] = 0x10;
        self.memory[17] = 0xF0;
        self.memory[18] = 0x10;
        self.memory[19] = 0xF0;
        // 4
        self.memory[20] = 0x90;
        self.memory[21] = 0x90;
        self.memory[22] = 0xF0;
        self.memory[23] = 0x10;
        self.memory[24] = 0x10;
        // 5
        self.memory[25] = 0xF0;
        self.memory[26] = 0x80;
        self.memory[27] = 0xF0;
        self.memory[28] = 0x10;
        self.memory[29] = 0xF0;
        // 6
        self.memory[30] = 0xF0;
        self.memory[31] = 0x80;
        self.memory[32] = 0xF0;
        self.memory[33] = 0x90;
        self.memory[34] = 0xF0;
        // 7
        self.memory[35] = 0xF0;
        self.memory[36] = 0x10;
        self.memory[37] = 0x20;
        self.memory[38] = 0x40;
        self.memory[39] = 0x40;
        // 8
        self.memory[40] = 0xF0;
        self.memory[41] = 0x90;
        self.memory[42] = 0xF0;
        self.memory[43] = 0x90;
        self.memory[44] = 0xF0;
        // 9
        self.memory[45] = 0xF0;
        self.memory[46] = 0x90;
        self.memory[47] = 0xF0;
        self.memory[48] = 0x10;
        self.memory[49] = 0xF0;
        // A
        self.memory[50] = 0xF0;
        self.memory[51] = 0x90;
        self.memory[52] = 0xF0;
        self.memory[53] = 0x90;
        self.memory[54] = 0x90;
        // B
        self.memory[55] = 0xE0;
        self.memory[56] = 0x90;
        self.memory[57] = 0xE0;
        self.memory[58] = 0x90;
        self.memory[59] = 0xE0;
        // C
        self.memory[60] = 0xF0;
        self.memory[61] = 0x80;
        self.memory[62] = 0x80;
        self.memory[63] = 0x80;
        self.memory[64] = 0xF0;
        // D
        self.memory[65] = 0xE0;
        self.memory[66] = 0x90;
        self.memory[67] = 0x90;
        self.memory[68] = 0x90;
        self.memory[69] = 0xE0;
        // E
        self.memory[70] = 0xF0;
        self.memory[71] = 0x80;
        self.memory[72] = 0xF0;
        self.memory[73] = 0x80;
        self.memory[74] = 0xF0;
        // F
        self.memory[75] = 0xF0;
        self.memory[76] = 0x80;
        self.memory[77] = 0xF0;
        self.memory[78] = 0x80;
        self.memory[79] = 0x80;
    }

    pub fn run(&mut self) {
        const CYCLES_TO_RUN: u16 = 10_000;
        let mut cycles_ran = 0;

        self.load_fontset();

        loop {
            if self.pc >= 4096 {
                break;
            }
            self.keyboard.get_input(self.display.window.clone());
            self.execute_op();
            sleep(time::Duration::from_millis(16));

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            cycles_ran += 1;
            println!("{0}", cycles_ran);

            if cycles_ran == CYCLES_TO_RUN {
                break;
            }
        }
    }

    pub fn print_debug(&mut self, message: String) {
        if self.debug_mode {
            println!("{}", message);
        }
    }

    pub fn execute_op(&mut self) {
        self.print_debug(format!("-------------------\nPC: {:#06X?}", self.pc));
        let opcode = ((self.memory[self.pc as usize] as u16) << 8) | self.memory[(self.pc + 1) as usize] as u16;
        self.print_debug(format!("OPCODE: {:#06X?}", opcode));

        self.pc += 2;

        let op1 = ((opcode & 0xF000) >> 12) as u8;
        let op2 = ((opcode & 0x0F00) >> 8) as usize;
        let op3 = ((opcode & 0x00F0) >> 4) as u8;
        let op4 = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        // self.disassemble_op(opcode);

        match (op1, op2, op3, op4) {
            (0x0, 0x0, 0xE, 0x0) => {
                self.print_debug(format!("Clear screen"));

                for i in 0..self.output_buffer.len() {
                    self.output_buffer[i] = 0;
                }
                self.display.update_buffer(&self.output_buffer);
            }
            (0x0, 0x0, 0xE, 0xE) => {
                self.print_debug(format!("Return from a subroutine"));

                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            // This instruction only applies to original CHIP-8 hardware
            (0x0, _, _, _) => {
                self.print_debug(format!("Execute machine language subroutine at address {:#05X?}", nnn));
            }
            (0x1, _, _, _) => {
                self.print_debug(format!("Jump to address {}", nnn));

                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                self.print_debug(format!("Execute subroutine at address {:#05X?}", nnn));

                self.stack[self.sp] = self.pc;
                self.sp += 1;

                self.pc = nnn;
            }
            (0x3, x, _, _) => {
                self.print_debug(format!("Skip the following instruction if V{} == {}", x, nn));

                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            (0x4, x, _, _) => {
                self.print_debug(format!("Skip the following instruction if V{} != {}", x, nn));

                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            (0x5, x, y, 0) => {
                self.print_debug(format!("Skip the following instruction if V{} == V{}", x, y));

                if self.v[x] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0x6, x, _, _) => {
                self.print_debug(format!("Store {} in register V{}", nn, x));

                self.v[x] = nn;
            }
            (0x7, x, _, _) => {
                self.print_debug(format!("Add {} to register V{}", nn, x));

                self.v[x] = (self.v[x] as u8).wrapping_add(nn);
            }
            (0x8, x, y, 0x0) => {
                self.print_debug(format!("Store V{} in V{}", y, x));

                self.v[x] = self.v[y as usize]
            }
            (0x8, x, y, 0x1) => {
                self.print_debug(format!("Set V{} to V{} | V{}", x, x, y));

                self.v[x] |= self.v[y as usize];
            }
            (0x8, x, y, 0x2) => {
                self.print_debug(format!("Set V{} to V{} & V{}", x, x, y));

                self.v[x] &= self.v[y as usize];
            }
            (0x8, x, y, 0x3) => {
                self.print_debug(format!("Set V{} to V{} ^ V{}", x, x, y));

                self.v[x] ^= self.v[y as usize];
            }
            (0x8, x, y, 0x4) => {
                self.print_debug(format!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", y, x));

                let new_val: u16 = self.v[x] as u16 + self.v[y as usize] as u16;
                self.v[0xF] = if new_val >= (u8::MAX as u16) {
                    1
                } else {
                    0
                };

                self.v[x] = (new_val & 0x0F) as u8;
            }
            (0x8, x, y, 0x5) => {
                self.print_debug(format!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", y, x));

                self.v[0xF] = if self.v[x] > self.v[y as usize] {
                    1
                } else {
                    0
                };

                self.v[x] = (self.v[x] as u8).wrapping_sub(self.v[y as usize]);
            }
            (0x8, x, _, 0x6) => {
                self.print_debug(format!("Shift the value of register V{} right one bit\n\tSet register VF to the least significant bit prior to the shift", x));

                self.v[0xF] = self.v[x] & 0x1;
                self.v[x] >>= 1;
            }
            (0x8, x, y, 0x7) => {
                self.print_debug(format!("Set register V{} to the value of V{} minus V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", x, y, x));

                self.v[0xF] = if self.v[y as usize] > self.v[x] {
                    1
                } else {
                    0
                };

                self.v[x] = (self.v[y as usize] as u8).wrapping_sub(self.v[x]);
            }
            (0x8, x, _, 0xE) => {
                self.print_debug(format!("Shift the value of register V{} left one bit\n\tSet register VF to the most significant bit prior to the shift", x));

                self.v[0xF] = self.v[x] >> 7;
                self.v[x] <<= 1;
            }
            (0x9, x, y, 0x0) => {
                self.print_debug(format!("Skip the following instruction if V{} != V{}", x, y));

                if self.v[x] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                self.print_debug(format!("Store memory address {:#05X?} to register I", nnn));

                self.i = nnn;
            }
            (0xB, _, _, _) => {
                self.print_debug(format!("Jump to address {} + V0", nnn));

                self.pc = nnn + (self.v[0] as u16);
            }
            (0xC, x, _, _) => {
                self.print_debug(format!("Set V{} = random byte AND {:#04X?}", x, nn));

                let rnd = self.rng.gen::<u8>();
                self.v[x] = rnd & nn;
            }
            (0xD, x, y, n) => {
                let y_coord = self.v[y as usize];
                let x_coord = self.v[x] as usize;
                self.print_debug(format!("Draw sprite {:?} at x={} y={}", n, x_coord, y_coord));

                let start_pos = (64 * y_coord) + x_coord;
                let mut unset = false;

                for i in 0usize..(n as usize) {
                    let mut line_data = self.memory[self.i as usize + i];

                    for j in 0..8 {
                        let pixel_on = 0x8 == (0x8 & line_data);
                        line_data = line_data << 1;

                        let current_pos = start_pos + (64 * i) + j;
                        if pixel_on {
                            if self.output_buffer[current_pos] == 0 {
                                // self.output_buffer[current_pos] = from_u8_rgb(self.rng.gen::<u8>(), self.rng.gen::<u8>(), self.rng.gen::<u8>());
                                self.output_buffer[current_pos] = from_u8_rgb(200, 200, 100);
                            } else {
                                self.output_buffer[current_pos] = 0;
                                unset = true;
                            }
                        }
                    }
                }

                self.v[0xF] = if unset {
                    1
                } else {
                    0
                };

                self.display.update_buffer(&self.output_buffer);
            }
            (0xE, x, 0xA, 0x1) => {
                self.print_debug(format!("Skips the next instruction if key {} isn't pressed.", self.v[x]));

                if !self.keyboard.is_key_pressed(self.v[x]) {
                    self.pc += 2
                } else {
                    println!("Key {} is pressed", self.v[x])
                }
            }
            (0xF, x, 0x0, 0x7) => {
                self.print_debug(format!("Set V{} = delay timer", x));

                self.v[x] = self.delay_timer;
            }
            (0xF, x, 0x1, 0x5) => {
                self.print_debug(format!("Set delay timer = V{}", x));

                self.delay_timer = self.v[x];
            }
            (0xF, x, 0x1, 0x8) => {
                self.print_debug(format!("Set sound timer = V{}", x));

                self.sound_timer = self.v[x];
            }
            (0xF, x, 0x2, 0x9) => {
                self.print_debug(format!("Set I = location of sprite for digit V{}.", x));

                self.i = match self.v[x] {
                    0 => 0x00,
                    1 => 0x05,
                    2 => 0x0A,
                    3 => 0x0F,
                    4 => 0x14,
                    5 => 0x19,
                    6 => 0x1E,
                    7 => 0x23,
                    8 => 0x28,
                    9 => 0x2D,
                    0xA => 0x32,
                    0xB => 0x37,
                    0xC => 0x3C,
                    0xD => 0x41,
                    0xE => 0x46,
                    0xF => 0x4B,
                    _ => 0x50
                };
            }
            (0xF, x, 0x3, 0x3) => {
                self.print_debug(format!("Store BCD representation of V{} in memory locations {:#06X?}, {:#06X?}, and {:#06X?}.", x, self.i, self.i + 1, self.i + 2));

                let num = self.v[x];

                let hundreds = num / 100;
                let tens = (num % 100) / 10;
                let ones = num % 10;

                let start_address = self.i as usize;

                self.memory[start_address] = hundreds;
                self.memory[start_address + 1] = tens;
                self.memory[start_address + 2] = ones;
            }
            (0xF, x, 0x5, 0x5) => {
                self.print_debug(format!("Store registers V0 through V{} in memory starting at location {:#06X?}.", x, self.i));

                for reg in 0..=x {
                    self.memory[self.i as usize + reg] = self.v[reg];
                }
            }
            (0xF, x, 0x6, 0x5) => {
                self.print_debug(format!("Read registers V0 through V{} from memory starting at location {:#06X?}.", x, self.i));

                for reg in 0..=x {
                    self.v[reg] = self.memory[(self.i as usize) + reg];
                }
            }
            (_, _, _, _) => panic!("UNRECOGNIZED OP: {:#06X?}", opcode)
        }
    }

//     pub fn disassemble_op(&mut self, opcode: u16) {
//         let op1 = ((opcode & 0xF000) >> 12) as u8;
//         let op2 = ((opcode & 0x0F00) >> 8) as u8;
//         let op3 = ((opcode & 0x00F0) >> 4) as u8;
//         let op4 = (opcode & 0x000F) as u8;
//
//         let nnn = opcode & 0x0FFF;
//         let nn = (opcode & 0x00FF) as u8;
//
//         print!("{:#06X?}: ", opcode);
//
//         match (op1, op2, op3, op4) {
//             (0x0, 0x0, 0xE, 0x0) => println!("Clear screen"),
//             (0x0, 0x0, 0xE, 0xE) => println!("Return from a subroutine"),
//             (0x0, _, _, _) => println!("Execute machine language subroutine at address {:#05X?}", nnn),
//             (0x1, _, _, _) => println!("Jump to address {}", nnn),
//             (0x2, _, _, _) => println!("Execute subroutine at address {:#05X?}", nnn),
//             (0x3, x, _, _) => println!("Skip the following instruction if V{} == {}", x, nn),
//             (0x4, x, _, _) => println!("Skip the following instruction if V{} != {}", x, nn),
//             (0x5, x, y, 0) => println!("Skip the following instruction if V{} == V{}", x, y),
//             (0x6, x, _, _) => { println!("Store {} in register V{}", nn, x) }
//             (0x7, x, _, _) => println!("Add {} to register V{}", nn, x),
//             (0x8, x, y, 0x0) => println!("Store V{} in V{}", y, x),
//             (0x8, x, y, 0x1) => println!("Set V{} to V{} | V{}", x, x, y),
//             (0x8, x, y, 0x2) => println!("Set V{} to V{} & V{}", x, x, y),
//             (0x8, x, y, 0x3) => println!("Set V{} to V{} ^ V{}", x, x, y),
//             (0x8, x, y, 0x4) => println!("Add the value of register V{} to register V{}\n\tSet VF to 01 if a carry occurs\n\tSet VF to 00 if a carry does not occur", y, x),
//             (0x8, x, y, 0x5) => println!("Subtract the value of register V{} from register V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", y, x),
//             (0x8, x, y, 0x6) => println!("Store the value of register V{} shifted right one bit in register V{}\n\tSet register VF to the least significant bit prior to the shift", y, x),
//             (0x8, x, y, 0x7) => println!("Set register V{} to the value of V{} minus V{}\n\tSet VF to 00 if a borrow occurs\n\tSet VF to 01 if a borrow does not occur", x, y, x),
//             (0x8, x, y, 0xE) => println!("Store the value of register V{} shifted left one bit in register V{}\n\tSet register VF to the most significant bit prior to the shift", y, x),
//             (0x9, x, y, 0x0) => println!("Skip the following instruction if V{} != V{}", x, y),
//             (0xA, _, _, _) => println!("Store memory address {:#05X?} to register I", nnn),
//             (0xB, _, _, _) => println!("Jump to address {} + V0", nnn),
//             (0xC, x, _, _) => println!("Set V{} = random byte AND {:#04X?}", x, nn),
//             (0xD, x, y, n) => println!("Draw sprite {:?} at x={} y={}", n, x, y),
//             (0xF, x, 0x0, 0x7) => println!("Set V{} = delay timer", x),
//             (0xF, x, 0x1, 0x5) => println!("Set delay timer = V{}", x),
//             (0xF, x, 0x2, 0x9) => println!("Set I = location of sprite for digit V{}.", x),
//             (0xF, x, 0x3, 0x3) => println!("Store BCD representation of V{} in memory locations {:#06X?}, {:#06X?}, and {:#06X?}.", x, self.i, self.i + 1, self.i + 2),
//             (0xF, x, 0x6, 0x5) => println!("Read registers V0 through V{} from memory starting at location {:#06X?}.", x, self.i),
//             (_, _, _, _) => println!("UNRECOGNIZED OP: {:#06X?}", opcode)
//         }
//     }
}
