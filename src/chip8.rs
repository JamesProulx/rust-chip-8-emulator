pub mod display;
pub mod instruction_information;
pub mod execution_information;

mod display_instructions;
mod system_instructions;
mod font;

use display::Display;
use instruction_information::InstructionInformation;
use execution_information::ExecutionInformation;

use std::fs::File;
use std::io::prelude::*;

use self::display_instructions::*;
use self::system_instructions::*;
use self::font::FONTS_MEMORY;

const BUFFER_SIZE_BYTES:u16 = 0xE00;
const PROGRAM_START_POSITION:u16 = 0x200;
const PROGRAM_START_POSITION_USIZE:usize = 0x200;
const MEMORY_SIZE_USIZE:usize = 0x1000;
const REGISTER_COUNT:usize = 16;

pub struct Chip8 {
    index: u16,
    stack: Vec<usize>,
    pub delay_timer: u8,
    sound_timer: u8,
    registers: [u8; REGISTER_COUNT],
    memory: [u8;MEMORY_SIZE_USIZE],
    pub display: Display,
    pub pc: usize
}

impl Chip8 {
    pub fn new() -> Self {

        let memory = get_and_populate_memory();

        Self {
            index: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; REGISTER_COUNT],
            memory,
            display: Display::new(),
            pc: PROGRAM_START_POSITION as usize
        }
    }

    pub fn fetch_instruction(&self) -> InstructionInformation {
        //0x4e12 would come in as [12,4e]
        let first_bit:u8 = self.memory[self.pc + 0]; //12
        let second_bit:u8 = self.memory[self.pc + 1]; //4e

        //Where an instruction is of the form
        // 4                      e   1   2
        // [instruction_nibble]  [x] [y] [n]
        // [instruction_nibble]  [x] [  nn ]
        // [instruction_nibble]  [   nnn   ]
        let instruction_nibble:u8 = (first_bit & 0xF0) >> 4;
        let x:u8 = first_bit & 0x0F;
        let y:u8 = (second_bit & 0xF0) >> 4;
        let n:u8 = second_bit & 0x0F;
        let nn:u8 = second_bit;
        let nnn:usize = second_bit as usize | ((x as usize) << 8);

        
        InstructionInformation {
            instruction_nibble,
            x,
            y,
            n,
            nn,
            nnn,
            key_pressed: Vec::new()
        }
    }

    pub fn decode_instruction(&self, instruction_information: &InstructionInformation) ->
        fn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {

        match instruction_information.get_nibbles() {
            (0x0,0x0, 0xE, 0x0)     => clear_screen_00e0,
            (0x0, 0x0, 0xE, 0xE)    => return_00ee,
            (0x1, _, _, _)          => jump_1nnn,
            (0x2, _, _, _)          => call_subroutine_2nnn,
            (0x3, _, _, _)          => skip_does_equal_3xnn,
            (0x4, _, _, _)          => skip_not_equal_4xnn,
            (0x5, _, _, 0x0)        => skip_registers_equal_5xy0,
            (0x6, _, _, _)          => set_register_6xnn,
            (0x7, _, _, _)          => add_value_to_register_7xnn,
            (0x8, _, _, 0x0)        => set_x_to_y_8xy0,
            (0x8, _, _, 0x1)        => binary_or_8xy1,
            (0x8, _, _, 0x2)        => binary_and_8xy2,
            (0x8, _, _, 0x3)        => binary_xor_8xy3,
            (0x8, _, _, 0x4)        => add_with_carry_8xy4,
            (0x8, _, _, 0x5)        => subtract_vy_from_vx_8xy5,
            (0x8, _, _, 0x6)        => shift_right_8xy6,
            (0x8, _, _, 0x7)        => subtract_vx_from_vy_8xy7,
            (0x8, _, _, 0xE)        => shift_left_8xye,
            (0x9, _, _, 0x0)        => skip_registers_not_equal_9xy0,
            (0xA, _, _, _)          => set_index_register_annn,
            (0xC, _, _, _)          => random_cxnn,
            (0xD, _, _, _)          => draw_dxyn,
            (0xE, _, 0x9, 0xE)      => skip_if_pressed_ex9e,
            (0xE, _, 0xA, 0x1)      => skip_if_not_pressed_exa1,
            (0xF, _, 0x0, 0x7)      => set_vx_to_delay_timer_fx07,
            (0xF, _, 0x1, 0x5)      => delay_timer_to_vx_fx15,
            (0xF, _, 0x1, 0x8)      => sound_timer_to_vs_fx18,
            (0xF, _, 0x2, 0x9)      => font_fx29,
            (0xF, _, 0x3, 0x3)      => binary_coded_decimal_conversion_fx33,
            (0xF, _, 0x5, 0x5)      => write_to_memory_fx55,
            (0xF, _, 0x6, 0x5)      => read_from_memory_fx65,
            _ => { 
                println!("Unexpected instruction with: {:?}", instruction_information.get_nibbles());
                panic!("ahh")
            }
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y:usize, color:u8) {
        self.display.set_value_at_coord(x, y, color)
    }

    pub fn load_program_into_memory(&mut self, file_name: &str) -> Result<usize, Box<dyn std::error::Error>> {
        
        let mut file = File::open(file_name)?;
        let mut input_buffer = [0u8; BUFFER_SIZE_BYTES as usize];

        let bytes_read = if let Ok(bytes_read) = file.read(&mut input_buffer) {
            bytes_read
        } else {
            0
        };

        for (i, &byte) in input_buffer.iter().enumerate() {
            if PROGRAM_START_POSITION_USIZE + i >= 0x1000 {
                break;
            }
            self.memory[PROGRAM_START_POSITION_USIZE + i] = byte;
            
        }

        return Ok(bytes_read);
    }
}

fn get_and_populate_memory() -> [u8;MEMORY_SIZE_USIZE] {
    let mut memory = [0u8;MEMORY_SIZE_USIZE];
    for i in 0usize..(FONTS_MEMORY.len()) {
        memory[i] = FONTS_MEMORY[i];
    };
    memory
}