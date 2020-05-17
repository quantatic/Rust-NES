mod controller;
mod cpu;
mod bus;
mod ppu;
mod rom;

use crate::controller::Controller;
use crate::cpu::{Cpu, Interrupt};
use crate::bus::Bus;
use crate::ppu::Ppu;
use crate::rom::Rom;

use std::{thread, time};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	//let rom = Rom::new("roms/nestest.nes").unwrap();
	let rom = Rom::new("roms/donkey.nes").unwrap();
    //let rom = Rom::new("roms/full_nes_palette.nes").unwrap();
	//let rom = Rom::new("roms/color_test.nes").unwrap();
    //let rom = Rom::new("roms/mario.nes").unwrap();
    //let rom = Rom::new("roms/balloon.nes").unwrap();
    //let rom = Rom::new("roms/color_test.nes").unwrap();
    //let rom = Rom::new("roms/nestest.nes").unwrap();
    //let rom = Rom::new("roms/mario.nes").unwrap();
	//let rom = Rom::new("roms/scanline.nes").unwrap();
	//let rom = Rom::new("/home/quantatic/nes-test-roms/blargg_nes_cpu_test5/cpu.nes").unwrap();
	//let rom = Rom::new("roms/nestest.nes").unwrap();

    let sdl_context = sdl2::init()
        .unwrap();

	let sdl_video_subsystem = sdl_context
		.video()
		.unwrap();

	let sdl_events = Rc::new(RefCell::new(sdl_context.event_pump().unwrap()));

    let ppu = Ppu::new(&sdl_video_subsystem);

	let controller = Controller::new(Rc::clone(&sdl_events));

    let bus = Bus::new(rom, ppu, controller);
    let mut cpu = Cpu::new(bus);

    cpu.interrupt(Interrupt::Reset);

    let mut master_clock_ticks: u64 = 0;

    // Master clocks run at 21.477272 MHz
    loop {
        // CPU runs every 12 master ticks
        if master_clock_ticks % 12 == 0 {
            cpu.step();
        }

        // PPU runs every 4 master ticks
        if master_clock_ticks % 4 == 0 {
            cpu.bus.ppu.step();
        }

		if master_clock_ticks % 10000 == 0 { 
			for event in sdl_events.borrow_mut().poll_iter() {
				if let sdl2::event::Event::Quit{ .. } = event {
					panic!("Exiting!");
				}
			}
		}

        master_clock_ticks += 1;
    }
}
