pub const FONTS_MEMORY:[u8;80] = [
    /* 0x00 */ 0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    /* 0x05 */ 0x20, 0x60, 0x20, 0x20, 0x70, // 1
    /* 0x0A */ 0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    /* 0x0F */ 0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    /* 0x14 */ 0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    /* 0x19 */ 0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    /* 0x1E */ 0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    /* 0x23 */ 0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    /* 0x28 */ 0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    /* 0x2D */ 0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    /* 0x32 */ 0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    /* 0x37 */ 0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    /* 0x3C */ 0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    /* 0x41 */ 0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    /* 0x46 */ 0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    /* 0x4B */ 0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

//Simply the addresses in the fonts array which represent the start of the corresponding character, in hexadecimal.
pub const FONT_HEX_ADDRESSES:[u16;16] = [0x00, 0x05, 0x0A, 0x0F, 0x14, 0x19, 0x1E, 0x23, 0x28, 0x2D, 0x32, 0x37, 0x3C, 0x41, 0x46, 0x4B];