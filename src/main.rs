mod fonts;
mod rom;
mod system;

use rom::fetch_rom;
use system::System;
use ui::start_ui;

fn main() {
    let rom = fetch_rom().expect("Failed to fetch ROM");
    let mut system = System::new();
    system.load_program(rom);
    let _ = start_ui();
}
