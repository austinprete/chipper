use minifb::{Window, KeyRepeat, Key};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Keyboard {
    pub(crate) window: Rc<RefCell<Window>>,
}

impl Keyboard {
    pub fn new(window: Rc<RefCell<Window>>) -> Keyboard {
        Keyboard {
            window,
        }
    }

    /*
    Keypad Layout:

    1	2	3	C
    4	5	6	D
    7	8	9	E
    A	0	B	F

     */
    pub fn is_key_pressed(&self, key_num: u8) -> bool {
        let key = match key_num {
            1 => Key::Key1,
            2 => Key::Key2,
            3 => Key::Key3,
            0xC => Key::Key4,
            4 => Key::Q,
            5 => Key::W,
            6 => Key::E,
            0xD => Key::R,
            7 => Key::A,
            8 => Key::S,
            9 => Key::D,
            0xE => Key::F,
            0xA => Key::Z,
            0x0 => Key::X,
            0xB => Key::C,
            0xF => Key::V,
            _ => Key::Space,
        };
        self.window.borrow().is_key_pressed(key, KeyRepeat::Yes)
    }
}
