mod bus;
mod controller;
mod cpu;
mod ppu;
mod rom;

use crate::bus::Bus;
use crate::controller::Controller;
use crate::cpu::{Cpu, Interrupt};
use crate::ppu::Ppu;
use crate::rom::Rom;

use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let rom = Rom::new("roms/galaga.nes")?;

    let sdl_context = sdl2::init()?;

    let sdl_video_subsystem = sdl_context.video()?;

    let sdl_events = Rc::new(RefCell::new(sdl_context.event_pump()?));

    let ppu = Ppu::new(&sdl_video_subsystem);

    let controller = Controller::new(Rc::clone(&sdl_events));

    let bus = Bus::new(rom, ppu, controller);
    let mut cpu = Cpu::new(bus);

    cpu.interrupt(Interrupt::Reset);

    let mut master_clock_ticks: u64 = 0;
    let mut last_frame_start = 0;

    let time_per_cpu_cycle = Duration::new(1, 0) / (236_250_000 / 11 / 12);
    let mut cpu_cycle_start_time = Instant::now();
    let start_time = Instant::now();

    // Master clocks run at 21.477272 MHz
    loop {
        // CPU runs every 12 master ticks ;)
        if master_clock_ticks % 12 == 0 {
            cpu.step();
            while cpu_cycle_start_time.elapsed() < time_per_cpu_cycle {} // spinlock :/

            cpu_cycle_start_time = Instant::now();
        }

        // PPU runs every 4 master ticks
        if master_clock_ticks % 4 == 0 {
            let new_frame = cpu.bus.ppu.step();
            if new_frame {
                let current_steps = master_clock_ticks / 4;
                let _last_frame_steps = current_steps - last_frame_start;
                last_frame_start = current_steps;
            }

            if cpu.bus.ppu.scanline == 0 && cpu.bus.ppu.cycle == 0 {
                for i in 0x00..=0x0F {
                    let addr = 0x3F00 + i;
                }
            }
        }

        if master_clock_ticks % 10000 == 0 {
            for event in sdl_events.borrow_mut().poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    panic!("Exiting!");
                }
            }
        }

        master_clock_ticks += 1;
    }
}
