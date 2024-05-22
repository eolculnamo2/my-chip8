// all characters are 5 in length from start

pub type FontArray = [FontMap; 16];
pub struct FontMap {
    pub character: char,
    pub memory_start: i16,
}

pub fn get_fonts_by_char(c: char) -> [u8; 5] {
    match c {
        '0' => [0xF0, 0x90, 0x90, 0x90, 0xF0],
        '1' => [0x20, 0x60, 0x20, 0x20, 0x70],
        '2' => [0xF0, 0x10, 0xF0, 0x80, 0xF0],
        '3' => [0xF0, 0x10, 0xF0, 0x10, 0xF0],
        '4' => [0x90, 0x90, 0xF0, 0x10, 0x10],
        '5' => [0xF0, 0x80, 0xF0, 0x10, 0xF0],
        '6' => [0xF0, 0x80, 0xF0, 0x90, 0xF0],
        '7' => [0xF0, 0x10, 0x20, 0x40, 0x40],
        '8' => [0xF0, 0x90, 0xF0, 0x90, 0xF0],
        '9' => [0xF0, 0x90, 0xF0, 0x10, 0xF0],
        'A' => [0xF0, 0x90, 0xF0, 0x90, 0x90],
        'B' => [0xE0, 0x90, 0xE0, 0x90, 0xE0],
        'C' => [0xF0, 0x80, 0x80, 0x80, 0xF0],
        'D' => [0xE0, 0x90, 0x90, 0x90, 0xE0],
        'E' => [0xF0, 0x80, 0xF0, 0x80, 0xF0],
        'F' => [0xF0, 0x80, 0xF0, 0x80, 0x80],
        _ => panic!("Invalid character"),
    }
}

pub fn init_fonts() -> FontArray {
    [
        FontMap {
            character: '0',
            memory_start: 0x050,
        },
        FontMap {
            character: '1',
            memory_start: 0x055,
        },
        FontMap {
            character: '2',
            memory_start: 0x05A,
        },
        FontMap {
            character: '3',
            memory_start: 0x05F,
        },
        FontMap {
            character: '4',
            memory_start: 0x064,
        },
        FontMap {
            character: '5',
            memory_start: 0x069,
        },
        FontMap {
            character: '6',
            memory_start: 0x06E,
        },
        FontMap {
            character: '7',
            memory_start: 0x073,
        },
        FontMap {
            character: '8',
            memory_start: 0x078,
        },
        FontMap {
            character: '9',
            memory_start: 0x07D,
        },
        FontMap {
            character: 'A',
            memory_start: 0x082,
        },
        FontMap {
            character: 'B',
            memory_start: 0x087,
        },
        FontMap {
            character: 'C',
            memory_start: 0x08C,
        },
        FontMap {
            character: 'D',
            memory_start: 0x091,
        },
        FontMap {
            character: 'E',
            memory_start: 0x096,
        },
        FontMap {
            character: 'F',
            memory_start: 0x09B,
        },
    ]
}
