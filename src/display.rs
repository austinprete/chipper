use minifb::Window;
use std::rc::Rc;
use std::cell::RefCell;
use crate::SCALE_FACTOR;

pub struct Display {
    pub(crate) window: Rc<RefCell<Window>>,
    height: usize,
    width: usize,
    buffer: Vec<u32>,
}

impl Display {
    pub fn new(window: Rc<RefCell<Window>>) -> Display {
        let (width, height) = (*window).borrow().get_size();
        Display {
            window,
            height,
            width,
            buffer: vec![0; width * height],
        }
    }

    pub fn update_buffer(&mut self, buf: &[u32; 2048]) {
        for (i, val) in buf.iter().enumerate() {
            let y = i / (self.width / SCALE_FACTOR);
            let x = i - (self.width / SCALE_FACTOR * y);
            for x_coord in (x * SCALE_FACTOR)..((x * SCALE_FACTOR) + SCALE_FACTOR) {
                for y_coord in (y * SCALE_FACTOR)..((y * SCALE_FACTOR) + SCALE_FACTOR) {
                    self.buffer[(y_coord * self.width) + x_coord] = *val;
                }
            }
        }

        (*self.window).borrow_mut().update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
