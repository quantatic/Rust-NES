use sdl2::keyboard::{Keycode, Scancode};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Controller {
	input: Rc<RefCell<sdl2::EventPump>>,
	strobe: bool, // strobe tracks whether this controller is "latched" or not
	state: Vec<bool>,
}

impl Controller {
	pub fn new(input: Rc<RefCell<sdl2::EventPump>>) -> Self {
		Self {
			input,
			strobe: false,
			state: Vec::new(),
		}
	}

	// Standard controller reports values as follows:
	// 0 - A
	// 1 - B
	// 2 - Select
	// 3 - Start
	// 4 - Up
	// 5 - Down
	// 6 - Left
	// 7 - Right
	pub fn set_strobe(&mut self, strobe: bool) {
		// If this isn't actually a change, do nothing
		if self.strobe == strobe {
			return;
		}

		// If setting strobe to true, just keep track of that. If setting to false, we need to
		// actually latch the values of each pressed key.
		match strobe {
			true => self.strobe = true,
			false => {
				self.strobe = false;
				self.state =
					/*        A           B           SELECT            START           UP           DOWN           LEFT           RIGHT    */ 
					[Keycode::Z, Keycode::X, Keycode::RShift, Keycode::Return, Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right]
						.iter()
						.map(|&keycode| {
							self.input.borrow()
								.keyboard_state()
								.is_scancode_pressed(Scancode::from_keycode(keycode).unwrap())
						})
						.rev()
						.collect::<Vec<_>>();
			}
		}
	}

	pub fn read(&mut self) -> bool {
		match self.strobe {
			true => self.input.borrow()
						.keyboard_state()
						.is_scancode_pressed(Scancode::from_keycode(Keycode::A).unwrap()),
			false => {
				self.state.pop().unwrap_or(false)
			}
		}
	}
}
