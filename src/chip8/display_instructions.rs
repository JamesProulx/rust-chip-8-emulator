use crate::Chip8;
use crate::chip8::InstructionInformation;

use super::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use super::execution_information::ExecutionInformation;

pub fn clear_screen_00e0(chip: &mut Chip8, _instruction_information: InstructionInformation) -> ExecutionInformation {
    
    chip.display.clear_screen();
    ExecutionInformation {
        new_pc: None,
        need_to_render: true
    }
}

pub fn draw_dxyn(chip: &mut Chip8, instruction_information: InstructionInformation) -> ExecutionInformation {
    chip.registers[0xF] = 0x0;
    for row in 0u16..instruction_information.n as u16 {
        let y_coord = ((chip.registers[(instruction_information.y) as usize] as u16 + row) % DISPLAY_HEIGHT as u16) as u8;

        for bit in 0u16..7u16 {
            let x_coord = (chip.registers[(instruction_information.x as usize) as usize] + bit as u8) % DISPLAY_WIDTH as u8;
            let color = (chip.memory[(chip.index + row) as usize] >> (7 - bit)) & 1;
            chip.registers[0xF] |= color & chip.display.get_value_at_coord(x_coord.into(), y_coord.into());
            chip.draw_pixel(x_coord.into(), y_coord.into(), color);
        }
    }

    ExecutionInformation {
        need_to_render: true,
        new_pc: None
    }
}