pub fn read_instruction(instruction: &u8) {
    println!("{:#04x}", instruction);
    // is there a better way than turning my u8 into a string?
    let instruction_string = format!("{:#04x}", instruction);
    match instruction_string.as_bytes() {
        [b'0', b'x', b'0', b'0'] => println!("clear 1"),
        [b'0', b'x', b'e', b'0'] => println!("clear 2"),
        [b'0', b'x', b'1', n] => println!("jump {n}"),
        [b'0', b'x', b'6', x] => println!("set {x}"),
        [b'0', b'x', b'7', x] => println!("add value {x}"),
        [b'0', b'x', b'd', x] => println!("display {x}"),
        missing => println!("... {missing:?}"),
    }
    // 00E0 (clear screen)
    // 1NNN (jump)
    // 6XNN (set register VX)
    // 7XNN (add value to register VX)
    // ANNN (set index register I)
    // DXYN (display/draw)
}
