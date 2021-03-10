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

    pub fn get_input(&mut self) {
        self.window.borrow_mut().get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::W => println!("pressed w"),
                    Key::T => println!("pressed t"),
                    _ => (),
                }
            }
        });
    }
}
