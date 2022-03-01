use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::time::{Duration, Instant};
use std::env;

mod cpu;
mod memory;
mod display_module;
mod timer_module;
mod keyboard_module;

use display_module::DisplayModule;
use memory::Memory;
use cpu::Cpu;
use timer_module::TimerModule;
use crate::keyboard_module::KeyboardModule;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const START_SIZE_MULTIPLIER: u32 = 8;

const TIMER_CLOCK_FREQ: u32 = 60;
const DEFAULT_CPU_CLOCK: u64 = 1000;




fn main() ->  Result<(), Error> {

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1]; //get filename form arguments

    let cycle_freq: u64;
    if args.len() == 3 {
        cycle_freq = args[2].parse().expect("Error with command-line arguments");
    } else {
        cycle_freq = DEFAULT_CPU_CLOCK;
    }
    println!("CPU Clock: {}Hz", cycle_freq);

    //Init event loop
    let event_loop = EventLoop::new();

    //init input helper
    let mut input = WinitInputHelper::new();

    //init display window
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let start_size = LogicalSize::new((WIDTH as f64) * (START_SIZE_MULTIPLIER as f64), (HEIGHT as f64) * (START_SIZE_MULTIPLIER as f64));
        WindowBuilder::new()
            .with_title("Chip8 Emulator")
            .with_inner_size(start_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    //init emulator components
    let mut display_module = DisplayModule::new(WIDTH, HEIGHT);
    let mut timer_module = TimerModule::new();
    let mut memory = Memory::new();
    let mut keyboard_module = KeyboardModule::new();
    let mut cpu = Cpu::new();
    memory.initialize(filename);

    //init clocks
    let cpu_clock_delay = Duration::from_nanos(1000000000 / cycle_freq);
    let timer_clock_delay = Duration::from_nanos(1000000000 / TIMER_CLOCK_FREQ as u64); //should run at 60Hz

    let mut last_cpu_clock = Instant::checked_sub(&(Instant::now()), cpu_clock_delay).expect("timing initialization error. (cpu clock)");
    let mut last_timer_clock = Instant::checked_sub(&(Instant::now()), timer_clock_delay).expect("timing initialization error. (timer clock)");

    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        let now = Instant::now();

        if now.duration_since(last_cpu_clock) > cpu_clock_delay {
            last_cpu_clock = now;
            //execute cpu instruction
            cpu.execute_instruction(&mut memory, &mut display_module, &mut timer_module, &mut keyboard_module);
        }

        if now.duration_since(last_timer_clock) > timer_clock_delay {
            last_timer_clock = now;
            window.request_redraw();
            //execute timer
            timer_module.update();
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            display_module.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {:?}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }


        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            keyboard_module.check_keys(&input, &mut cpu);

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
        }

    });
}


