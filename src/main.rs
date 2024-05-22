mod fonts;
mod system;

use system::System;
use ui::start_ui;

fn main() {
    let mut system = System::new();
    let _ = start_ui();
}
