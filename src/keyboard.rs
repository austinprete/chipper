use minifb::{Window, KeyRepeat, Key};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Keyboard {
    pub(crate) window: Rc<RefCell<Window>>,
    keys: [bool; 16]
}

impl Keyboard {
    pub fn new(window: Rc<RefCell<Window>>) -> Keyboard {
        Keyboard {
            window,
            keys: [false; 16]
        }
    }

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn is_key_pressed(&mut self, key: u8) -> bool {
        return self.keys[key as usize];
    }

    pub fn reset_keys(&mut self) { self.keys = [false; 16]}

    /*
    Keypad Layout:

    1	2	3	C
    4	5	6	D
    7	8	9	E
    A	0	B	F

     */
    pub fn get_input(&mut self, window: Rc<RefCell<Window>>) {
        self.reset_keys();
        window.borrow_mut().get_keys_pressed(KeyRepeat::Yes).map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 => self.key_down(1),
                    Key::Key2 => self.key_down(2),
                    Key::Key3 => self.key_down(3),
                    Key::Key4 => self.key_down(0xC),
                    Key::Q => self.key_down(4),
                    Key::W => self.key_down(5),
                    Key::E => self.key_down(6),
                    Key::R => self.key_down(0xD),
                    Key::A => self.key_down(7),
                    Key::S => self.key_down(8),
                    Key::D => self.key_down(9),
                    Key::F => self.key_down(0xE),
                    Key::Z => self.key_down(0xA),
                    Key::X => self.key_down(0),
                    Key::C => self.key_down(0xB),
                    Key::V => self.key_down(0xF),
                    _ => (),
                }
            }
        });
    }
}
