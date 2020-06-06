use std::io;
use std::io::prelude::*;
use std::fs::File;

mod cpu;

use cpu::CPU;

mod rom;

use rom::ROM;

fn main() -> io::Result<()> {
    let mut cpu = CPU::new();

    let mut rom = ROM::new("programs/PONG");

    cpu.load_rom(rom);

    for i in 256..256 + (rom.size / 2) {
        println!("{:?}", i);
        let opcode = ((cpu.memory[i * 2] as u16) << 8) | cpu.memory[(i * 2) + 1] as u16;
        // println!("{:#06x?}", opcode);
        cpu.execute_op(opcode);
    }


    Ok(())
}
