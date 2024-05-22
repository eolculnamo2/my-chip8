use crate::fonts::{get_fonts_by_char, init_fonts, FontArray};

type MemoryArray = [u8; 4096];

// 64 wide, 32  tall
type DisplayArray = [u8; 64 * 32];

pub struct System {
    pub fonts: FontArray,
    pub memory: MemoryArray,
    pub display: DisplayArray,
}

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

    pub fn new() -> Self {
        let fonts = init_fonts();
        let memory = Self::init_memory(&fonts);

        Self {
            fonts,
            memory,
            display: [0; 64 * 32],
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
