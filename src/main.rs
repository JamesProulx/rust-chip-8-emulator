pub mod chip8;

use chip8::Chip8;

use crate::chip8::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::chip8::instruction_information::InstructionInformation;
use crate::chip8::execution_information::ExecutionInformation;

use std::io::prelude::*;
use std::io;
use std::{thread, time};

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const PROGRAM:&str = "data/pong2.rom";

fn main() -> Result<(), Error> {
    println!("Iniatializing CHIP-8...");

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DISPLAY_HEIGHT as f64, DISPLAY_WIDTH as f64);
        WindowBuilder::new()
            .with_title("Chip8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32, surface_texture)?
    };

    let mut chip:Chip8 = Chip8::new();
    match chip.load_program_into_memory(PROGRAM) {
        Ok(bytes_read) => println!("Loaded {} bytes of instructions into memory", bytes_read),
        Err(error) => panic!("Could not load program with error: {}", error)
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
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

            let mut keys_pressed:Vec<u8> = vec![];
            if input.key_pressed(VirtualKeyCode::Key1)  { keys_pressed.push(0x1u8) }
            if input.key_pressed(VirtualKeyCode::Key2)  { keys_pressed.push(0x2u8) }
            if input.key_pressed(VirtualKeyCode::Key3)  { keys_pressed.push(0x3u8) }
            if input.key_pressed(VirtualKeyCode::Key4)  { keys_pressed.push(0xCu8) }
            if input.key_pressed(VirtualKeyCode::Q)     { keys_pressed.push(0x4u8) }
            if input.key_pressed(VirtualKeyCode::W)     { keys_pressed.push(0x5u8) }
            if input.key_pressed(VirtualKeyCode::E)     { keys_pressed.push(0x6u8) }
            if input.key_pressed(VirtualKeyCode::R)     { keys_pressed.push(0xDu8) }
            if input.key_pressed(VirtualKeyCode::A)     { keys_pressed.push(0x7u8) }
            if input.key_pressed(VirtualKeyCode::S)     { keys_pressed.push(0x8u8) }
            if input.key_pressed(VirtualKeyCode::D)     { keys_pressed.push(0x9u8) }
            if input.key_pressed(VirtualKeyCode::F)     { keys_pressed.push(0xEu8) }
            if input.key_pressed(VirtualKeyCode::Z)     { keys_pressed.push(0xAu8) }
            if input.key_pressed(VirtualKeyCode::X)     { keys_pressed.push(0x0u8) }
            if input.key_pressed(VirtualKeyCode::C)     { keys_pressed.push(0xBu8) }
            if input.key_pressed(VirtualKeyCode::V)     { keys_pressed.push(0xFu8) }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            
            tick(&mut chip, &mut pixels, keys_pressed);
            thread::sleep(time::Duration::from_millis(3));
            window.request_redraw();
        }
    });

    
}

fn tick(chip: &mut Chip8, pixels: &mut Pixels, keys_pressed: Vec<u8>) -> bool {
    let mut instruction_info:InstructionInformation = chip.fetch_instruction();
        chip.pc += 2; //one instruction is 2 bytes

        let prepared_instruction:fn(&mut Chip8, InstructionInformation) -> ExecutionInformation
            = chip.decode_instruction(&instruction_info);

        instruction_info.key_pressed = keys_pressed;

        let instruction_execution_result: ExecutionInformation = prepared_instruction(chip, instruction_info);

        match instruction_execution_result.new_pc {
            Some(new_pc) => chip.pc = new_pc,
            None => ()
        }

        if chip.delay_timer > 0 {
            chip.delay_timer -= 1;
        }

        if instruction_execution_result.need_to_render {
            chip.display.draw(pixels);
        }

        instruction_execution_result.need_to_render
}

fn _pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to step...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}