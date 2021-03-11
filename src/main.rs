use std::cell::RefCell;
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

const WIDTH: usize = 64 * SCALE_FACTOR;
const HEIGHT: usize = 32 * SCALE_FACTOR;
const SCALE_FACTOR: usize = 10;


fn main() {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
    // window.limit_update_rate(Some(std::time::Duration::from_micros(100)));
    window.set_key_repeat_delay(0.01);
    window.set_key_repeat_rate(0.01);

    let window_ref = Rc::new(RefCell::new(window));
    let display = Display::new(window_ref.clone());
    let keyboard = Keyboard::new(window_ref.clone());
    let mut cpu = CPU::new(keyboard, display);

    let rom = ROM::new("programs/PONG");

    cpu.load_rom(rom);

    // cpu.enable_debug();
    cpu.run();
}
