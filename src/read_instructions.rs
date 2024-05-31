pub fn create_nibble_pairs(rom_data: Vec<u8>) -> Vec<(u8, u8)> {
    let mut pairs: Vec<(u8, u8)> = vec![];
    let mut i = 0;
    while i < rom_data.len() {
        let instruction_nibble = rom_data.get(i).expect("Instruction nibble does not exist");
        let info_nibble = rom_data.get(i + 1).expect("Info nibble does not exist");
        pairs.push((*instruction_nibble, *info_nibble));
        i += 2;
    }
    println!("pairs {:?}", pairs);
    pairs
}

pub fn read_instruction(instruction: &(u8, u8)) {
    // is there a better way than turning my u8 into a string?
    let instruction_string = format!("{:#04x}", instruction.0);
    let info_string = format!("{:#04x}", instruction.1);
    match instruction_string.as_bytes() {
        // 0x00
        [b'0', b'x', b'0', b'0'] => {
            match info_string.as_bytes() {
                // 0xE0 clear screen
                [b'0', b'x', b'e', b'0'] => println!("clear 2"),
                _ => println!("0x00 unhandled: {info_string}"),
            }
        }

        //0x1N
        [b'0', b'x', b'1', n] => match info_string.as_bytes() {
            // 0xNN
            [b'0', b'x', n1, n2] => println!("double value {n} {n1} {n2}"),
            _ => println!("0x1N unhandled: {info_string}"),
        },

        //0x6X
        [b'0', b'x', b'6', x] => match info_string.as_bytes() {
            // 0xNN
            [b'0', b'x', n1, n2] => println!("set register {x} {n1} {n2}"),
            _ => println!("0x6X unhandled: {info_string}"),
        },

        //0x7X
        [b'0', b'x', b'7', x] => match info_string.as_bytes() {
            // 0xNN
            [b'0', b'x', n1, n2] => println!("add value to register {x} {n1} {n2}"),
            _ => println!("0x7X unhandled: {info_string}"),
        },

        //0xAN
        [b'0', b'x', b'a', n] => match info_string.as_bytes() {
            // 0xNN
            [b'0', b'x', n1, n2] => println!("add value to register {n} {n1} {n2}"),
            _ => println!("0xAN unhandled: {info_string}"),
        },

        //0xDX
        [b'0', b'x', b'd', x] => match info_string.as_bytes() {
            // 0xYN
            [b'0', b'x', y, n] => println!("add value to register {x} {y} {n}"),
            _ => println!("0xDX unhandled: {info_string}"),
        },
        _ => println!("... {instruction_string} {info_string}"),
    }

    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)
}
