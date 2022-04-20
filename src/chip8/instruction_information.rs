pub struct InstructionInformation {
    pub instruction_nibble: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: usize,
    pub key_pressed: Vec<u8>
}

impl InstructionInformation {
    pub fn get_nibbles(&self) -> (u8, u8, u8, u8) {
        (self.instruction_nibble, self.x, self.y, self.n)
    }

    pub fn print(&self) {
        println!("INibble: {}, x: {}, y:{}, n:{}", self.instruction_nibble, self.x, self.y, self.n);
    }
}