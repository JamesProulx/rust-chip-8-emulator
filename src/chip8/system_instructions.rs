use rand::prelude::*;

use super::{
    Chip8, 
    instruction_information::InstructionInformation, 
    execution_information::ExecutionInformation
};

pub fn return_00ee(chip: &mut Chip8, _instruction_information: InstructionInformation) -> ExecutionInformation {
    ExecutionInformation {
        need_to_render: false,
        new_pc: Some(chip.stack.pop().unwrap())
    }
}

pub fn jump_1nnn(_chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    ExecutionInformation {
        need_to_render: false,
        new_pc: Some(instruction_information.nnn)
    }
}

pub fn call_subroutine_2nnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.stack.push(chip.pc.to_owned());
    ExecutionInformation {
        need_to_render: false,
        new_pc: Some(instruction_information.nnn)
    }
}

pub fn skip_does_equal_3xnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];

    let new_pc: Option<usize> = if vx == instruction_information.nn {
        Some(chip.pc + 2) //skip next instruction
    } else {
        None
    };

    ExecutionInformation {
        new_pc,
        need_to_render: false,
    }
}

pub fn skip_not_equal_4xnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];

    let new_pc: Option<usize> = if vx == instruction_information.nn {
        None
    } else {
        Some(chip.pc + 2) //skip next instruction
    };

    ExecutionInformation {
        new_pc,
        need_to_render: false,
    }
}

pub fn skip_registers_equal_5xy0(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    let new_pc: Option<usize> = if vx == vy {
        Some(chip.pc + 2)
    } else {
        None
    };
    
    ExecutionInformation {
        new_pc,
        need_to_render: false
    }
}

pub fn set_register_6xnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.registers[instruction_information.x as usize] = instruction_information.nn;
    ExecutionInformation::default()
}

pub fn add_value_to_register_7xnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    
    let vx = chip.registers[instruction_information.x as usize] as u16;
    let nn = instruction_information.nn as u16;

    let new_value = vx + nn;

    chip.registers[instruction_information.x as usize] = new_value as u8;

    ExecutionInformation::default()
}

pub fn set_x_to_y_8xy0(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.registers[instruction_information.x as usize] = chip.registers[instruction_information.y as usize];
    ExecutionInformation::default()
}

pub fn binary_or_8xy1(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    chip.registers[instruction_information.x as usize] = vx | vy;
    ExecutionInformation::default()
}

pub fn binary_and_8xy2(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    chip.registers[instruction_information.x as usize] = vx & vy;
    ExecutionInformation::default()
}

pub fn binary_xor_8xy3(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    chip.registers[instruction_information.x as usize] = vx ^ vy;
    ExecutionInformation::default()
}

pub fn add_with_carry_8xy4(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize] as u16;
    let vy = chip.registers[instruction_information.y as usize] as u16;

    let result = vx + vy;

    chip.registers[instruction_information.x as usize] = result as u8;
    chip.registers[0xF] = if result > u8::MAX as u16 { 1 } else { 0 };
    ExecutionInformation::default()
}

pub fn subtract_vy_from_vx_8xy5(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    chip.registers[instruction_information.x as usize] = vx.wrapping_sub(vy);

    chip.registers[0xF] = if vx > vy { 1 } else { 0 };
    ExecutionInformation::default()
}

pub fn shift_right_8xy6(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];

    chip.registers[0xF] = vx & 1;
    chip.registers[instruction_information.x as usize] >>= 1;

    ExecutionInformation::default()
}

pub fn subtract_vx_from_vy_8xy7(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    chip.registers[instruction_information.x as usize] = vy.wrapping_sub(vx);

    chip.registers[0xF] = if vx > vy { 1 } else { 0 };
    ExecutionInformation::default()
}

pub fn shift_left_8xye(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    chip.registers[0xF] = if vx & 0b10000000 == 0 {
        0
    } else {
        1
    };

    chip.registers[instruction_information.x as usize] <<= 1;
    ExecutionInformation::default()
}

pub fn skip_registers_not_equal_9xy0(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    let vy = chip.registers[instruction_information.y as usize];

    let new_pc: Option<usize> = if vx == vy {
        None
    } else {
        Some(chip.pc + 2)
    };
    
    ExecutionInformation {
        new_pc,
        need_to_render: false
    }
}

pub fn set_index_register_annn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.index = instruction_information.nnn as u16;
    ExecutionInformation::default()
}

pub fn random_cxnn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let mut rng = rand::thread_rng();
    chip.registers[instruction_information.x as usize] = rng.gen::<u8>() & instruction_information.nn;
    ExecutionInformation::default()
}

pub fn skip_if_pressed_ex9e(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let relevant_key:u8 = chip.registers[instruction_information.x as usize];
    
    if instruction_information.key_pressed.contains(&relevant_key) {
        ExecutionInformation {
            need_to_render: false,
            new_pc: Some(chip.pc + 2)
        }
    } else {
        ExecutionInformation::default()
    }
}

pub fn skip_if_not_pressed_exa1(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let relevant_key:u8 = chip.registers[instruction_information.x as usize];
    
    if !instruction_information.key_pressed.contains(&relevant_key) {
        ExecutionInformation {
            need_to_render: false,
            new_pc: Some(chip.pc + 2)
        }
    } else {
        ExecutionInformation::default()
    }
}

pub fn set_vx_to_delay_timer_fx07(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.registers[instruction_information.x as usize] = chip.delay_timer;
    ExecutionInformation::default()
}

pub fn delay_timer_to_vx_fx15(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.delay_timer = chip.registers[instruction_information.x as usize];
    ExecutionInformation::default()
}

pub fn sound_timer_to_vs_fx18(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.sound_timer = chip.registers[instruction_information.x as usize];
    ExecutionInformation::default()
}

pub fn font_fx29(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let hex_char_to_point_at:u8 = chip.registers[instruction_information.x as usize];

    chip.index = crate::chip8::font::FONT_HEX_ADDRESSES[(hex_char_to_point_at % 16) as usize];
    ExecutionInformation::default()
}

pub fn binary_coded_decimal_conversion_fx33(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    let vx = chip.registers[instruction_information.x as usize];
    
    chip.memory[chip.index as usize] = vx / 100;
    chip.memory[chip.index as usize + 1usize] = (vx % 100) / 10;
    chip.memory[chip.index as usize + 2usize] = vx % 10;

    ExecutionInformation::default()
}

pub fn write_to_memory_fx55(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    for register_index in 0u8..=instruction_information.x {
        chip.memory[(chip.index + register_index as u16) as usize] = chip.registers[register_index as usize];
    }

    ExecutionInformation::default()
}

pub fn read_from_memory_fx65(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    for register_index in 0u8..=instruction_information.x {
        chip.registers[register_index as usize] = chip.memory[(chip.index + register_index as u16) as usize]
    }

    ExecutionInformation::default()
}