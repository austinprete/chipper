use std::cell::RefCell;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;

use minifb::{Window, WindowOptions};

use cpu::CPU;
use display::Display;
use keyboard::Keyboard;
use rom::ROM;

mod cpu;
mod rom;
mod keyboard;
mod display;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SCALE_FACTOR: usize = 10;


fn main() {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let window_ref = Rc::new(RefCell::new(window));
    let display = Display::new(window_ref.clone());
    let keyboard = Keyboard::new(window_ref.clone());
    let mut cpu = CPU::new(keyboard, display);

    let rom = ROM::new("programs/PONG");

    cpu.load_rom(rom);

    // for i in 256..256 + (rom.size / 2) {
    //     println!("{:?}", i);
    //     // let opcode = ((cpu.memory[i * 2] as u16) << 8) | cpu.memory[(i * 2) + 1] as u16;
    //     // println!("{:#06x?}", opcode);
    //     // cpu.execute_op(opcode);
    // }

    cpu.enable_debug();
    cpu.run();
}
