use crate::fonts::{get_fonts_by_char, init_fonts, FontArray};
use crate::read_instructions::{create_nibble_pairs, read_instruction};

type MemoryArray = [u8; 4096];

// 64 wide, 32  tall
type DisplayArray = [u8; 64 * 32];

pub struct System {
    pub is_loaded: bool,
    pub fonts: FontArray,
    pub memory: MemoryArray,
    pub display: DisplayArray,
    pub program_counter: u8,
    pub index: u16,
    pub stack: Vec<u8>,
}

const START_MEMORY_INDEX: usize = 0x200;
impl System {
    fn init_memory(fonts: &FontArray) -> MemoryArray {
        let mut memory = [0; 4096];
        fonts.iter().for_each(|f| {
            let font_defs = get_fonts_by_char(f.character);
            for (i, font_byte) in font_defs.iter().enumerate() {
                let idx = f.memory_start as usize + i;
                memory[idx] = *font_byte;
            }
        });
        memory
    }

    pub fn load_program(&mut self, rom_data: Vec<u8>) {
        if self.is_loaded {
            todo!("Todo clear out old program");
        }

        let pairs = create_nibble_pairs(rom_data);
        pairs.iter().for_each(|pair| {
            read_instruction(pair);
        });

        // let mut index = START_MEMORY_INDEX;
        // rom_data.iter().for_each(|item| {
        //     read_instruction(item);
        //     *self
        //         .memory
        //         .get_mut(index)
        //         .expect("Program too large. Out of memory!") = *item;
        //     index += 1;
        // });
        //
        // let program_size = index - START_MEMORY_INDEX;
        // assert!(
        //     rom_data.len() == program_size,
        //     "input is expected to equal number of instuctions loaded into memory"
        // );
        // println!("Program loaded with {program_size} instructions",);
    }

    pub fn new() -> Self {
        let fonts = init_fonts();
        let memory = Self::init_memory(&fonts);

        Self {
            is_loaded: false,
            fonts,
            memory,
            display: [0; 64 * 32],
            program_counter: 0,
            index: 0,
            stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn init_memory() {
        let system = System::new();
        // first
        assert_eq!(system.memory[80], 0xF0);
        // last
        assert_eq!(system.memory[159], 0x80);
    }
}
